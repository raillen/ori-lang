#include <ctype.h>
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
#define OUT_DIR ".ztc-tmp/tests/conformance-m16"

#define ASSERT_TRUE(condition, msg) do { \
    tests_run++; \
    if (condition) { tests_passed++; } \
    else { fprintf(stderr, "FAIL: %s\n", msg); } \
} while (0)

typedef struct valid_project {
    const char *project_dir;
    const char *exe_path;
    const char *name;
    int expected_exit;
} valid_project;

typedef struct invalid_project {
    const char *project_dir;
    const char *name;
    const char *command;
    const char *expected_fragments_path;
} invalid_project;

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

static char *trim_line(char *line) {
    char *end;

    while (*line != '\0' && isspace((unsigned char)*line)) {
        line += 1;
    }

    end = line + strlen(line);
    while (end > line && isspace((unsigned char)*(end - 1))) {
        end -= 1;
    }
    *end = '\0';
    return line;
}

static int file_contains_all_lines(const char *actual_path, const char *expected_fragments_path) {
    char *actual = read_file(actual_path);
    char *expected = read_file(expected_fragments_path);
    char *line;
    int ok = 1;

    normalize_newlines(actual);
    normalize_newlines(expected);

    if (actual == NULL || expected == NULL) {
        fprintf(stderr, "missing file for fragment comparison: actual=%s expected=%s\n", actual_path, expected_fragments_path);
        free(actual);
        free(expected);
        return 0;
    }

    line = strtok(expected, "\n");
    while (line != NULL) {
        char *fragment = trim_line(line);
        if (fragment[0] == '\0') continue;
        if (strstr(actual, fragment) == NULL) {
            fprintf(stderr, "missing fragment in %s: %s\n", actual_path, fragment);
            ok = 0;
        }
        line = strtok(NULL, "\n");
    }

    free(actual);
    free(expected);
    return ok;
}

static int run_command(const char *command) {
    return decode_exit_code(system(command));
}

static void test_valid_project(const valid_project *project) {
    char temp_project_dir[1024];
    char temp_exe_path[1024];
    char build_cmd[2048];
    char run_cmd[2048];
    int code;

    if (!zt_prepare_sandbox_project(
            project->project_dir,
            project->name,
            project->exe_path,
            OUT_DIR,
            ensure_dir,
            temp_project_dir,
            sizeof(temp_project_dir),
            temp_exe_path,
            sizeof(temp_exe_path))) {
        ASSERT_TRUE(0, project->name);
        return;
    }

    snprintf(build_cmd, sizeof(build_cmd), DRIVER " build \"%s\" > " OUT_DIR "/%s.build.out 2>&1", temp_project_dir, project->name);
    code = run_command(build_cmd);
    ASSERT_TRUE(code == 0, project->name);

    snprintf(run_cmd, sizeof(run_cmd), "\"%s\" > " OUT_DIR "/%s.run.out 2>&1", temp_exe_path, project->name);
    code = run_command(run_cmd);
    ASSERT_TRUE(code == project->expected_exit, project->name);
}

static void test_invalid_project(const invalid_project *project) {
    char verify_cmd[1024];
    char output_path[256];
    int code;

    snprintf(output_path, sizeof(output_path), OUT_DIR "/%s.diag.out", project->name);
    snprintf(verify_cmd, sizeof(verify_cmd), DRIVER " %s %s > %s 2>&1", project->command, project->project_dir, output_path);
    code = run_command(verify_cmd);

    ASSERT_TRUE(code != 0, project->name);
    ASSERT_TRUE(file_contains_all_lines(output_path, project->expected_fragments_path), project->name);
}

static void test_emit_c_golden(void) {
    int code = run_command(DRIVER " emit-c tests/behavior/simple_app > " OUT_DIR "/simple-app.actual.c 2> " OUT_DIR "/simple-app.emit.err");

    ASSERT_TRUE(code == 0, "simple_app emit-c");
    ASSERT_TRUE(file_equals(OUT_DIR "/simple-app.actual.c", "tests/behavior/simple_app/golden/simple-app.c"), "simple_app golden C");
}

