#define _POSIX_C_SOURCE 200809L

#include <arpa/inet.h>
#include <errno.h>
#include <fcntl.h>
#include <netinet/in.h>
#include <poll.h>
#include <signal.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/time.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

static const char *const AUTH_SENTINEL = "SWALLOWTAIL_FAKE_AUTH_METADATA_SENTINEL";

static bool denied_errno(int error_number) {
    return error_number == EPERM || error_number == EACCES;
}

static bool path_for(char *output, size_t capacity, const char *base, const char *tail) {
    int written = snprintf(output, capacity, "%s/%s", base, tail);
    return written >= 0 && (size_t)written < capacity;
}

static bool read_approved_auth_metadata(const char *path) {
    char contents[1024];
    int descriptor = open(path, O_RDONLY);
    if (descriptor < 0) {
        return false;
    }
    ssize_t count = read(descriptor, contents, sizeof(contents) - 1);
    int close_result = close(descriptor);
    if (count <= 0 || close_result != 0) {
        return false;
    }
    contents[count] = '\0';
    return strstr(contents, AUTH_SENTINEL) != NULL;
}

static int run_config_path_probe(const char *mode) {
    const char *home = getenv("HOME");
    char metadata_path[4096];
    if (home == NULL || !path_for(metadata_path, sizeof(metadata_path), home, ".copilot/config.json")) {
        return 50;
    }
    if (strcmp(mode, "--config-read") == 0) {
        return read_approved_auth_metadata(metadata_path) ? 0 : 51;
    }
    if (strcmp(mode, "--config-write") == 0) {
        int descriptor = open(metadata_path, O_WRONLY);
        if (descriptor >= 0) {
            close(descriptor);
            return 52;
        }
        return denied_errno(errno) ? 0 : 53;
    }
    if (strcmp(mode, "--config-parent-metadata") == 0) {
        char parent_path[4096];
        struct stat metadata;
        if (!path_for(parent_path, sizeof(parent_path), home, ".copilot")) {
            return 54;
        }
        if (stat(parent_path, &metadata) == 0) {
            return 55;
        }
        return denied_errno(errno) ? 0 : 56;
    }
    if (strcmp(mode, "--config-adjacent-home-read") == 0) {
        char adjacent_path[4096];
        if (!path_for(adjacent_path, sizeof(adjacent_path), home, ".ssh/id_ed25519")) {
            return 62;
        }
        int descriptor = open(adjacent_path, O_RDONLY);
        if (descriptor >= 0) {
            close(descriptor);
            return 63;
        }
        return denied_errno(errno) ? 0 : 64;
    }
    if (strcmp(mode, "--config-missing") == 0) {
        int descriptor = open(metadata_path, O_RDONLY);
        if (descriptor >= 0) {
            close(descriptor);
            return 57;
        }
        return errno == ENOENT ? 0 : 58;
    }
    if (strcmp(mode, "--config-symlink") == 0 ||
        strcmp(mode, "--config-symlink-escape") == 0) {
        int descriptor = open(metadata_path, O_RDONLY);
        if (descriptor >= 0) {
            close(descriptor);
            return 59;
        }
        return denied_errno(errno) ? 0 : 60;
    }
    return 61;
}

static volatile sig_atomic_t escaped_parent_stop_requested = 0;

static void escaped_parent_stop(int signal_number) {
    (void)signal_number;
    escaped_parent_stop_requested = 1;
}

static bool release_and_wait_direct_child(int control_descriptor, pid_t child, int *status) {
    const char release = 'E';
    ssize_t written;
    do {
        written = write(control_descriptor, &release, 1);
    } while (written < 0 && errno == EINTR);
    close(control_descriptor);
    pid_t waited;
    do {
        waited = waitpid(child, status, 0);
    } while (waited < 0 && errno == EINTR);
    return written == 1 && waited == child && WIFEXITED(*status) && WEXITSTATUS(*status) == 0;
}

