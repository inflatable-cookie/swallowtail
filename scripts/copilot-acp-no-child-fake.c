#define _POSIX_C_SOURCE 200809L

#include <arpa/inet.h>
#include <errno.h>
#include <fcntl.h>
#include <mach/mach.h>
#include <netinet/in.h>
#include <spawn.h>
#include <stdbool.h>
#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>
#include <servers/bootstrap.h>

extern char **environ;
static volatile sig_atomic_t cancellation_seen = 0;

static void note_cancellation(int signal_number) {
    (void)signal_number;
    cancellation_seen = 1;
}

static bool denied_errno(int error_number) {
    return error_number == EPERM || error_number == EACCES;
}

static bool denied_child_creation(const char *case_name, int error_number) {
    if (denied_errno(error_number)) {
        return true;
    }
    printf("CHILD_CASE:%s:unexpected-errno:%d\n", case_name, error_number);
    return false;
}

static bool path_for(char *output, size_t capacity, const char *base, const char *tail) {
    int written = snprintf(output, capacity, "%s/%s", base, tail);
    return written >= 0 && (size_t)written < capacity;
}

static bool mark_child_effect(const char *case_name) {
    const char *action = getenv("SWALLOWTAIL_ACTION_DIR");
    char path[4096];
    if (action == NULL || !path_for(path, sizeof(path), action, case_name)) {
        return false;
    }
    int descriptor = open(path, O_WRONLY | O_CREAT | O_EXCL, 0600);
    if (descriptor < 0) {
        return false;
    }
    const char effect[] = "child-effect";
    ssize_t written = write(descriptor, effect, sizeof(effect) - 1);
    int close_result = close(descriptor);
    return written == (ssize_t)(sizeof(effect) - 1) && close_result == 0;
}

static bool sentinel_read_denied(const char *path) {
    int descriptor = open(path, O_RDONLY);
    if (descriptor >= 0) {
        close(descriptor);
        return false;
    }
    return denied_errno(errno);
}

static bool sentinel_write_denied(const char *path) {
    int descriptor = open(path, O_WRONLY);
    if (descriptor >= 0) {
        close(descriptor);
        return false;
    }
    return denied_errno(errno);
}

static bool sentinel_create_denied(const char *path) {
    int descriptor = open(path, O_WRONLY | O_CREAT | O_EXCL, 0600);
    if (descriptor >= 0) {
        close(descriptor);
        unlink(path);
        return false;
    }
    return denied_errno(errno);
}

static bool file_policy_probes(void) {
    const char *blocked_home = getenv("SWALLOWTAIL_BLOCKED_HOME");
    const char *blocked_repository = getenv("SWALLOWTAIL_BLOCKED_REPOSITORY");
    char auth_config[4096];
    char keychain[4096];
    char repository_sentinel[4096];
    char home_write[4096];
    char repository_write[4096];
    if (blocked_home == NULL || blocked_repository == NULL ||
        !path_for(auth_config, sizeof(auth_config), blocked_home, ".copilot/config.json") ||
        !path_for(keychain, sizeof(keychain), blocked_home, "Library/Keychains/fake.keychain") ||
        !path_for(repository_sentinel, sizeof(repository_sentinel), blocked_repository, "README.md") ||
        !path_for(home_write, sizeof(home_write), blocked_home, ".copilot/write-probe") ||
        !path_for(repository_write, sizeof(repository_write), blocked_repository, "write-probe")) {
        return false;
    }
    return sentinel_read_denied(auth_config) && sentinel_read_denied(keychain) &&
        sentinel_read_denied(repository_sentinel) && sentinel_write_denied(auth_config) &&
        sentinel_create_denied(home_write) && sentinel_create_denied(repository_write);
}