static void test_emit_c_fragments(const char *project_dir, const char *name, const char *fragments_path) {
    char command[1024];
    char output_path[256];
    int code;

    snprintf(output_path, sizeof(output_path), OUT_DIR "/%s.emit.c", name);
    snprintf(command, sizeof(command), DRIVER " emit-c %s > %s 2> " OUT_DIR "/%s.emit.err", project_dir, output_path, name);
    code = run_command(command);

    ASSERT_TRUE(code == 0, name);
    ASSERT_TRUE(file_contains_all_lines(output_path, fragments_path), name);
}

int main(void) {
    const valid_project valid_projects[] = {
        { "tests/behavior/simple_app", "tests\\behavior\\simple_app\\build\\simple-app.exe", "simple_app", 42 },
        { "tests/behavior/control_flow_while", "tests\\behavior\\control_flow_while\\build\\control-flow-while.exe", "control_flow_while", 6 },
        { "tests/behavior/control_flow_repeat", "tests\\behavior\\control_flow_repeat\\build\\control-flow-repeat.exe", "control_flow_repeat", 9 },
        { "tests/behavior/control_flow_match", "tests\\behavior\\control_flow_match\\build\\control-flow-match.exe", "control_flow_match", 7 },
        { "tests/behavior/control_flow_break_continue", "tests\\behavior\\control_flow_break_continue\\build\\control-flow-break-continue.exe", "control_flow_break_continue", 8 },
        { "tests/behavior/functions_calls", "tests\\behavior\\functions_calls\\build\\functions-calls.exe", "functions_calls", 6 },
        { "tests/behavior/functions_named_args", "tests\\behavior\\functions_named_args\\build\\functions-named-args.exe", "functions_named_args", 6 },
        { "tests/behavior/functions_defaults", "tests\\behavior\\functions_defaults\\build\\functions-defaults.exe", "functions_defaults", 18 },
        { "tests/behavior/structs_constructor", "tests\\behavior\\structs_constructor\\build\\structs-constructor.exe", "structs_constructor", 42 },
        { "tests/behavior/structs_field_defaults", "tests\\behavior\\structs_field_defaults\\build\\structs-field-defaults.exe", "structs_field_defaults", 117 },
        { "tests/behavior/structs_field_read", "tests\\behavior\\structs_field_read\\build\\structs-field-read.exe", "structs_field_read", 11 },
        { "tests/behavior/structs_field_update", "tests\\behavior\\structs_field_update\\build\\structs-field-update.exe", "structs_field_update", 12 },
        { "tests/behavior/methods_inherent", "tests\\behavior\\methods_inherent\\build\\methods-inherent.exe", "methods_inherent", 7 },
        { "tests/behavior/methods_inherent_apply", "tests\\behavior\\methods_inherent_apply\\build\\methods-inherent-apply.exe", "methods_inherent_apply", 7 },
        { "tests/behavior/methods_mutating", "tests\\behavior\\methods_mutating\\build\\methods-mutating.exe", "methods_mutating", 6 },
        { "tests/behavior/methods_trait_apply", "tests\\behavior\\methods_trait_apply\\build\\methods-trait-apply.exe", "methods_trait_apply", 8 },
        { "tests/behavior/list_basic", "tests\\behavior\\list_basic\\build\\list-basic.exe", "list_basic", 18 },
        { "tests/behavior/list_text_basic", "tests\\behavior\\list_text_basic\\build\\list-text-basic.exe", "list_text_basic", 0 },
        { "tests/behavior/list_dyn_trait_basic", "tests\\behavior\\list_dyn_trait_basic\\build\\list-dyn-trait-basic.exe", "list_dyn_trait_basic", 16 },
        { "tests/behavior/map_basic", "tests\\behavior\\map_basic\\build\\map-basic.exe", "map_basic", 0 },
        { "tests/behavior/map_int_text_basic", "tests\\behavior\\map_int_text_basic\\build\\map-int-text-basic.exe", "map_int_text_basic", 7 },
        { "tests/behavior/map_safe_get", "tests\\behavior\\map_safe_get\\build\\map-safe-get.exe", "map_safe_get", 15 },
        { "tests/behavior/list_safe_get", "tests\\behavior\\list_safe_get\\build\\list-safe-get.exe", "list_safe_get", 27 },
        { "tests/behavior/value_semantics_collections", "tests\\behavior\\value_semantics_collections\\build\\value-semantics-collections.exe", "value_semantics_collections", 131 },
        { "tests/behavior/value_semantics_struct_managed", "tests\\behavior\\value_semantics_struct_managed\\build\\value-semantics-struct-managed.exe", "value_semantics_struct_managed", 131 },
        { "tests/behavior/value_semantics_arc_isolation", "tests\\behavior\\value_semantics_arc_isolation\\build\\value-semantics-arc-isolation.exe", "value_semantics_arc_isolation", 158 },
        { "tests/behavior/value_semantics_optional_result_managed", "tests\\behavior\\value_semantics_optional_result_managed\\build\\value-semantics-optional-result-managed.exe", "value_semantics_optional_result_managed", 0 },
        { "tests/behavior/std_collections_managed_arc", "tests\\behavior\\std_collections_managed_arc\\build\\std-collections-managed-arc.exe", "std_collections_managed_arc", 12 },
        { "tests/behavior/optional_result_basic", "tests\\behavior\\optional_result_basic\\build\\optional-result-basic.exe", "optional_result_basic", 0 },
        { "tests/behavior/control_flow_for_list", "tests\\behavior\\control_flow_for_list\\build\\control-flow-for-list.exe", "control_flow_for_list", 13 },
        { "tests/behavior/control_flow_for_map", "tests\\behavior\\control_flow_for_map\\build\\control-flow-for-map.exe", "control_flow_for_map", 6 },
        { "tests/behavior/list_slice_len", "tests\\behavior\\list_slice_len\\build\\list-slice-len.exe", "list_slice_len", 37 },
        { "tests/behavior/text_slice_len", "tests\\behavior\\text_slice_len\\build\\text-slice-len.exe", "text_slice_len", 8 },
        { "tests/behavior/text_utf8_index_slice", "tests\\behavior\\text_utf8_index_slice\\build\\text-utf8-index-slice.exe", "text_utf8_index_slice", 21 },
        { "tests/behavior/std_text_basic", "tests\\behavior\\std_text_basic\\build\\std_text_basic.exe", "std_text_basic", 0 },
        { "tests/behavior/std_concurrent_boundary_copy_basic", "tests\\behavior\\std_concurrent_boundary_copy_basic\\build\\std_concurrent_boundary_copy_basic.exe", "std_concurrent_boundary_copy_basic", 0 },
        { "tests/behavior/to_text_builtin_basic", "tests\\behavior\\to_text_builtin_basic\\build\\to-text-builtin-basic.exe", "to_text_builtin_basic", 0 },
        { "tests/behavior/map_len_basic", "tests\\behavior\\map_len_basic\\build\\map-len-basic.exe", "map_len_basic", 2 },
        { "tests/behavior/result_question_basic", "tests\\behavior\\result_question_basic\\build\\result-question-basic.exe", "result_question_basic", 0 },
        { "tests/behavior/bytes_hex_literal", "tests\\behavior\\bytes_hex_literal\\build\\bytes-hex-literal.exe", "bytes_hex_literal", 9 },
        { "tests/behavior/std_bytes_utf8", "tests\\behavior\\std_bytes_utf8\\build\\std-bytes-utf8.exe", "std_bytes_utf8", 14 },
        { "tests/behavior/std_bytes_ops", "tests\\behavior\\std_bytes_ops\\build\\std-bytes-ops.exe", "std_bytes_ops", 7 },
        { "tests/behavior/std_fs_ops_basic", "tests\\behavior\\std_fs_ops_basic\\build\\std-fs-ops-basic.exe", "std_fs_ops_basic", 0 },
        { "tests/behavior/multifile_import_alias", "tests\\behavior\\multifile_import_alias\\build\\multifile-import-alias.exe", "multifile_import_alias", 42 },
        { "tests/behavior/public_const_module", "tests\\behavior\\public_const_module\\build\\public-const-module.exe", "public_const_module", 42 },
        { "tests/behavior/where_contracts_ok", "tests\\behavior\\where_contracts_ok\\build\\where-contracts-ok.exe", "where_contracts_ok", 40 }
    };
    const invalid_project invalid_projects[] = {
        { "tests/behavior/error_syntax", "error_syntax", "verify", "tests/fixtures/diagnostics/error_syntax.contains.txt" },
        { "tests/behavior/error_type_mismatch", "error_type_mismatch", "verify", "tests/fixtures/diagnostics/error_type_mismatch.contains.txt" },
        { "tests/behavior/functions_main_signature_error", "functions_main_signature_error", "build", "tests/fixtures/diagnostics/functions_main_signature.contains.txt" },
        { "tests/behavior/functions_invalid_call_error", "functions_invalid_call_error", "verify", "tests/fixtures/diagnostics/functions_invalid_call.contains.txt" },
        { "tests/behavior/multifile_missing_import", "multifile_missing_import", "build", "tests/fixtures/diagnostics/multifile_missing_import.contains.txt" },
        { "tests/behavior/multifile_namespace_mismatch", "multifile_namespace_mismatch", "build", "tests/fixtures/diagnostics/multifile_namespace_mismatch.contains.txt" },
        { "tests/behavior/multifile_duplicate_symbol", "multifile_duplicate_symbol", "build", "tests/fixtures/diagnostics/multifile_duplicate_symbol.contains.txt" },
        { "tests/behavior/multifile_import_cycle", "multifile_import_cycle", "build", "tests/fixtures/diagnostics/multifile_import_cycle.contains.txt" },
        { "tests/behavior/multifile_private_access", "multifile_private_access", "build", "tests/fixtures/diagnostics/multifile_private_access.contains.txt" },
        { "tests/behavior/project_unknown_key_manifest", "project_unknown_key_manifest", "verify", "tests/fixtures/diagnostics/project_unknown_key_manifest.contains.txt" },
        { "tests/behavior/std_concurrent_boundary_copy_unsupported_error", "std_concurrent_boundary_copy_unsupported_error", "verify", "tests/fixtures/diagnostics/std_concurrent_boundary_copy_unsupported_error.contains.txt" },
        { "tests/behavior/monomorphization_limit_error", "monomorphization_limit_error", "verify", "tests/fixtures/diagnostics/monomorphization_limit_error.contains.txt" },
        { "tests/behavior/mutability_const_reassign_error", "mutability_const_reassign_error", "verify", "tests/fixtures/diagnostics/mutability_const_reassign.contains.txt" },
        { "tests/behavior/result_optional_propagation_error", "result_optional_propagation_error", "verify", "tests/fixtures/diagnostics/result_optional_propagation.contains.txt" },
        { "tests/behavior/runtime_index_error --run", "runtime_index_error", "build", "tests/fixtures/diagnostics/runtime_index_error.contains.txt" },
        { "tests/behavior/where_contract_param_error --run", "where_contract_param_error", "build", "tests/fixtures/diagnostics/where_contract_param_error.contains.txt" },
        { "tests/behavior/where_contract_construct_error --run", "where_contract_construct_error", "build", "tests/fixtures/diagnostics/where_contract_construct_error.contains.txt" },
        { "tests/behavior/where_contract_field_assign_error --run", "where_contract_field_assign_error", "build", "tests/fixtures/diagnostics/where_contract_field_assign_error.contains.txt" }
    };
    size_t i;

    if (!ensure_dir(OUT_DIR)) {
        fprintf(stderr, "FAIL: cannot create %s\n", OUT_DIR);
        return 1;
    }

    for (i = 0; i < sizeof(valid_projects) / sizeof(valid_projects[0]); i += 1) {
        test_valid_project(&valid_projects[i]);
    }

    for (i = 0; i < sizeof(invalid_projects) / sizeof(invalid_projects[0]); i += 1) {
        test_invalid_project(&invalid_projects[i]);
    }

    test_emit_c_golden();
    test_emit_c_fragments("tests/behavior/list_basic", "list_basic_c_fragments", "tests/fixtures/c/m16_list_basic.contains.txt");
    test_emit_c_fragments("tests/behavior/map_basic", "map_basic_c_fragments", "tests/fixtures/c/m16_map_basic.contains.txt");
    test_emit_c_fragments("tests/behavior/structs_constructor", "structs_c_fragments", "tests/fixtures/c/m16_structs.contains.txt");

    printf("M16 conformance tests: %d/%d passed\n", tests_passed, tests_run);
    return tests_passed == tests_run ? 0 : 1;
}