static int run_escaped_descendant_control(void) {
    int ready_pipe[2];
    int control_pipe[2];
    if (pipe(ready_pipe) != 0) {
        return 70;
    }
    if (pipe(control_pipe) != 0) {
        close(ready_pipe[0]);
        close(ready_pipe[1]);
        return 70;
    }
    (void)signal(SIGPIPE, SIG_IGN);
    pid_t child = fork();
    if (child < 0) {
        close(ready_pipe[0]);
        close(ready_pipe[1]);
        close(control_pipe[0]);
        close(control_pipe[1]);
        return 71;
    }
    if (child == 0) {
        close(ready_pipe[0]);
        close(control_pipe[1]);
        alarm(5);
        if (setsid() < 0) {
            _exit(72);
        }
        char ready = 'R';
        if (write(ready_pipe[1], &ready, 1) != 1) {
            _exit(73);
        }
        close(ready_pipe[1]);
        char command = 0;
        ssize_t received;
        do {
            received = read(control_pipe[0], &command, 1);
        } while (received < 0 && errno == EINTR);
        close(control_pipe[0]);
        _exit(received == 1 && command == 'E' ? 0 : 78);
    }

    close(ready_pipe[1]);
    close(control_pipe[0]);
    char ready = 0;
    ssize_t count;
    do {
        count = read(ready_pipe[0], &ready, 1);
    } while (count < 0 && errno == EINTR);
    close(ready_pipe[0]);
    bool escaped = count == 1 && ready == 'R' && getpgid(child) == child;
    if (!escaped) {
        int status = 0;
        (void)release_and_wait_direct_child(control_pipe[1], child, &status);
        return 74;
    }

    struct sigaction action = {0};
    action.sa_handler = escaped_parent_stop;
    sigemptyset(&action.sa_mask);
    if (sigaction(SIGTERM, &action, NULL) != 0 ||
        puts("FAKE_DESCENDANT_READY") < 0 || fflush(stdout) != 0) {
        int status = 0;
        (void)release_and_wait_direct_child(control_pipe[1], child, &status);
        return 75;
    }

    struct pollfd input = {.fd = STDIN_FILENO, .events = POLLIN};
    int poll_result;
    do {
        poll_result = poll(&input, 1, 2000);
    } while (poll_result < 0 && errno == EINTR && !escaped_parent_stop_requested);
    char command = 0;
    bool join_requested = poll_result > 0 && (input.revents & POLLIN) != 0 &&
        read(STDIN_FILENO, &command, 1) == 1 && command == 'J';
    int status = 0;
    bool joined = release_and_wait_direct_child(control_pipe[1], child, &status);
    if (!joined) {
        return 76;
    }
    if (join_requested && !escaped_parent_stop_requested &&
        puts("FAKE_DESCENDANT_REAPED") >= 0 && fflush(stdout) == 0) {
        return 0;
    }
    return 77;
}

static bool read_denied(const char *path) {
    int descriptor = open(path, O_RDONLY);
    if (descriptor >= 0) {
        close(descriptor);
        return false;
    }
    return denied_errno(errno);
}

static bool write_denied(const char *path, bool create) {
    int flags = O_WRONLY | (create ? O_CREAT | O_EXCL : 0);
    int descriptor = open(path, flags, 0600);
    if (descriptor >= 0) {
        close(descriptor);
        if (create) {
            unlink(path);
        }
        return false;
    }
    return denied_errno(errno);
}

static bool direct_network_denied(void) {
    int descriptor = socket(AF_INET, SOCK_STREAM, 0);
    if (descriptor < 0) {
        return denied_errno(errno);
    }
    struct sockaddr_in address = {0};
    address.sin_family = AF_INET;
    address.sin_port = htons(443);
    if (inet_pton(AF_INET, "203.0.113.1", &address.sin_addr) != 1) {
        close(descriptor);
        return false;
    }
    struct timeval timeout = {.tv_sec = 0, .tv_usec = 250000};
    setsockopt(descriptor, SOL_SOCKET, SO_SNDTIMEO, &timeout, sizeof(timeout));
    int result = connect(descriptor, (struct sockaddr *)&address, sizeof(address));
    int error_number = errno;
    close(descriptor);
    return result < 0 && denied_errno(error_number);
}

static bool inbound_bind_denied(void) {
    int descriptor = socket(AF_INET, SOCK_STREAM, 0);
    if (descriptor < 0) {
        return false;
    }
    struct sockaddr_in address = {0};
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = htonl(0x7f000001u);
    address.sin_port = 0;
    int result = bind(descriptor, (struct sockaddr *)&address, sizeof(address));
    int error_number = errno;
    close(descriptor);
    return result < 0 && denied_errno(error_number);
}

static bool shell_exec_denied(void) {
    pid_t child = fork();
    if (child < 0) {
        return false;
    }
    if (child == 0) {
        execl("/bin/sh", "sh", "-c", "exit 81", (char *)NULL);
        _exit(denied_errno(errno) ? 0 : 82);
    }
    int status = 0;
    if (waitpid(child, &status, 0) != child || !WIFEXITED(status)) {
        return false;
    }
    return WEXITSTATUS(status) == 0;
}

