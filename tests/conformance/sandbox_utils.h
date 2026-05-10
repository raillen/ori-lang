#ifndef ZT_CONFORMANCE_SANDBOX_UTILS_H
#define ZT_CONFORMANCE_SANDBOX_UTILS_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef int (*zt_ensure_dir_fn)(const char *path);

static unsigned long zt_sandbox_counter = 0;

static const char *zt_path_basename(const char *path) {
    const char *last_slash = strrchr(path, '/');
    const char *last_backslash = strrchr(path, '\\');
    const char *last = last_slash;
    if (last_backslash != NULL && (last == NULL || last_backslash > last)) {
        last = last_backslash;
    }
    return last == NULL ? path : last + 1;
}

#ifdef _WIN32
static void zt_windows_normalize_separators(char *path) {
    char *cursor = path;
    while (*cursor != '\0') {
        if (*cursor == '/') {
            *cursor = '\\';
        }
        cursor += 1;
    }
}
#endif

static char *zt_read_file_text(const char *path) {
    FILE *file;
    long file_size;
    char *buffer;

    file = fopen(path, "rb");
    if (file == NULL) return NULL;

    if (fseek(file, 0, SEEK_END) != 0) {
        fclose(file);
        return NULL;
    }

    file_size = ftell(file);
    if (file_size < 0) {
        fclose(file);
        return NULL;
    }

    if (fseek(file, 0, SEEK_SET) != 0) {
        fclose(file);
        return NULL;
    }

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

static int zt_write_file_text(const char *path, const char *text, size_t length) {
    FILE *file = fopen(path, "wb");
    if (file == NULL) return 0;
    if (length > 0 && fwrite(text, 1, length, file) != length) {
        fclose(file);
        return 0;
    }
    fclose(file);
    return 1;
}

static int zt_manifest_set_output_build_tmp(const char *manifest_path) {
    const char *needle = "output = \"build\"";
    const char *replacement = "output = \"build_tmp\"";
    char *text = zt_read_file_text(manifest_path);
    char *pos;
    size_t prefix_len;
    size_t suffix_len;
    size_t replacement_len = strlen(replacement);
    size_t needle_len = strlen(needle);
    size_t new_len;
    char *updated;
    int ok;

    if (text == NULL) return 0;

    pos = strstr(text, needle);
    if (pos == NULL) {
        free(text);
        return 0;
    }

    prefix_len = (size_t)(pos - text);
    suffix_len = strlen(pos + needle_len);
    new_len = prefix_len + replacement_len + suffix_len;
    updated = (char *)malloc(new_len + 1);
    if (updated == NULL) {
        free(text);
        return 0;
    }

    memcpy(updated, text, prefix_len);
    memcpy(updated + prefix_len, replacement, replacement_len);
    memcpy(updated + prefix_len + replacement_len, pos + needle_len, suffix_len);
    updated[new_len] = '\0';

    ok = zt_write_file_text(manifest_path, updated, new_len);

    free(updated);
    free(text);
    return ok;
}

static int zt_prepare_sandbox_project(
    const char *project_dir,
    const char *name,
    const char *exe_template_path,
    const char *out_dir,
    zt_ensure_dir_fn ensure_dir,
    char *temp_project_dir,
    size_t temp_project_dir_cap,
    char *temp_exe_path,
    size_t temp_exe_path_cap
) {
    char sandbox_root[512];
    char manifest_path[1024];
    char copy_cmd[4096];
    const char *exe_name = zt_path_basename(exe_template_path);

    zt_sandbox_counter += 1;

#ifdef _WIN32
    if (snprintf(sandbox_root, sizeof(sandbox_root), "%s\\sandbox", out_dir) >= (int)sizeof(sandbox_root)) return 0;
    zt_windows_normalize_separators(sandbox_root);
    if (!ensure_dir(sandbox_root)) return 0;
    if (snprintf(temp_project_dir, temp_project_dir_cap, "%s\\%s_%lu", sandbox_root, name, zt_sandbox_counter) >= (int)temp_project_dir_cap) return 0;
    zt_windows_normalize_separators(temp_project_dir);

    if (snprintf(copy_cmd, sizeof(copy_cmd), "if exist \"%s\" rmdir /s /q \"%s\" >nul 2>&1", temp_project_dir, temp_project_dir) >= (int)sizeof(copy_cmd)) return 0;
    (void)system(copy_cmd);

    if (snprintf(copy_cmd, sizeof(copy_cmd), "xcopy /E /I /Q /Y \"%s\" \"%s\" >nul", project_dir, temp_project_dir) >= (int)sizeof(copy_cmd)) return 0;
    if (system(copy_cmd) != 0) return 0;

    if (snprintf(manifest_path, sizeof(manifest_path), "%s\\zenith.ztproj", temp_project_dir) >= (int)sizeof(manifest_path)) return 0;
    zt_windows_normalize_separators(manifest_path);
    if (snprintf(temp_exe_path, temp_exe_path_cap, "%s\\build_tmp\\%s", temp_project_dir, exe_name) >= (int)temp_exe_path_cap) return 0;
    zt_windows_normalize_separators(temp_exe_path);
#else
    if (snprintf(sandbox_root, sizeof(sandbox_root), "%s/sandbox", out_dir) >= (int)sizeof(sandbox_root)) return 0;
    if (!ensure_dir(sandbox_root)) return 0;
    if (snprintf(temp_project_dir, temp_project_dir_cap, "%s/%s_%lu", sandbox_root, name, zt_sandbox_counter) >= (int)temp_project_dir_cap) return 0;

    if (snprintf(copy_cmd, sizeof(copy_cmd), "rm -rf \"%s\" && cp -R \"%s\" \"%s\"", temp_project_dir, project_dir, temp_project_dir) >= (int)sizeof(copy_cmd)) return 0;
    if (system(copy_cmd) != 0) return 0;

    if (snprintf(manifest_path, sizeof(manifest_path), "%s/zenith.ztproj", temp_project_dir) >= (int)sizeof(manifest_path)) return 0;
    if (snprintf(temp_exe_path, temp_exe_path_cap, "%s/build_tmp/%s", temp_project_dir, exe_name) >= (int)temp_exe_path_cap) return 0;
#endif

    return zt_manifest_set_output_build_tmp(manifest_path);
}

#endif
