#define _POSIX_C_SOURCE 200809L

#include <errno.h>
#include <fcntl.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static char errno_code(int error_number) {
    switch (error_number) {
    case EPERM:
        return 'P';
    case EACCES:
        return 'A';
    case ENOENT:
        return 'N';
    case ENOEXEC:
        return 'F';
    case EINVAL:
        return 'I';
    case EIO:
        return 'O';
#ifdef ETIMEDOUT
    case ETIMEDOUT:
        return 'T';
#endif
    default:
        return 'U';
    }
}

static bool write_byte(int descriptor, char value) {
    for (;;) {
        ssize_t written = write(descriptor, &value, 1);
        if (written == 1) {
            return true;
        }
        if (written < 0 && errno == EINTR) {
            continue;
        }
        return false;
    }
}

int main(int argument_count, char **arguments) {
    if (argument_count < 7 || strcmp(arguments[1], "--stage-fd") != 0 ||
        strcmp(arguments[3], "--target") != 0 || strcmp(arguments[5], "--") != 0) {
        return 125;
    }

    char *end = NULL;
    long descriptor_long = strtol(arguments[2], &end, 10);
    if (end == arguments[2] || *end != '\0' || descriptor_long < 0 ||
        descriptor_long > 1048576) {
        return 125;
    }
    int stage_descriptor = (int)descriptor_long;
    const char *target = arguments[4];
    if (target[0] != '/' || arguments[6][0] != '/' || strcmp(target, arguments[6]) != 0) {
        return 125;
    }
    if (!write_byte(stage_descriptor, 'L')) {
        return 126;
    }

    int flags = fcntl(stage_descriptor, F_GETFD);
    if (flags < 0 || fcntl(stage_descriptor, F_SETFD, flags | FD_CLOEXEC) < 0) {
        return 126;
    }
    execv(target, &arguments[6]);
    int error_number = errno;
    (void)write_byte(stage_descriptor, 'E');
    (void)write_byte(stage_descriptor, errno_code(error_number));
    close(stage_descriptor);
    return 127;
}