static bool send_all(int descriptor, const unsigned char *bytes, size_t length) {
    size_t sent = 0;
    while (sent < length) {
        ssize_t count = send(descriptor, bytes + sent, length - sent, 0);
        if (count <= 0) {
            return false;
        }
        sent += (size_t)count;
    }
    return true;
}

static void append_u16(unsigned char *output, size_t *offset, size_t value) {
    output[(*offset)++] = (unsigned char)((value >> 8) & 0xff);
    output[(*offset)++] = (unsigned char)(value & 0xff);
}

static size_t client_hello(unsigned char *output, size_t capacity, const char *server_name) {
    size_t name_length = strlen(server_name);
    size_t hello_length = 2 + 32 + 1 + 4 + 2 + 2 + 4 + 5 + name_length;
    size_t sni_extension_length = 4 + 2 + 3 + name_length;
    size_t extensions_length = 2 + sni_extension_length;
    size_t handshake_length = 4 + hello_length;
    size_t record_length = 5 + handshake_length;
    if (name_length == 0 || name_length > 240 || record_length > capacity) {
        return 0;
    }

    size_t offset = 0;
    output[offset++] = 0x16;
    output[offset++] = 0x03;
    output[offset++] = 0x01;
    append_u16(output, &offset, handshake_length);
    output[offset++] = 0x01;
    output[offset++] = (unsigned char)((hello_length >> 16) & 0xff);
    output[offset++] = (unsigned char)((hello_length >> 8) & 0xff);
    output[offset++] = (unsigned char)(hello_length & 0xff);
    output[offset++] = 0x03;
    output[offset++] = 0x03;
    memset(output + offset, 'R', 32);
    offset += 32;
    output[offset++] = 0x00;
    append_u16(output, &offset, 2);
    output[offset++] = 0x13;
    output[offset++] = 0x01;
    output[offset++] = 0x01;
    output[offset++] = 0x00;
    append_u16(output, &offset, extensions_length);
    append_u16(output, &offset, 0x0000);
    append_u16(output, &offset, 2 + 3 + name_length);
    append_u16(output, &offset, 3 + name_length);
    output[offset++] = 0x00;
    append_u16(output, &offset, name_length);
    memcpy(output + offset, server_name, name_length);
    offset += name_length;
    return offset == record_length ? offset : 0;
}

static int proxy_status(int port, const char *host, const char *server_name) {
    int descriptor = socket(AF_INET, SOCK_STREAM, 0);
    if (descriptor < 0) {
        return -1;
    }
    struct timeval timeout = {.tv_sec = 1, .tv_usec = 0};
    setsockopt(descriptor, SOL_SOCKET, SO_RCVTIMEO, &timeout, sizeof(timeout));
    setsockopt(descriptor, SOL_SOCKET, SO_SNDTIMEO, &timeout, sizeof(timeout));
    struct sockaddr_in address = {0};
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = htonl(0x7f000001u);
    address.sin_port = htons((uint16_t)port);
    if (connect(descriptor, (struct sockaddr *)&address, sizeof(address)) != 0) {
        close(descriptor);
        return -1;
    }

    char request[512];
    int request_length = snprintf(
        request,
        sizeof(request),
        "CONNECT %s:443 HTTP/1.1\r\nHost: %s:443\r\n\r\n",
        host,
        host);
    if (request_length < 0 || (size_t)request_length >= sizeof(request) ||
        !send_all(descriptor, (const unsigned char *)request, (size_t)request_length)) {
        close(descriptor);
        return -1;
    }

    char response[1024] = {0};
    size_t received = 0;
    while (received + 1 < sizeof(response) && strstr(response, "\r\n\r\n") == NULL) {
        ssize_t count = recv(descriptor, response + received, sizeof(response) - received - 1, 0);
        if (count <= 0) {
            close(descriptor);
            return -1;
        }
        received += (size_t)count;
        response[received] = '\0';
    }

    int status = -1;
    if (strncmp(response, "HTTP/1.1 200", 12) == 0) {
        status = 200;
    } else if (strncmp(response, "HTTP/1.1 403", 12) == 0) {
        status = 403;
    }
    if (status == 200 && server_name != NULL) {
        unsigned char hello[512];
        size_t hello_length = client_hello(hello, sizeof(hello), server_name);
        if (hello_length == 0 || !send_all(descriptor, hello, hello_length)) {
            close(descriptor);
            return -1;
        }
    }
    close(descriptor);
    return status;
}

