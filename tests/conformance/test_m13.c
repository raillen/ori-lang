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
#define OUT_DIR ".ztc-tmp/tests/conformance-m13"

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
        "tests/behavior/list_basic",
        "tests\\behavior\\list_basic\\build\\list-basic.exe",
        18,
        "list_basic");

    test_behavior_project(
        "tests/behavior/list_text_basic",
        "tests\\behavior\\list_text_basic\\build\\list-text-basic.exe",
        0,
        "list_text_basic");

    test_behavior_project(
        "tests/behavior/map_basic",
        "tests\\behavior\\map_basic\\build\\map-basic.exe",
        0,
        "map_basic");

    test_behavior_project(
        "tests/behavior/map_safe_get",
        "tests\\behavior\\map_safe_get\\build\\map-safe-get.exe",
        15,
        "map_safe_get");

    test_behavior_project(
        "tests/behavior/list_safe_get",
        "tests\\behavior\\list_safe_get\\build\\list-safe-get.exe",
        27,
        "list_safe_get");

    test_behavior_project(
        "tests/behavior/optional_result_basic",
        "tests\\behavior\\optional_result_basic\\build\\optional-result-basic.exe",
        0,
        "optional_result_basic");

    test_behavior_project(
        "tests/behavior/list_slice_len",
        "tests\\behavior\\list_slice_len\\build\\list-slice-len.exe",
        37,
        "list_slice_len");

    test_behavior_project(
        "tests/behavior/text_slice_len",
        "tests\\behavior\\text_slice_len\\build\\text-slice-len.exe",
        8,
        "text_slice_len");

    test_behavior_project(
        "tests/behavior/map_len_basic",
        "tests\\behavior\\map_len_basic\\build\\map-len-basic.exe",
        2,
        "map_len_basic");

    test_behavior_project(
        "tests/behavior/result_question_basic",
        "tests\\behavior\\result_question_basic\\build\\result-question-basic.exe",
        0,
        "result_question_basic");

    test_behavior_project(
        "tests/behavior/value_semantics_collections",
        "tests\\behavior\\value_semantics_collections\\build\\value-semantics-collections.exe",
        131,
        "value_semantics_collections");

    test_behavior_project(
        "tests/behavior/value_semantics_struct_managed",
        "tests\\behavior\\value_semantics_struct_managed\\build\\value-semantics-struct-managed.exe",
        131,
        "value_semantics_struct_managed");

    test_behavior_project(
        "tests/behavior/value_semantics_arc_isolation",
        "tests\\behavior\\value_semantics_arc_isolation\\build\\value-semantics-arc-isolation.exe",
        158,
        "value_semantics_arc_isolation");

    test_behavior_project(
        "tests/behavior/value_semantics_optional_result_managed",
        "tests\\behavior\\value_semantics_optional_result_managed\\build\\value-semantics-optional-result-managed.exe",
        0,
        "value_semantics_optional_result_managed");

    test_behavior_project(
        "tests/behavior/std_collections_managed_arc",
        "tests\\behavior\\std_collections_managed_arc\\build\\std-collections-managed-arc.exe",
        12,
        "std_collections_managed_arc");

    printf("M13 conformance tests: %d/%d passed\n", tests_passed, tests_run);
    return tests_passed == tests_run ? 0 : 1;
}