static bool network_policy_probes(void) {
    int descriptor = socket(AF_INET, SOCK_STREAM, 0);
    if (descriptor < 0) {
        return denied_errno(errno);
    }
    struct sockaddr_in remote = {0};
    remote.sin_family = AF_INET;
    remote.sin_port = htons(443);
    if (inet_pton(AF_INET, "203.0.113.1", &remote.sin_addr) != 1) {
        close(descriptor);
        return false;
    }
    int result = connect(descriptor, (struct sockaddr *)&remote, sizeof(remote));
    int error_number = errno;
    close(descriptor);
    if (result == 0 || !denied_errno(error_number)) {
        return false;
    }

    descriptor = socket(AF_INET, SOCK_STREAM, 0);
    if (descriptor < 0) {
        return denied_errno(errno);
    }
    struct sockaddr_in local = {0};
    local.sin_family = AF_INET;
    local.sin_addr.s_addr = htonl(0x7f000001u);
    local.sin_port = 0;
    result = bind(descriptor, (struct sockaddr *)&local, sizeof(local));
    error_number = errno;
    close(descriptor);
    return result < 0 && denied_errno(error_number);
}

static bool auth_service_denied(const char *service_name) {
    mach_port_t service = MACH_PORT_NULL;
    kern_return_t result = bootstrap_look_up(bootstrap_port, service_name, &service);
    if (service != MACH_PORT_NULL) {
        mach_port_deallocate(mach_task_self(), service);
    }
    return result == BOOTSTRAP_NOT_PRIVILEGED || result == KERN_NO_ACCESS ||
        result == KERN_PROTECTION_FAILURE;
}

static bool service_policy_probes(void) {
    return auth_service_denied("com.apple.securityd") &&
        auth_service_denied("com.apple.SecurityServer");
}

static bool spawn_self(char *self, const char *case_name) {
    char *arguments[] = {self, "--child-effect", (char *)case_name, NULL};
    pid_t child = -1;
    int result = posix_spawn(&child, self, NULL, NULL, arguments, environ);
    if (result != 0) {
        return denied_child_creation(case_name, result);
    }
    int status = 0;
    if (waitpid(child, &status, 0) != child) {
        return false;
    }
    return false;
}

static bool spawn_helper(const char *case_name) {
    const char *action = getenv("SWALLOWTAIL_ACTION_DIR");
    char marker[4096];
    if (action == NULL || !path_for(marker, sizeof(marker), action, case_name)) {
        return false;
    }
    char *arguments[] = {"/usr/bin/touch", marker, NULL};
    pid_t child = -1;
    int result = posix_spawn(&child, "/usr/bin/touch", NULL, NULL, arguments, environ);
    if (result != 0) {
        return denied_child_creation(case_name, result);
    }
    int status = 0;
    if (waitpid(child, &status, 0) != child) {
        return false;
    }
    return false;
}

static bool spawn_stage_launcher(void) {
    const char *launcher = getenv("SWALLOWTAIL_DIAGNOSTIC_SHIM");
    if (launcher == NULL) {
        return false;
    }
    char *arguments[] = {(char *)launcher, "--deliberately-invalid", NULL};
    pid_t child = -1;
    int result = posix_spawn(&child, launcher, NULL, NULL, arguments, environ);
    if (result != 0) {
        return denied_child_creation("spawn-stage-launcher", result);
    }
    int status = 0;
    if (waitpid(child, &status, 0) != child) {
        return false;
    }
    return false;
}

static bool fork_case(
    char *self,
    const char *case_name,
    bool escape_session,
    bool run_system_helper
) {
    const char *action = getenv("SWALLOWTAIL_ACTION_DIR");
    char helper_marker[4096];
    if (action == NULL || !path_for(helper_marker, sizeof(helper_marker), action, case_name)) {
        return false;
    }
    pid_t child = fork();
    if (child < 0) {
        return denied_child_creation(case_name, errno);
    }
    if (child == 0) {
        (void)mark_child_effect(case_name);
        if (escape_session) {
            (void)setsid();
        }
        if (run_system_helper) {
            char *helper_arguments[] = {"/usr/bin/touch", helper_marker, NULL};
            execv("/usr/bin/touch", helper_arguments);
            _exit(122);
        }
        char *arguments[] = {self, "--child-effect", (char *)case_name, NULL};
        execv(self, arguments);
        _exit(120);
    }
    int status = 0;
    if (waitpid(child, &status, 0) != child) {
        return false;
    }
    return false;
}