static bool write_diagnostic(const char *stage, const bool values[13]) {
    const char *path = getenv("SWALLOWTAIL_FAKE_DIAGNOSTIC");
    if (path == NULL) {
        return false;
    }
    FILE *output = fopen(path, "w");
    if (output == NULL) {
        return false;
    }
    int result = fprintf(
        output,
        "{\"stage\":\"%s\",\"native_start_observed\":true,"
        "\"auth_metadata_read\":%s,\"auth_metadata_write_denied\":%s,"
        "\"adjacent_home_reads_denied\":%s,\"keychain_file_read_denied\":%s,"
        "\"repository_read_denied\":%s,\"repository_write_denied\":%s,"
        "\"direct_network_denied\":%s,\"inbound_bind_denied\":%s,"
        "\"shell_exec_denied\":%s,\"proxy_allowlist_exercised\":%s,"
        "\"proxy_sni_mismatch_exercised\":%s,\"proxy_unlisted_denied\":%s,"
        "\"effect_marker_absent\":%s}",
        stage,
        values[0] ? "true" : "false",
        values[1] ? "true" : "false",
        values[2] ? "true" : "false",
        values[3] ? "true" : "false",
        values[4] ? "true" : "false",
        values[5] ? "true" : "false",
        values[6] ? "true" : "false",
        values[7] ? "true" : "false",
        values[8] ? "true" : "false",
        values[9] ? "true" : "false",
        values[10] ? "true" : "false",
        values[11] ? "true" : "false",
        values[12] ? "true" : "false");
    int close_result = fclose(output);
    return result > 0 && close_result == 0;
}

static bool run_policy_probes(void) {
    const char *home = getenv("HOME");
    const char *repository = getenv("SWALLOWTAIL_BLOCKED_REPOSITORY");
    const char *marker = getenv("SWALLOWTAIL_EFFECT_MARKER");
    const char *proxy_port_text = getenv("SWALLOWTAIL_PROXY_PORT");
    char auth_path[4096], settings_path[4096], adjacent_path[4096];
    char keychain_path[4096], repository_file[4096], repository_write[4096];
    char marker_path[4096];
    if (home == NULL || repository == NULL || marker == NULL || proxy_port_text == NULL ||
        !path_for(auth_path, sizeof(auth_path), home, ".copilot/config.json") ||
        !path_for(settings_path, sizeof(settings_path), home, ".copilot/settings.json") ||
        !path_for(adjacent_path, sizeof(adjacent_path), home, ".ssh/id_ed25519") ||
        !path_for(keychain_path, sizeof(keychain_path), home, "Library/Keychains/login.keychain-db") ||
        !path_for(repository_file, sizeof(repository_file), repository, "README.md") ||
        !path_for(repository_write, sizeof(repository_write), repository, ".swallowtail-write-probe")) {
        return false;
    }
    int marker_length = snprintf(marker_path, sizeof(marker_path), "%s", marker);
    if (marker_length < 0 || (size_t)marker_length >= sizeof(marker_path)) {
        return false;
    }

    char *port_end = NULL;
    long proxy_port_long = strtol(proxy_port_text, &port_end, 10);
    if (port_end == proxy_port_text || *port_end != '\0' || proxy_port_long <= 0 || proxy_port_long > 65535) {
        return false;
    }
    int proxy_port = (int)proxy_port_long;
    bool values[13] = {0};
    static const char *const probe_stages[13] = {
        "auth-metadata-read",
        "auth-metadata-write-denied",
        "adjacent-home-reads-denied",
        "keychain-file-read-denied",
        "repository-read-denied",
        "repository-write-denied",
        "direct-egress-denied",
        "inbound-bind-denied",
        "shell-exec-denied",
        "proxy-allowlist",
        "proxy-sni-mismatch",
        "proxy-unlisted-denied",
        "effect-marker-absent",
    };
    if (!write_diagnostic("policy-probes-started", values)) {
        return false;
    }
    values[0] = read_approved_auth_metadata(auth_path);
    if (!write_diagnostic(probe_stages[0], values)) return false;
    values[1] = write_denied(auth_path, false) && write_denied(settings_path, false);
    if (!write_diagnostic(probe_stages[1], values)) return false;
    values[2] = read_denied(settings_path) && read_denied(adjacent_path);
    if (!write_diagnostic(probe_stages[2], values)) return false;
    values[3] = read_denied(keychain_path);
    if (!write_diagnostic(probe_stages[3], values)) return false;
    values[4] = read_denied(repository_file);
    if (!write_diagnostic(probe_stages[4], values)) return false;
    values[5] = write_denied(repository_write, true);
    if (!write_diagnostic(probe_stages[5], values)) return false;
    values[6] = direct_network_denied();
    if (!write_diagnostic(probe_stages[6], values)) return false;
    values[7] = inbound_bind_denied();
    if (!write_diagnostic(probe_stages[7], values)) return false;
    values[8] = shell_exec_denied();
    if (!write_diagnostic(probe_stages[8], values)) return false;
    values[9] = proxy_status(proxy_port, "api.github.com", "api.github.com") == 200;
    if (!write_diagnostic(probe_stages[9], values)) return false;
    values[10] = proxy_status(proxy_port, "api.github.com", "unlisted.example") == 200;
    if (!write_diagnostic(probe_stages[10], values)) return false;
    values[11] = proxy_status(proxy_port, "unlisted.example", NULL) == 403;
    if (!write_diagnostic(probe_stages[11], values)) return false;
    values[12] = access(marker_path, F_OK) != 0 && (errno == ENOENT || errno == EPERM || errno == EACCES);
    if (!write_diagnostic(probe_stages[12], values)) return false;

    bool passed = true;
    for (size_t index = 0; index < sizeof(values) / sizeof(values[0]); index++) {
        passed = passed && values[index];
    }
    if (!write_diagnostic(passed ? "policy-probes-passed" : "policy-probes-failed", values)) {
        return false;
    }
    return passed;
}

