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
#define OUT_DIR ".ztc-tmp/tests/conformance-m12"

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

int main(void) {
    if (!ensure_dir(OUT_DIR)) {
        fprintf(stderr, "FAIL: cannot create %s\n", OUT_DIR);
        return 1;
    }

    test_behavior_project(
        "tests/behavior/structs_constructor",
        "tests\\behavior\\structs_constructor\\build\\structs-constructor.exe",
        42,
        "structs_constructor");

    test_behavior_project(
        "tests/behavior/structs_field_defaults",
        "tests\\behavior\\structs_field_defaults\\build\\structs-field-defaults.exe",
        117,
        "structs_field_defaults");

    test_behavior_project(
        "tests/behavior/structs_field_read",
        "tests\\behavior\\structs_field_read\\build\\structs-field-read.exe",
        11,
        "structs_field_read");

    test_behavior_project(
        "tests/behavior/structs_field_update",
        "tests\\behavior\\structs_field_update\\build\\structs-field-update.exe",
        12,
        "structs_field_update");

    test_behavior_project(
        "tests/behavior/methods_inherent",
        "tests\\behavior\\methods_inherent\\build\\methods-inherent.exe",
        7,
        "methods_inherent");

    test_behavior_project(
        "tests/behavior/methods_mutating",
        "tests\\behavior\\methods_mutating\\build\\methods-mutating.exe",
        6,
        "methods_mutating");

    test_behavior_project(
        "tests/behavior/methods_trait_apply",
        "tests\\behavior\\methods_trait_apply\\build\\methods-trait-apply.exe",
        8,
        "methods_trait_apply");

    printf("M12 conformance tests: %d/%d passed\n", tests_passed, tests_run);
    return tests_passed == tests_run ? 0 : 1;
}
