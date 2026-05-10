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

#define DRIVER "zt.exe"
#define OUT_DIR ".ztc-tmp/tests/conformance"

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

static void normalize_newlines(char *text) {
    char *read_cursor;
    char *write_cursor;

    if (text == NULL) return;

    read_cursor = text;
    write_cursor = text;
    while (*read_cursor != '\0') {
        if (*read_cursor != '\r') {
            *write_cursor = *read_cursor;
            write_cursor += 1;
        }
        read_cursor += 1;
    }
    *write_cursor = '\0';
}

static int file_equals(const char *actual_path, const char *expected_path) {
    char *actual = read_file(actual_path);
    char *expected = read_file(expected_path);
    int ok;

    normalize_newlines(actual);
    normalize_newlines(expected);
    ok = actual != NULL && expected != NULL && strcmp(actual, expected) == 0;

    if (!ok) {
        fprintf(stderr, "golden mismatch\nactual: %s\nexpected: %s\n", actual_path, expected_path);
        if (actual != NULL) fprintf(stderr, "actual text:\n%s\n", actual);
    }

    free(actual);
    free(expected);
    return ok;
}

static int file_contains(const char *path, const char *fragment) {
    char *text = read_file(path);
    int ok = text != NULL && strstr(text, fragment) != NULL;

    if (!ok) {
        fprintf(stderr, "missing fragment in %s: %s\n", path, fragment);
        if (text != NULL) fprintf(stderr, "text:\n%s\n", text);
    }

    free(text);
    return ok;
}

static int file_not_contains(const char *path, const char *fragment) {
    char *text = read_file(path);
    int ok = text != NULL && strstr(text, fragment) == NULL;

    if (!ok) {
        fprintf(stderr, "unexpected fragment in %s: %s\n", path, fragment);
        if (text != NULL) fprintf(stderr, "text:\n%s\n", text);
    }

    free(text);
    return ok;
}

static int run_command(const char *command) {
    return decode_exit_code(system(command));
}

static void test_project_info_smoke(void) {
    int code = run_command(DRIVER " project-info tests/behavior/simple_app > " OUT_DIR "/project-info.out 2>&1");

    ASSERT_TRUE(code == 0, "project-info succeeds");
    ASSERT_TRUE(file_contains(OUT_DIR "/project-info.out", "project simple-app"), "project-info prints project name");
    ASSERT_TRUE(file_contains(OUT_DIR "/project-info.out", "entry_path: tests/behavior/simple_app/src/app/main.zt"), "project-info prints entry path");
}

static void test_check_alias(void) {
    int code = run_command(DRIVER " check tests/behavior/simple_app > " OUT_DIR "/check.out 2>&1");

    ASSERT_TRUE(code == 0, "check alias succeeds");
    ASSERT_TRUE(file_contains(OUT_DIR "/check.out", "verification ok"), "check alias runs verification pipeline");
}

static void test_check_alias_ci_output(void) {
    int code = run_command(DRIVER " check tests/behavior/simple_app --ci > " OUT_DIR "/check-ci.out 2>&1");

    ASSERT_TRUE(code == 0, "check alias with --ci succeeds");
    ASSERT_TRUE(file_contains(OUT_DIR "/check-ci.out", "check ok"), "check alias with --ci prints short success");
}

static void test_doc_check_canonical(void) {
    int code = run_command(DRIVER " doc check tests/behavior/simple_app > " OUT_DIR "/doc-check.out 2>&1");

    ASSERT_TRUE(code == 0, "doc check succeeds");
    ASSERT_TRUE(file_contains(OUT_DIR "/doc-check.out", "doc check ok"), "doc check prints success");
}

static void test_fmt_check_bootstrap(void) {
    int code = run_command(DRIVER " fmt tests/behavior/simple_app --check > " OUT_DIR "/fmt-check.out 2>&1");

    ASSERT_TRUE(code == 0 || code == 1, "fmt --check returns gate status");
    ASSERT_TRUE(file_contains(OUT_DIR "/fmt-check.out", "fmt check"), "fmt --check prints bootstrap status");
}

static void test_test_alias_bootstrap(void) {
    int code = run_command(DRIVER " test tests/behavior/simple_app > " OUT_DIR "/test-alias.out 2>&1");

    ASSERT_TRUE(code == 0, "test alias succeeds");
    ASSERT_TRUE(file_contains(OUT_DIR "/test-alias.out", "test bootstrap fallback"), "test alias prints fallback note when no std.test outcome is emitted");
}

static void test_test_attr_pass_skip_ci(void) {
    int code = run_command(DRIVER " test tests/behavior/std_test_attr_pass_skip --ci > " OUT_DIR "/test-attr-pass-skip.out 2>&1");

    ASSERT_TRUE(code == 0, "test attr pass+skip succeeds");
    ASSERT_TRUE(
        file_contains(OUT_DIR "/test-attr-pass-skip.out", "test ok (pass=1 skip=1)"),
        "test attr pass+skip summary");
}