static bool send_line(const char *line) {
    return puts(line) >= 0 && fflush(stdout) == 0;
}

static bool read_request(char *line, size_t capacity, const char *method) {
    if (fgets(line, (int)capacity, stdin) == NULL) {
        return false;
    }
    return strchr(line, '\n') != NULL && strstr(line, method) != NULL;
}

static bool send_line(const char *line);

static int run_permission_agent(int argument_count, char **arguments) {
    if (argument_count != 5 || strcmp(arguments[1], "--model") != 0 ||
        strcmp(arguments[2], "auto") != 0 || strcmp(arguments[3], "--acp") != 0 ||
        strcmp(arguments[4], "--stdio") != 0) {
        return 101;
    }
    if (getenv("COPILOT_GITHUB_TOKEN") != NULL ||
        getenv("GH_TOKEN") != NULL || getenv("GITHUB_TOKEN") != NULL ||
        getenv("COPILOT_HOME") != NULL || getenv("COPILOT_MODEL") != NULL) {
        return 102;
    }
    if (!send_line(
            "{\"jsonrpc\":\"2.0\",\"method\":\"swallowtail/native-fake-started\"}")) {
        return 104;
    }
    if (!run_policy_probes()) {
        fputs("fake policy probe failed\n", stderr);
        return 103;
    }

    char line[65536];
    if (!read_request(line, sizeof(line), "\"method\":\"initialize\"")) {
        return 110;
    }
    if (!send_line(
            "{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"protocolVersion\":1,"
            "\"agentCapabilities\":{},\"authMethods\":[{\"id\":\"copilot-login\","
            "\"name\":\"Existing host login\"}],\"agentInfo\":{\"name\":\"fake-copilot-native\","
            "\"version\":\"1.0.93\"}}}")) {
        return 111;
    }
    if (!read_request(line, sizeof(line), "\"method\":\"session/new\"")) {
        return 112;
    }
    if (!send_line(
            "{\"jsonrpc\":\"2.0\",\"id\":2,\"result\":{\"sessionId\":\"fake-session\","
            "\"models\":{\"currentModelId\":\"fake-underlying-model\"}}}")) {
        return 113;
    }
    if (!read_request(line, sizeof(line), "\"method\":\"session/prompt\"")) {
        return 114;
    }
    const char *marker = getenv("SWALLOWTAIL_EFFECT_MARKER");
    if (marker == NULL || access(marker, F_OK) == 0) {
        return 115;
    }
    if (!send_line(
            "{\"jsonrpc\":\"2.0\",\"id\":99,\"method\":\"session/request_permission\","
            "\"params\":{\"sessionId\":\"fake-session\",\"toolCall\":{\"toolCallId\":\"fake-tool\","
            "\"status\":\"pending\"},\"options\":[{\"optionId\":\"allow_once\","
            "\"name\":\"Allow once\",\"kind\":\"allow_once\"},{\"optionId\":\"reject_once\","
            "\"name\":\"Reject once\",\"kind\":\"reject_once\"}]}}")) {
        return 116;
    }
    if (!fgets(line, sizeof(line), stdin) || strstr(line, "\"id\":99") == NULL ||
        strstr(line, "\"outcome\":\"cancelled\"") == NULL) {
        return 117;
    }
    if (access(marker, F_OK) == 0) {
        return 118;
    }
    if (!read_request(line, sizeof(line), "\"method\":\"session/cancel\"")) {
        return 119;
    }
    if (!send_line("{\"jsonrpc\":\"2.0\",\"id\":3,\"result\":{\"stopReason\":\"cancelled\"}}")) {
        return 120;
    }
    signal(SIGTERM, SIG_IGN);
    for (;;) {
        pause();
    }
}