static bool vfork_case(char *self, const char *case_name, bool run_system_helper) {
    const char *action = getenv("SWALLOWTAIL_ACTION_DIR");
    char marker[4096];
    if (action == NULL || !path_for(marker, sizeof(marker), action, case_name)) {
        return false;
    }
    pid_t child = vfork();
    if (child < 0) {
        return denied_child_creation(case_name, errno);
    }
    if (child == 0) {
        int descriptor = open(marker, O_WRONLY | O_CREAT | O_EXCL, 0600);
        if (descriptor >= 0) {
            const char effect[] = "child-effect";
            (void)write(descriptor, effect, sizeof(effect) - 1);
            (void)close(descriptor);
        }
        if (run_system_helper) {
            char *helper_arguments[] = {"/usr/bin/touch", marker, NULL};
            execv("/usr/bin/touch", helper_arguments);
            _exit(123);
        }
        char *arguments[] = {self, "--child-effect", (char *)case_name, NULL};
        execv(self, arguments);
        _exit(121);
    }
    int status = 0;
    if (waitpid(child, &status, 0) != child) {
        return false;
    }
    return false;
}

static bool child_creation_probes(char *self) {
    const char *launcher = getenv("SWALLOWTAIL_DIAGNOSTIC_SHIM");
    if (launcher == NULL) {
        return false;
    }
    const bool results[] = {
        fork_case(self, "fork-self-exec", false, false),
        fork_case(self, "fork-session-escape", true, false),
        fork_case(self, "fork-system-helper", false, true),
        vfork_case(self, "vfork-self-exec", false),
        vfork_case(self, "vfork-system-helper", true),
        spawn_self(self, "spawn-self-exec"),
        spawn_helper("spawn-system-helper"),
        spawn_stage_launcher(),
    };
    static const char *const names[] = {
        "fork-self-exec",
        "fork-session-escape",
        "fork-system-helper",
        "vfork-self-exec",
        "vfork-system-helper",
        "spawn-self-exec",
        "spawn-system-helper",
        "spawn-stage-launcher",
    };
    bool passed = true;
    for (size_t index = 0; index < sizeof(results) / sizeof(results[0]); index++) {
        printf("CHILD_CASE:%s:%s\n", names[index], results[index] ? "denied" : "created-or-unknown");
        passed = passed && results[index];
    }
    return passed;
}

static bool write_atexit_result(bool denied) {
    const char *action = getenv("SWALLOWTAIL_ACTION_DIR");
    char path[4096];
    if (action == NULL || !path_for(path, sizeof(path), action, "atexit-result")) {
        return false;
    }
    int descriptor = open(path, O_WRONLY | O_CREAT | O_EXCL, 0600);
    if (descriptor < 0) {
        return false;
    }
    char value = denied ? 'D' : 'C';
    ssize_t written = write(descriptor, &value, 1);
    int close_result = close(descriptor);
    return written == 1 && close_result == 0;
}

static void child_at_exit_probe(void) {
    bool denied = spawn_helper("atexit-child-effect");
    (void)write_atexit_result(denied);
}

static bool write_replacement_pid(void) {
    const char *action = getenv("SWALLOWTAIL_ACTION_DIR");
    char path[4096];
    if (action == NULL || !path_for(path, sizeof(path), action, "exec-replacement-pid")) {
        return false;
    }
    int descriptor = open(path, O_WRONLY | O_CREAT | O_EXCL, 0600);
    if (descriptor < 0) {
        return false;
    }
    char value[64];
    int length = snprintf(value, sizeof(value), "%ld", (long)getpid());
    ssize_t written = length > 0 ? write(descriptor, value, (size_t)length) : -1;
    int close_result = close(descriptor);
    return written == length && close_result == 0;
}

