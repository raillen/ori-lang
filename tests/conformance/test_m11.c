#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

#ifdef _WIN32
#include <direct.h>
#define ZT_MKDIR(path) _mkdir(path)
#else
#include <sys/wait.h>
#define ZT_MKDIR(path) mkdir((path), 0755)
#endif

#include "sandbox_utils.h"

static int tests_run = 0;
static int tests_passed = 0;

#define DRIVER "compiler\\driver\\zt-next.exe"
#define OUT_DIR ".ztc-tmp/tests/conformance-m11"

#define ASSERT_TRUE(condition, msg) do { \
    tests_run++; \
    if (condition) { tests_passed++; } \
    else { fprintf(stderr, "FAIL: %s\n", msg); } \
} while (0)

static int decode_exit_code(int status) {
#ifdef _WIN32
    return status;
#else
    if (WIFEXITED(status)) return WEXITSTATUS(status);
    return status;
#endif
}

static int path_is_dir(const char *path) {
    struct stat info;
    if (stat(path, &info) != 0) return 0;
#ifdef _WIN32
    return (info.st_mode & S_IFDIR) != 0;
#else
    return S_ISDIR(info.st_mode);
#endif
}

static int ensure_dir(const char *path) {
    char buffer[512];
    char *cursor;
    size_t length = strlen(path);

    if (length + 1 > sizeof(buffer)) return 0;
    memcpy(buffer, path, length + 1);

    for (cursor = buffer; *cursor != '\0'; cursor += 1) {
        if (*cursor == '/' || *cursor == '\\') {
            char saved = *cursor;
            if (cursor == buffer) continue;
            *cursor = '\0';
            if (buffer[0] != '\0' && !path_is_dir(buffer)) {
                if (ZT_MKDIR(buffer) != 0 && !path_is_dir(buffer)) {
                    *cursor = saved;
                    return 0;
                }
            }
            *cursor = saved;
        }
    }

    if (!path_is_dir(buffer)) {
        if (ZT_MKDIR(buffer) != 0 && !path_is_dir(buffer)) return 0;
    }

    return 1;
}

static char *read_file(const char *path) {
    FILE *file;
    long file_size;
    char *buffer;

    file = fopen(path, "rb");
    if (file == NULL) return NULL;

    fseek(file, 0, SEEK_END);
    file_size = ftell(file);
    fseek(file, 0, SEEK_SET);

    buffer = (char *)malloc((size_t)file_size + 1);
    if (buffer == NULL) {
        fclose(file);
        return NULL;
    }

    if (file_size > 0 && fread(buffer, 1, (size_t)file_size, file) != (size_t)file_size) {
        free(buffer);
        fclose(file);
        return NULL;
    }

    buffer[file_size] = '\0';
    fclose(file);
    return buffer;
}

static int file_contains(const char *path, const char *fragment) {
    char *text = read_file(path);
    int ok = text != NULL && strstr(text, fragment) != NULL;
    free(text);
    return ok;
}

static int run_command(const char *command) {
    return decode_exit_code(system(command));
}

static void test_behavior_project(const char *project_dir, const char *exe_path, int expected_code, const char *name) {
    char temp_project_dir[1024];
    char temp_exe_path[1024];
    char build_cmd[2048];
    char run_cmd[2048];
    int code;

    if (!zt_prepare_sandbox_project(
            project_dir,
            name,
            exe_path,
            OUT_DIR,
            ensure_dir,
            temp_project_dir,
            sizeof(temp_project_dir),
            temp_exe_path,
            sizeof(temp_exe_path))) {
        ASSERT_TRUE(0, name);
        return;
    }

    snprintf(build_cmd, sizeof(build_cmd), DRIVER " build \"%s\" > " OUT_DIR "/%s.build.out 2>&1", temp_project_dir, name);
    code = run_command(build_cmd);
    ASSERT_TRUE(code == 0, name);

    snprintf(run_cmd, sizeof(run_cmd), "\"%s\" > " OUT_DIR "/%s.run.out 2>&1", temp_exe_path, name);
    code = run_command(run_cmd);
    ASSERT_TRUE(code == expected_code, name);
}

static void test_main_signature_error(void) {
    int code = run_command(DRIVER " build tests/behavior/functions_main_signature_error > " OUT_DIR "/main-signature.out 2>&1");
    ASSERT_TRUE(code != 0, "main signature build fails");
    ASSERT_TRUE(file_contains(OUT_DIR "/main-signature.out", "main function cannot have parameters in the current C emitter subset"), "main signature message");
}

static void test_invalid_call_error(void) {
    int code = run_command(DRIVER " verify tests/behavior/functions_invalid_call_error > " OUT_DIR "/invalid-call.out 2>&1");
    ASSERT_TRUE(code != 0, "invalid call verify fails");
    ASSERT_TRUE(file_contains(OUT_DIR "/invalid-call.out", "tests/behavior/functions_invalid_call_error/src/app/main.zt:8:"), "invalid call includes source line");
    ASSERT_TRUE(file_contains(OUT_DIR "/invalid-call.out", "missing argument for parameter 'b' in call to 'add'"), "invalid call message");
}

int main(void) {
    if (!ensure_dir(OUT_DIR)) {
        fprintf(stderr, "FAIL: cannot create %s\n", OUT_DIR);
        return 1;
    }

    test_behavior_project(
        "tests/behavior/functions_calls",
        "tests\\behavior\\functions_calls\\build\\functions-calls.exe",
        6,
        "functions_calls");

    test_behavior_project(
        "tests/behavior/functions_named_args",
        "tests\\behavior\\functions_named_args\\build\\functions-named-args.exe",
        6,
        "functions_named_args");

    test_behavior_project(
        "tests/behavior/functions_defaults",
        "tests\\behavior\\functions_defaults\\build\\functions-defaults.exe",
        18,
        "functions_defaults");

    test_main_signature_error();
    test_invalid_call_error();

    printf("M11 conformance tests: %d/%d passed\n", tests_passed, tests_run);
    return tests_passed == tests_run ? 0 : 1;
}