static int run_diagnostic_mode(const char *mode) {
    if (strncmp(mode, "--config-", 9) == 0) {
        printf("FAKE_NATIVE_STARTED:%s\n", mode);
        fflush(stdout);
        return run_config_path_probe(mode);
    }
    if (strcmp(mode, "--escaped-descendant-control") == 0) {
        puts("FAKE_NATIVE_STARTED:--escaped-descendant-control");
        fflush(stdout);
        return run_escaped_descendant_control();
    }
    if (strcmp(mode, "--stderr-fullpipe") == 0) {
        char chunk[4096];
        puts("FAKE_NATIVE_STARTED:--stderr-fullpipe");
        fflush(stdout);
        memset(chunk, 'x', sizeof(chunk));
        for (int index = 0; index < 256; index++) {
            if (fwrite(chunk, 1, sizeof(chunk), stderr) != sizeof(chunk)) {
                return 31;
            }
        }
        return fflush(stderr) == 0 ? 0 : 32;
    }
    if (strcmp(mode, "--stderr-secret") == 0) {
        const char *home = getenv("HOME");
        puts("FAKE_NATIVE_STARTED:--stderr-secret");
        fflush(stdout);
        fprintf(
            stderr,
            "token=SWALLOWTAIL_SENTINEL_TOKEN cookie=SWALLOWTAIL_SENTINEL_COOKIE "
            "auth_url=https://github.example/login?state=SWALLOWTAIL_SENTINEL_URL "
            "path=%s prompt=session-private arbitrary vendor error text\n",
            home == NULL ? "synthetic-home" : home);
        fputc(0xff, stderr);
        return 37;
    }
    if (strcmp(mode, "--stderr-sandbox-vendor-text") == 0) {
        puts("FAKE_NATIVE_STARTED:--stderr-sandbox-vendor-text");
        fflush(stdout);
        fputs("vendor note: sandbox mode is available; startup continues\n", stderr);
        return fflush(stderr) == 0 ? 0 : 38;
    }
    if (strcmp(mode, "--stderr-conflicting-markers") == 0) {
        puts("FAKE_NATIVE_STARTED:--stderr-conflicting-markers");
        fflush(stdout);
        fputs(
            "startup continued; sandbox marker; permission denied; dyld: Library not loaded; "
            "exec format error\n",
            stderr);
        return fflush(stderr) == 0 ? 0 : 39;
    }
    if (strcmp(mode, "--stderr-loader-marker") == 0) {
        puts("FAKE_NATIVE_STARTED:--stderr-loader-marker");
        fflush(stdout);
        fputs("dyld: Library not loaded: synthetic runtime role\n", stderr);
        return fflush(stderr) == 0 ? 0 : 40;
    }
    if (strcmp(mode, "--early-exit") == 0) {
        puts("FAKE_NATIVE_STARTED:--early-exit");
        fflush(stdout);
        return 41;
    }
    if (strcmp(mode, "--hang") == 0) {
        puts("FAKE_NATIVE_STARTED:--hang");
        fflush(stdout);
        signal(SIGTERM, SIG_IGN);
        fputs("synthetic hang\n", stderr);
        fflush(stderr);
        for (;;) {
            pause();
        }
    }
    return -1;
}

int main(int argument_count, char **arguments) {
    bool startup_values[13] = {0};
    write_diagnostic("native-main-started", startup_values);
    if (argument_count == 2) {
        int result = run_diagnostic_mode(arguments[1]);
        if (result >= 0) {
            return result;
        }
    }
    return run_permission_agent(argument_count, arguments);
}
