/* LD_PRELOAD shim for the build-input inventory (alloc-delta/audit/inventory).
 * Logs every getenv/secure_getenv call to the file named by
 * ALLOC_INV_GETENV_LOG, one line per call:
 *   <pid>\t<exe>\t<name>\t<present 0|1>\t<stack objects, ';'-separated>
 * (The last field is empty: stack attribution with backtrace() deadlocked in
 * early process start-up under this sandbox, so a read made by a proc macro
 * inside rustc is attributed to the rustc process, not to the macro.)
 * It never changes a result. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static __thread int in_hook;
typedef char *(*getenv_t)(const char *);

static void log_call(const char *name, const char *res) {
    if (in_hook) return;
    in_hook = 1;
    static getenv_t real_getenv;
    if (!real_getenv) real_getenv = (getenv_t)dlsym(RTLD_NEXT, "getenv");
    const char *path = real_getenv ? real_getenv("ALLOC_INV_GETENV_LOG") : NULL;
    if (path) {
        char exe[1024];
        ssize_t n = readlink("/proc/self/exe", exe, sizeof exe - 1);
        exe[n > 0 ? n : 0] = 0;
        char line[8192];
        int off = snprintf(line, sizeof line, "%d\t%s\t%s\t%d\t", (int)getpid(), exe, name, res ? 1 : 0);
        if (off < (int)sizeof line - 1) line[off++] = '\n';
        int fd = open(path, O_WRONLY | O_APPEND | O_CREAT | O_CLOEXEC, 0644);
        if (fd >= 0) {
            ssize_t w = write(fd, line, off);
            (void)w;
            close(fd);
        }
    }
    in_hook = 0;
}

char *getenv(const char *name) {
    static getenv_t real;
    if (!real) real = (getenv_t)dlsym(RTLD_NEXT, "getenv");
    char *r = real(name);
    log_call(name, r);
    return r;
}

char *secure_getenv(const char *name) {
    static getenv_t real;
    if (!real) real = (getenv_t)dlsym(RTLD_NEXT, "secure_getenv");
    char *r = real ? real(name) : NULL;
    log_call(name, r);
    return r;
}