static void test_test_attr_fail_ci(void) {
    int code = run_command(DRIVER " test tests/behavior/std_test_attr_fail --ci > " OUT_DIR "/test-attr-fail.out 2>&1");

    ASSERT_TRUE(code != 0, "test attr fail exits non-zero");
    ASSERT_TRUE(
        file_contains(OUT_DIR "/test-attr-fail.out", "test failed (pass=1 skip=1 fail=1)"),
        "test attr fail summary");
}

static void test_ci_short_diagnostic_renderer(void) {
    int code = run_command(DRIVER " verify tests/behavior/error_type_mismatch --ci > " OUT_DIR "/error-type-ci.out 2>&1");

    ASSERT_TRUE(code != 0, "verify with --ci fails for type mismatch");
    ASSERT_TRUE(file_contains(OUT_DIR "/error-type-ci.out", "[type.mismatch]"), "ci output keeps stable code");
    ASSERT_TRUE(file_not_contains(OUT_DIR "/error-type-ci.out", "\nwhere\n"), "ci output uses short renderer");
}

static void test_emit_c_golden(void) {
    int code = run_command(DRIVER " emit-c tests/behavior/simple_app > " OUT_DIR "/simple-app.actual.c 2> " OUT_DIR "/simple-app.emit.err");

    ASSERT_TRUE(code == 0, "emit-c succeeds");
    ASSERT_TRUE(file_equals(OUT_DIR "/simple-app.actual.c", "tests/behavior/simple_app/golden/simple-app.c"), "emit-c matches golden C");
}

static void test_build_and_observable_exit_code(void) {
    char temp_project_dir[1024];
    char temp_exe_path[1024];
    char build_cmd[2048];
    char run_cmd[2048];
    int code;
    int run_code;

    if (!zt_prepare_sandbox_project(
            "tests/behavior/simple_app",
            "simple_app",
            "tests\\behavior\\simple_app\\build\\simple-app.exe",
            OUT_DIR,
            ensure_dir,
            temp_project_dir,
            sizeof(temp_project_dir),
            temp_exe_path,
            sizeof(temp_exe_path))) {
        ASSERT_TRUE(0, "build succeeds");
        return;
    }

    snprintf(build_cmd, sizeof(build_cmd), DRIVER " build \"%s\" > " OUT_DIR "/simple-app.build.out 2>&1", temp_project_dir);
    code = run_command(build_cmd);
    ASSERT_TRUE(code == 0, "build succeeds");

    snprintf(run_cmd, sizeof(run_cmd), "\"%s\" > " OUT_DIR "/simple-app.run.out 2>&1", temp_exe_path);
    run_code = run_command(run_cmd);
    ASSERT_TRUE(run_code == 42, "built executable returns 42");
}

static void test_type_error_has_span(void) {
    int code = run_command(DRIVER " verify tests/behavior/error_type_mismatch > " OUT_DIR "/error-type.out 2>&1");

    ASSERT_TRUE(code != 0, "type mismatch project fails");
    ASSERT_TRUE(file_contains(OUT_DIR "/error-type.out", "tests/behavior/error_type_mismatch/src/app/main.zt:4:"), "type mismatch includes source line");
    ASSERT_TRUE(file_contains(OUT_DIR "/error-type.out", "[type.mismatch]"), "type mismatch includes diagnostic code");
    ASSERT_TRUE(file_contains(OUT_DIR "/error-type.out", "const initializer type mismatch"), "type mismatch includes message");
    ASSERT_TRUE(file_contains(OUT_DIR "/error-type.out", "stage: type"), "type mismatch includes stage");
}

static void test_parse_error_has_span(void) {
    int code = run_command(DRIVER " verify tests/behavior/error_syntax > " OUT_DIR "/error-syntax.out 2>&1");

    ASSERT_TRUE(code != 0, "syntax error project fails");
    ASSERT_TRUE(file_contains(OUT_DIR "/error-syntax.out", "tests/behavior/error_syntax/src/app/main.zt:4:"), "syntax error includes source line");
    ASSERT_TRUE(file_contains(OUT_DIR "/error-syntax.out", "[syntax.unexpected_token]"), "syntax error includes diagnostic code");
    ASSERT_TRUE(file_contains(OUT_DIR "/error-syntax.out", "expected"), "syntax error includes parser expectation");
    ASSERT_TRUE(file_contains(OUT_DIR "/error-syntax.out", "stage: parser"), "syntax error includes parser stage");
}

int main(void) {
    if (!ensure_dir(OUT_DIR)) {
        fprintf(stderr, "FAIL: cannot create %s\n", OUT_DIR);
        return 1;
    }

    test_project_info_smoke();
    test_check_alias();
    test_check_alias_ci_output();
    test_doc_check_canonical();
    test_fmt_check_bootstrap();
    test_test_alias_bootstrap();
    test_test_attr_pass_skip_ci();
    test_test_attr_fail_ci();
    test_ci_short_diagnostic_renderer();
    test_emit_c_golden();
    test_build_and_observable_exit_code();
    test_type_error_has_span();
    test_parse_error_has_span();

    printf("M9 conformance tests: %d/%d passed\n", tests_passed, tests_run);
    return tests_passed == tests_run ? 0 : 1;
}