static int initialization_mode(char *self) {
    char request[8192];
    if (fgets(request, sizeof(request), stdin) == NULL || strchr(request, '\n') == NULL ||
        strstr(request, "\"method\":\"initialize\"") == NULL) {
        return 20;
    }
    if (puts("{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"protocolVersion\":1,\"agentCapabilities\":{},\"agentInfo\":{\"name\":\"fake-copilot-initialize\",\"version\":\"1.0.93\"}}}") < 0 || fflush(stdout) != 0) {
        return 21;
    }
    bool file_denied = file_policy_probes();
    puts(file_denied ? "POLICY:file-denial:passed" : "POLICY:file-denial:failed");
    bool network_denied = network_policy_probes();
    puts(network_denied ? "POLICY:network-denial:passed" : "POLICY:network-denial:failed");
    bool auth_services_denied = service_policy_probes();
    puts(auth_services_denied ? "POLICY:auth-service-denial:passed" : "POLICY:auth-service-denial:failed");
    bool child_creation_denied = child_creation_probes(self);
    puts(child_creation_denied ? "POLICY:child-creation-denial:passed" : "POLICY:child-creation-denial:failed");
    return file_denied && network_denied && auth_services_denied && child_creation_denied ? 0 : 34;
}

static int run_acp(char *self, int argument_count, char **arguments) {
    if (argument_count != 3 || strcmp(arguments[1], "--acp") != 0 ||
        strcmp(arguments[2], "--stdio") != 0) {
        return 10;
    }
    if (getenv("GITHUB_TOKEN") != NULL || getenv("GH_TOKEN") != NULL ||
        getenv("COPILOT_GITHUB_TOKEN") != NULL || getenv("COPILOT_MODEL") != NULL) {
        return 11;
    }
    printf("FAKE_ROOT_PID:%ld\n", (long)getpid());
    fflush(stdout);
    if (atexit(child_at_exit_probe) != 0) {
        return 12;
    }
    return initialization_mode(self);
}

static int child_effect_mode(const char *case_name) {
    return mark_child_effect(case_name) ? 0 : 40;
}

int main(int argument_count, char **arguments) {
    if (argument_count == 3 && strcmp(arguments[1], "--child-effect") == 0) {
        return child_effect_mode(arguments[2]);
    }
    if (argument_count == 2 && strcmp(arguments[1], "--replacement-finished") == 0) {
        bool written = write_replacement_pid();
        puts(written ? "EXEC_REPLACEMENT:same-root" : "EXEC_REPLACEMENT:failed");
        return written ? 0 : 41;
    }
    if (argument_count == 2 && strcmp(arguments[1], "--exec-replacement") == 0) {
        char *replacement_arguments[] = {arguments[0], "--replacement-finished", NULL};
        execv(arguments[0], replacement_arguments);
        return denied_errno(errno) ? 42 : 43;
    }
    if (argument_count == 2 && strcmp(arguments[1], "--system-helper-replacement") == 0) {
        const char *action = getenv("SWALLOWTAIL_ACTION_DIR");
        char marker[4096];
        if (action == NULL || !path_for(marker, sizeof(marker), action, "system-helper-effect")) {
            return 44;
        }
        char *helper_arguments[] = {"/usr/bin/touch", marker, NULL};
        execv("/usr/bin/touch", helper_arguments);
        return denied_errno(errno) ? 0 : 45;
    }
    if (argument_count == 2 && strcmp(arguments[1], "--fail-before-start") == 0) {
        return 46;
    }
    if (argument_count == 2 && strcmp(arguments[1], "--hang-after-start") == 0) {
        signal(SIGTERM, SIG_IGN);
        puts("FAKE_ROOT_STARTED");
        fflush(stdout);
        for (;;) {
            pause();
        }
    }
    if (argument_count == 2 && strcmp(arguments[1], "--cancel-ready") == 0) {
        if (signal(SIGTERM, note_cancellation) == SIG_ERR) {
            return 47;
        }
        puts("FAKE_CANCEL_READY");
        fflush(stdout);
        while (!cancellation_seen) {
            pause();
        }
        puts("FAKE_CANCELLED");
        return 0;
    }
    if (argument_count == 2 && strcmp(arguments[1], "--diagnostic-secret") == 0) {
        const char *home = getenv("HOME");
        puts("FAKE_DIAGNOSTIC_STARTED");
        fflush(stdout);
        fprintf(
            stderr,
            "sandbox access denied token=SWALLOWTAIL_DIAGNOSTIC_SECRET_SENTINEL "
            "path=%s/private/config hash=SWALLOWTAIL_RAW_HASH_SENTINEL arbitrary text\n",
            home == NULL ? "synthetic-home" : home);
        return 48;
    }
    return run_acp(arguments[0], argument_count, arguments);
}
