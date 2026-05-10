static zt_outcome_text_core_error zt_host_default_read_file(const zt_text *path) {
    char *path_data;
    FILE *file;
    long size_long;
    size_t size;
    char *buffer;
    size_t read_count;
    size_t error_index;
    const char *error_reason;
    zt_text *value;
    zt_core_error path_error;
    zt_outcome_text_core_error outcome;

    path_data = zt_host_prepare_path_copy(path, "zt_host_read_file requires path", &path_error);
    if (path_data == NULL) {
        return zt_fs_outcome_text_failure_error(path_error);
    }

    file = fopen(path_data, "rb");
    if (file == NULL) {
        free(path_data);
        return zt_fs_outcome_text_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    if (fseek(file, 0, SEEK_END) != 0) {
        fclose(file);
        free(path_data);
        return zt_fs_outcome_text_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    size_long = ftell(file);
    if (size_long < 0) {
        fclose(file);
        free(path_data);
        return zt_fs_outcome_text_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    if (fseek(file, 0, SEEK_SET) != 0) {
        fclose(file);
        free(path_data);
        return zt_fs_outcome_text_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    size = (size_t)size_long;
    buffer = (char *)malloc(size + 1);
    if (buffer == NULL) {
        fclose(file);
        free(path_data);
        return zt_fs_outcome_text_failure_error(zt_fs_core_error_from_code_message("fs.io", "failed to allocate file buffer"));
    }

    read_count = 0;
    if (size > 0) {
        read_count = fread(buffer, 1, size, file);
        if (read_count != size) {
            int had_error = ferror(file);
            free(buffer);
            fclose(file);
            free(path_data);
            if (had_error) {
                return zt_fs_outcome_text_failure_error(zt_fs_core_error_from_errno(errno, NULL));
            }
            return zt_fs_outcome_text_failure_error(zt_fs_core_error_from_code_message("fs.io", "failed to read full file"));
        }
    }

    buffer[size] = '\0';
    fclose(file);
    free(path_data);

    if (!zt_utf8_validate((const uint8_t *)buffer, size, &error_index, &error_reason)) {
        char decode_message[256];
        snprintf(
            decode_message,
            sizeof(decode_message),
            "file content is not valid UTF-8 at byte %zu (%s)",
            error_index,
            error_reason != NULL ? error_reason : "invalid encoding");
        free(buffer);
        return zt_fs_outcome_text_failure_error(zt_fs_core_error_from_code_message("fs.io", decode_message));
    }

    value = zt_text_from_utf8(buffer, size);
    free(buffer);
    outcome = zt_outcome_text_core_error_success(value);
    zt_release(value);
    return outcome;
}

static zt_outcome_void_core_error zt_host_default_write_file(const zt_text *path, const zt_text *value) {
    char *path_data;
    FILE *file;
    size_t write_count;
    zt_core_error path_error;

    path_data = zt_host_prepare_path_copy(path, "zt_host_write_file requires path", &path_error);
    if (path_data == NULL) {
        return zt_fs_outcome_void_failure_error(path_error);
    }
    zt_runtime_require_text(value, "zt_host_write_file requires value");
    file = fopen(path_data, "wb");
    if (file == NULL) {
        free(path_data);
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    if (value->len > 0) {
        write_count = fwrite(value->data, 1, value->len, file);
        if (write_count != value->len) {
            fclose(file);
            free(path_data);
            return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
        }
    }

    if (fclose(file) != 0) {
        free(path_data);
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    free(path_data);
    return zt_outcome_void_core_error_success();
}

static zt_outcome_bytes_core_error zt_host_default_fs_read_bytes(const zt_text *path) {
    char *path_data;
    FILE *file;
    long size_long;
    size_t size;
    uint8_t *buffer;
    size_t read_count;
    zt_bytes *value;
    zt_outcome_bytes_core_error outcome;
    zt_core_error path_error;

    path_data = zt_host_prepare_path_copy(path, "zt_host_fs_read_bytes_core requires path", &path_error);
    if (path_data == NULL) {
        return zt_fs_outcome_bytes_failure_error(path_error);
    }

    file = fopen(path_data, "rb");
    if (file == NULL) {
        free(path_data);
        return zt_fs_outcome_bytes_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    if (fseek(file, 0, SEEK_END) != 0) {
        fclose(file);
        free(path_data);
        return zt_fs_outcome_bytes_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    size_long = ftell(file);
    if (size_long < 0) {
        fclose(file);
        free(path_data);
        return zt_fs_outcome_bytes_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    if (fseek(file, 0, SEEK_SET) != 0) {
        fclose(file);
        free(path_data);
        return zt_fs_outcome_bytes_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    size = (size_t)size_long;
    buffer = size > 0 ? (uint8_t *)malloc(size) : NULL;
    if (size > 0 && buffer == NULL) {
        fclose(file);
        free(path_data);
        return zt_fs_outcome_bytes_failure_error(zt_fs_core_error_from_code_message("fs.io", "failed to allocate file buffer"));
    }

    read_count = size > 0 ? fread(buffer, 1, size, file) : 0;
    if (read_count != size) {
        free(buffer);
        fclose(file);
        free(path_data);
        return zt_fs_outcome_bytes_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    if (fclose(file) != 0) {
        free(buffer);
        free(path_data);
        return zt_fs_outcome_bytes_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    value = zt_bytes_from_array(buffer, size);
    free(buffer);
    free(path_data);
    outcome = zt_outcome_bytes_core_error_success(value);
    zt_release(value);
    return outcome;
}

static zt_outcome_void_core_error zt_host_default_fs_write_bytes(const zt_text *path, const zt_bytes *value) {
    char *path_data;
    FILE *file;
    size_t write_count;
    zt_core_error path_error;

    path_data = zt_host_prepare_path_copy(path, "zt_host_fs_write_bytes_core requires path", &path_error);
    if (path_data == NULL) {
        return zt_fs_outcome_void_failure_error(path_error);
    }
    zt_runtime_require_bytes(value, "zt_host_fs_write_bytes_core requires value");

    file = fopen(path_data, "wb");
    if (file == NULL) {
        free(path_data);
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    if (value->len > 0) {
        write_count = fwrite(value->data, 1, value->len, file);
        if (write_count != value->len) {
            fclose(file);
            free(path_data);
            return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
        }
    }

    if (fclose(file) != 0) {
        free(path_data);
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    free(path_data);
    return zt_outcome_void_core_error_success();
}

static zt_bool zt_host_default_path_exists(const zt_text *path) {
    struct stat info;
    char *path_data;
    zt_core_error path_error;
    zt_bool exists;

    path_data = zt_host_prepare_path_copy(path, "zt_host_path_exists requires path", &path_error);
    if (path_data == NULL) {
        zt_core_error_dispose(&path_error);
        return false;
    }

    exists = stat(path_data, &info) == 0;
    free(path_data);
    return exists;
}

static zt_outcome_void_core_error zt_host_default_fs_append_text(const zt_text *path, const zt_text *value) {
    char *path_data;
    FILE *file;
    size_t write_count;
    zt_core_error path_error;

    path_data = zt_host_prepare_path_copy(path, "zt_host_fs_append_text_core requires path", &path_error);
    if (path_data == NULL) {
        return zt_fs_outcome_void_failure_error(path_error);
    }
    zt_runtime_require_text(value, "zt_host_fs_append_text_core requires value");

    file = fopen(path_data, "ab");
    if (file == NULL) {
        free(path_data);
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    if (value->len > 0) {
        write_count = fwrite(value->data, 1, value->len, file);
        if (write_count != value->len) {
            fclose(file);
            free(path_data);
            return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
        }
    }

    if (fclose(file) != 0) {
        free(path_data);
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    free(path_data);
    return zt_outcome_void_core_error_success();
}

static zt_outcome_bool_core_error zt_host_default_fs_is_file(const zt_text *path) {
    char *path_data;
    struct stat info;
    zt_core_error path_error;

    path_data = zt_host_prepare_path_copy(path, "zt_host_fs_is_file_core requires path", &path_error);
    if (path_data == NULL) {
        return zt_fs_outcome_bool_failure_error(path_error);
    }

    if (stat(path_data, &info) != 0) {
        free(path_data);
        return zt_fs_outcome_bool_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    free(path_data);
    return zt_outcome_bool_core_error_success(S_ISREG(info.st_mode));
}

static zt_outcome_bool_core_error zt_host_default_fs_is_dir(const zt_text *path) {
    char *path_data;
    struct stat info;
    zt_core_error path_error;

    path_data = zt_host_prepare_path_copy(path, "zt_host_fs_is_dir_core requires path", &path_error);
    if (path_data == NULL) {
        return zt_fs_outcome_bool_failure_error(path_error);
    }

    if (stat(path_data, &info) != 0) {
        free(path_data);
        return zt_fs_outcome_bool_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    free(path_data);
    return zt_outcome_bool_core_error_success(S_ISDIR(info.st_mode));
}

static zt_outcome_void_core_error zt_host_default_fs_create_dir(const zt_text *path) {
    char *path_data;
    zt_core_error path_error;
    zt_outcome_void_core_error outcome;

    path_data = zt_host_prepare_path_copy(path, "zt_host_fs_create_dir_core requires path", &path_error);
    if (path_data == NULL) {
        return zt_fs_outcome_void_failure_error(path_error);
    }
    outcome = zt_fs_create_dir_path(path_data);
    free(path_data);
    return outcome;
}

static zt_outcome_void_core_error zt_host_default_fs_create_dir_all(const zt_text *path) {
    char *path_data;
    zt_core_error path_error;
    zt_outcome_void_core_error outcome;

    path_data = zt_host_prepare_path_copy(path, "zt_host_fs_create_dir_all_core requires path", &path_error);
    if (path_data == NULL) {
        return zt_fs_outcome_void_failure_error(path_error);
    }
    outcome = zt_fs_create_dir_all_path(path_data);
    free(path_data);
    return outcome;
}

static zt_outcome_list_text_core_error zt_host_default_fs_list(const zt_text *path) {
    char *path_data;
    struct stat info;
    zt_list_text *items;
    zt_core_error path_error;

    path_data = zt_host_prepare_path_copy(path, "zt_host_fs_list_core requires path", &path_error);
    if (path_data == NULL) {
        return zt_fs_outcome_list_text_failure_error(path_error);
    }

    if (stat(path_data, &info) != 0) {
        free(path_data);
        return zt_fs_outcome_list_text_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }
    if (!S_ISDIR(info.st_mode)) {
        free(path_data);
        return zt_fs_outcome_list_text_failure_error(zt_fs_core_error_from_code_message("fs.not_a_directory", "path is not a directory"));
    }

    items = zt_list_text_new();
#ifdef _WIN32
    {
        char *pattern = zt_fs_join_path(path_data, "*");
        WIN32_FIND_DATAA entry;
        HANDLE handle;

        if (pattern == NULL) {
            zt_release(items);
            free(path_data);
            return zt_outcome_list_text_core_error_failure_message("failed to allocate directory listing path");
        }

        handle = FindFirstFileA(pattern, &entry);
        free(pattern);
        if (handle == INVALID_HANDLE_VALUE) {
            zt_release(items);
            free(path_data);
            return zt_fs_outcome_list_text_failure_error(zt_fs_core_error_from_windows(GetLastError(), NULL));
        }

        do {
            zt_text *item;
            const char *name = entry.cFileName;
            if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0) {
                continue;
            }
            item = zt_text_from_utf8_literal(name);
            zt_list_text_push(items, item);
            zt_release(item);
        } while (FindNextFileA(handle, &entry) != 0);

        if (GetLastError() != ERROR_NO_MORE_FILES) {
            DWORD error_code = GetLastError();
            FindClose(handle);
            zt_release(items);
            free(path_data);
            return zt_fs_outcome_list_text_failure_error(zt_fs_core_error_from_windows(error_code, NULL));
        }

        FindClose(handle);
        free(path_data);
    }
#else
    {
        DIR *dir = opendir(path_data);
        struct dirent *entry;

        if (dir == NULL) {
            zt_release(items);
            free(path_data);
            return zt_fs_outcome_list_text_failure_error(zt_fs_core_error_from_errno(errno, NULL));
        }

        while ((entry = readdir(dir)) != NULL) {
            zt_text *item;
            const char *name = entry->d_name;
            if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0) {
                continue;
            }
            item = zt_text_from_utf8_literal(name);
            zt_list_text_push(items, item);
            zt_release(item);
        }

        if (closedir(dir) != 0) {
            zt_release(items);
            free(path_data);
            return zt_fs_outcome_list_text_failure_error(zt_fs_core_error_from_errno(errno, NULL));
        }

        free(path_data);
    }
#endif

    {
        zt_outcome_list_text_core_error outcome = zt_outcome_list_text_core_error_success(items);
        zt_release(items);
        return outcome;
    }
}

static zt_outcome_void_core_error zt_host_default_fs_walk_dir_into(const char *root, zt_list_text *items) {
    struct stat info;

    if (stat(root, &info) != 0) {
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }
    if (!S_ISDIR(info.st_mode)) {
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_code_message("fs.not_a_directory", "path is not a directory"));
    }

#ifdef _WIN32
    {
        char *pattern = zt_fs_join_path(root, "*");
        WIN32_FIND_DATAA entry;
        HANDLE handle;

        if (pattern == NULL) {
            return zt_outcome_void_core_error_failure_message("failed to allocate directory walk path");
        }

        handle = FindFirstFileA(pattern, &entry);
        free(pattern);
        if (handle == INVALID_HANDLE_VALUE) {
            return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_windows(GetLastError(), NULL));
        }

        do {
            const char *name = entry.cFileName;
            char *child_path;
            zt_text *item;

            if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0) {
                continue;
            }

            child_path = zt_fs_join_path(root, name);
            if (child_path == NULL) {
                FindClose(handle);
                return zt_outcome_void_core_error_failure_message("failed to allocate directory walk child path");
            }

            item = zt_text_from_utf8_literal(child_path);
            zt_list_text_push(items, item);
            zt_release(item);

            if ((entry.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) != 0) {
                zt_outcome_void_core_error child = zt_host_default_fs_walk_dir_into(child_path, items);
                if (!child.is_success) {
                    free(child_path);
                    FindClose(handle);
                    return child;
                }
            }

            free(child_path);
        } while (FindNextFileA(handle, &entry) != 0);

        if (GetLastError() != ERROR_NO_MORE_FILES) {
            DWORD error_code = GetLastError();
            FindClose(handle);
            return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_windows(error_code, NULL));
        }

        FindClose(handle);
    }
#else
    {
        DIR *dir = opendir(root);
        struct dirent *entry;

        if (dir == NULL) {
            return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
        }

        while ((entry = readdir(dir)) != NULL) {
            const char *name = entry->d_name;
            char *child_path;
            zt_text *item;
            struct stat child_info;

            if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0) {
                continue;
            }

            child_path = zt_fs_join_path(root, name);
            if (child_path == NULL) {
                closedir(dir);
                return zt_outcome_void_core_error_failure_message("failed to allocate directory walk child path");
            }

            item = zt_text_from_utf8_literal(child_path);
            zt_list_text_push(items, item);
            zt_release(item);

            if (stat(child_path, &child_info) == 0 && S_ISDIR(child_info.st_mode)) {
                zt_outcome_void_core_error child = zt_host_default_fs_walk_dir_into(child_path, items);
                if (!child.is_success) {
                    free(child_path);
                    closedir(dir);
                    return child;
                }
            }

            free(child_path);
        }

        if (closedir(dir) != 0) {
            return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
        }
    }
#endif

    return zt_outcome_void_core_error_success();
}

static zt_outcome_list_text_core_error zt_host_default_fs_walk_dir(const zt_text *path) {
    char *path_data;
    zt_list_text *items;
    zt_outcome_void_core_error walk_result;
    zt_core_error path_error;
    zt_outcome_list_text_core_error outcome;

    path_data = zt_host_prepare_path_copy(path, "zt_host_fs_walk_dir_core requires path", &path_error);
    if (path_data == NULL) {
        return zt_fs_outcome_list_text_failure_error(path_error);
    }

    items = zt_list_text_new();
    walk_result = zt_host_default_fs_walk_dir_into(path_data, items);
    free(path_data);

    if (!walk_result.is_success) {
        zt_release(items);
        return zt_fs_outcome_list_text_failure_error(walk_result.error);
    }

    outcome = zt_outcome_list_text_core_error_success(items);
    zt_release(items);
    return outcome;
}

static zt_outcome_void_core_error zt_host_default_fs_remove_file(const zt_text *path) {
    char *path_data;
    zt_core_error path_error;

    path_data = zt_host_prepare_path_copy(path, "zt_host_fs_remove_file_core requires path", &path_error);
    if (path_data == NULL) {
        return zt_fs_outcome_void_failure_error(path_error);
    }

    if (remove(path_data) != 0) {
        free(path_data);
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }
    free(path_data);
    return zt_outcome_void_core_error_success();
}

static zt_outcome_void_core_error zt_host_default_fs_remove_dir(const zt_text *path) {
    char *path_data;
    zt_core_error path_error;

    path_data = zt_host_prepare_path_copy(path, "zt_host_fs_remove_dir_core requires path", &path_error);
    if (path_data == NULL) {
        return zt_fs_outcome_void_failure_error(path_error);
    }

#ifdef _WIN32
    if (_rmdir(path_data) != 0) {
#else
    if (rmdir(path_data) != 0) {
#endif
        free(path_data);
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }
    free(path_data);
    return zt_outcome_void_core_error_success();
}

static zt_outcome_void_core_error zt_fs_remove_dir_all_path(const char *path_data) {
    struct stat info;

    if (stat(path_data, &info) != 0) {
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }
    if (!S_ISDIR(info.st_mode)) {
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_code_message("fs.not_a_directory", "path is not a directory"));
    }

#ifdef _WIN32
    {
        char *pattern = zt_fs_join_path(path_data, "*");
        WIN32_FIND_DATAA entry;
        HANDLE handle;

        if (pattern == NULL) {
            return zt_outcome_void_core_error_failure_message("failed to allocate recursive directory pattern");
        }

        handle = FindFirstFileA(pattern, &entry);
        free(pattern);
        if (handle == INVALID_HANDLE_VALUE) {
            return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_windows(GetLastError(), NULL));
        }

        do {
            const char *name = entry.cFileName;
            char *child_path;
            zt_outcome_void_core_error outcome;

            if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0) {
                continue;
            }

            child_path = zt_fs_join_path(path_data, name);
            if (child_path == NULL) {
                FindClose(handle);
                return zt_outcome_void_core_error_failure_message("failed to allocate recursive child path");
            }

            if ((entry.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) != 0) {
                outcome = zt_fs_remove_dir_all_path(child_path);
            } else {
                if (DeleteFileA(child_path) == 0) {
                    outcome = zt_fs_outcome_void_failure_error(zt_fs_core_error_from_windows(GetLastError(), NULL));
                } else {
                    outcome = zt_outcome_void_core_error_success();
                }
            }

            free(child_path);
            if (!outcome.is_success) {
                FindClose(handle);
                return outcome;
            }
        } while (FindNextFileA(handle, &entry) != 0);

        if (GetLastError() != ERROR_NO_MORE_FILES) {
            DWORD error_code = GetLastError();
            FindClose(handle);
            return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_windows(error_code, NULL));
        }

        FindClose(handle);
        if (_rmdir(path_data) != 0) {
            return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
        }
    }
#else
    {
        DIR *dir = opendir(path_data);
        struct dirent *entry;

        if (dir == NULL) {
            return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
        }

        while ((entry = readdir(dir)) != NULL) {
            const char *name = entry->d_name;
            char *child_path;
            struct stat child_info;
            zt_outcome_void_core_error outcome;

            if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0) {
                continue;
            }

            child_path = zt_fs_join_path(path_data, name);
            if (child_path == NULL) {
                closedir(dir);
                return zt_outcome_void_core_error_failure_message("failed to allocate recursive child path");
            }

            if (stat(child_path, &child_info) != 0) {
                free(child_path);
                closedir(dir);
                return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
            }

            if (S_ISDIR(child_info.st_mode)) {
                outcome = zt_fs_remove_dir_all_path(child_path);
            } else {
                if (remove(child_path) != 0) {
                    outcome = zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
                } else {
                    outcome = zt_outcome_void_core_error_success();
                }
            }

            free(child_path);
            if (!outcome.is_success) {
                closedir(dir);
                return outcome;
            }
        }

        if (closedir(dir) != 0) {
            return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
        }
        if (rmdir(path_data) != 0) {
            return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
        }
    }
#endif

    return zt_outcome_void_core_error_success();
}

static zt_outcome_void_core_error zt_host_default_fs_remove_dir_all(const zt_text *path) {
    char *path_data;
    zt_core_error path_error;
    zt_outcome_void_core_error outcome;

    path_data = zt_host_prepare_path_copy(path, "zt_host_fs_remove_dir_all_core requires path", &path_error);
    if (path_data == NULL) {
        return zt_fs_outcome_void_failure_error(path_error);
    }
    outcome = zt_fs_remove_dir_all_path(path_data);
    free(path_data);
    return outcome;
}

static zt_outcome_void_core_error zt_host_default_fs_copy_file(const zt_text *from_path, const zt_text *to_path) {
    char *from_data;
    char *to_data;
    FILE *from_file;
    FILE *to_file;
    unsigned char buffer[8192];
    zt_core_error from_error;
    zt_core_error to_error;

    from_data = zt_host_prepare_path_copy(from_path, "zt_host_fs_copy_file_core requires from_path", &from_error);
    if (from_data == NULL) {
        return zt_fs_outcome_void_failure_error(from_error);
    }
    to_data = zt_host_prepare_path_copy(to_path, "zt_host_fs_copy_file_core requires to_path", &to_error);
    if (to_data == NULL) {
        free(from_data);
        return zt_fs_outcome_void_failure_error(to_error);
    }

    from_file = fopen(from_data, "rb");
    if (from_file == NULL) {
        free(from_data);
        free(to_data);
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    to_file = fopen(to_data, "wb");
    if (to_file == NULL) {
        fclose(from_file);
        free(from_data);
        free(to_data);
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    while (!feof(from_file)) {
        size_t read_count = fread(buffer, 1, sizeof(buffer), from_file);
        if (read_count > 0) {
            size_t written = fwrite(buffer, 1, read_count, to_file);
            if (written != read_count) {
                fclose(from_file);
                fclose(to_file);
                remove(to_data);
                free(from_data);
                free(to_data);
                return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
            }
        }
        if (ferror(from_file)) {
            fclose(from_file);
            fclose(to_file);
            remove(to_data);
            free(from_data);
            free(to_data);
            return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
        }
    }

    if (fclose(from_file) != 0) {
        fclose(to_file);
        remove(to_data);
        free(from_data);
        free(to_data);
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }
    if (fclose(to_file) != 0) {
        remove(to_data);
        free(from_data);
        free(to_data);
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    free(from_data);
    free(to_data);
    return zt_outcome_void_core_error_success();
}

static zt_outcome_void_core_error zt_host_default_fs_move(const zt_text *from_path, const zt_text *to_path) {
    char *from_data;
    char *to_data;
    zt_core_error from_error;
    zt_core_error to_error;

    from_data = zt_host_prepare_path_copy(from_path, "zt_host_fs_move_core requires from_path", &from_error);
    if (from_data == NULL) {
        return zt_fs_outcome_void_failure_error(from_error);
    }
    to_data = zt_host_prepare_path_copy(to_path, "zt_host_fs_move_core requires to_path", &to_error);
    if (to_data == NULL) {
        free(from_data);
        return zt_fs_outcome_void_failure_error(to_error);
    }

    if (rename(from_data, to_data) != 0) {
        free(from_data);
        free(to_data);
        return zt_fs_outcome_void_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }
    free(from_data);
    free(to_data);
    return zt_outcome_void_core_error_success();
}

static zt_outcome_i64_core_error zt_host_default_fs_size(const zt_text *path) {
    char *path_data;
    struct stat info;
    zt_core_error path_error;

    path_data = zt_host_prepare_path_copy(path, "zt_host_fs_size_core requires path", &path_error);
    if (path_data == NULL) {
        return zt_fs_outcome_i64_failure_error(path_error);
    }

    if (stat(path_data, &info) != 0) {
        free(path_data);
        return zt_fs_outcome_i64_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    free(path_data);
    return zt_outcome_i64_core_error_success((zt_int)info.st_size);
}

static zt_outcome_i64_core_error zt_host_default_fs_modified_at(const zt_text *path) {
    char *path_data;
    struct stat info;
    long long millis;
    zt_core_error path_error;

    path_data = zt_host_prepare_path_copy(path, "zt_host_fs_modified_at_core requires path", &path_error);
    if (path_data == NULL) {
        return zt_fs_outcome_i64_failure_error(path_error);
    }

    if (stat(path_data, &info) != 0) {
        free(path_data);
        return zt_fs_outcome_i64_failure_error(zt_fs_core_error_from_errno(errno, NULL));
    }

    millis = (long long)info.st_mtime * 1000LL;
    free(path_data);
    return zt_outcome_i64_core_error_success((zt_int)millis);
}

static zt_outcome_optional_i64_core_error zt_host_default_fs_created_at(const zt_text *path) {
    char *path_data;
    zt_core_error path_error;

    path_data = zt_host_prepare_path_copy(path, "zt_host_fs_created_at_core requires path", &path_error);
    if (path_data == NULL) {
        return zt_fs_outcome_optional_i64_failure_error(path_error);
    }

#ifdef _WIN32
    {
        WIN32_FILE_ATTRIBUTE_DATA data;
        ULARGE_INTEGER ticks;
        unsigned long long windows_ticks;
        unsigned long long unix_ticks;

        if (GetFileAttributesExA(path_data, GetFileExInfoStandard, &data) == 0) {
            free(path_data);
            return zt_fs_outcome_optional_i64_failure_error(zt_fs_core_error_from_windows(GetLastError(), NULL));
        }

        ticks.LowPart = data.ftCreationTime.dwLowDateTime;
        ticks.HighPart = data.ftCreationTime.dwHighDateTime;
        windows_ticks = ticks.QuadPart;
        unix_ticks = windows_ticks - UINT64_C(116444736000000000);
        free(path_data);
        return zt_outcome_optional_i64_core_error_success(zt_optional_i64_present((zt_int)(unix_ticks / 10000ULL)));
    }
#else
    {
        struct stat info;

        if (stat(path_data, &info) != 0) {
            free(path_data);
            return zt_fs_outcome_optional_i64_failure_error(zt_fs_core_error_from_errno(errno, NULL));
        }
        free(path_data);
        return zt_outcome_optional_i64_core_error_success(zt_optional_i64_empty());
    }
#endif
}

static zt_outcome_optional_text_core_error zt_host_default_read_line_stdin(void) {
    zt_outcome_optional_text_core_error outcome;
    size_t capacity = 128;
    size_t len = 0;
    char *buffer = (char *)malloc(capacity);
    int ch;

    if (buffer == NULL) {
        return zt_outcome_optional_text_core_error_failure_message("failed to allocate stdin line buffer");
    }

    while ((ch = fgetc(stdin)) != EOF) {
        if (ch == '\r') {
            int maybe_lf = fgetc(stdin);
            if (maybe_lf != '\n' && maybe_lf != EOF) {
                ungetc(maybe_lf, stdin);
            }
            break;
        }
        if (ch == '\n') {
            break;
        }

        if (len + 1 >= capacity) {
            size_t new_capacity = capacity * 2;
            char *new_buffer = (char *)realloc(buffer, new_capacity);
            if (new_buffer == NULL) {
                free(buffer);
                return zt_outcome_optional_text_core_error_failure_message("failed to grow stdin line buffer");
            }
            buffer = new_buffer;
            capacity = new_capacity;
        }

        buffer[len++] = (char)ch;
    }

    if (ferror(stdin)) {
        free(buffer);
        clearerr(stdin);
        return zt_outcome_optional_text_core_error_failure_message(strerror(errno));
    }

    if (ch == EOF && len == 0) {
        free(buffer);
        return zt_outcome_optional_text_core_error_success(zt_optional_text_empty());
    }

    {
        zt_text *line;
        zt_optional_text value;

        if (!zt_utf8_validate((const uint8_t *)buffer, len, NULL, NULL)) {
            free(buffer);
            return zt_outcome_optional_text_core_error_failure_message("stdin line is not valid UTF-8");
        }
        line = zt_text_from_utf8_unchecked(buffer, len);
        free(buffer);

        value = zt_optional_text_present(line);
        zt_release(line);

        outcome = zt_outcome_optional_text_core_error_success(value);
        return outcome;
    }
}

static zt_outcome_text_core_error zt_host_default_read_all_stdin(void) {
    size_t capacity = 256;
    size_t len = 0;
    char *buffer = (char *)malloc(capacity);
    int ch;
    zt_text *value;
    zt_outcome_text_core_error outcome;

    if (buffer == NULL) {
        return zt_outcome_text_core_error_failure_message("failed to allocate stdin buffer");
    }

    while ((ch = fgetc(stdin)) != EOF) {
        if (len + 1 >= capacity) {
            size_t new_capacity = capacity * 2;
            char *new_buffer = (char *)realloc(buffer, new_capacity);
            if (new_buffer == NULL) {
                free(buffer);
                return zt_outcome_text_core_error_failure_message("failed to grow stdin buffer");
            }
            buffer = new_buffer;
            capacity = new_capacity;
        }
        buffer[len++] = (char)ch;
    }

    if (ferror(stdin)) {
        free(buffer);
        clearerr(stdin);
        return zt_outcome_text_core_error_failure_message(strerror(errno));
    }

    if (!zt_utf8_validate((const uint8_t *)buffer, len, NULL, NULL)) {
        free(buffer);
        return zt_outcome_text_core_error_failure_message("stdin content is not valid UTF-8");
    }
    value = zt_text_from_utf8_unchecked(buffer, len);
    free(buffer);
    outcome = zt_outcome_text_core_error_success(value);
    zt_release(value);
    return outcome;
}
static zt_outcome_void_core_error zt_host_default_write_stream(FILE *stream, const zt_text *value, const char *label) {
    size_t write_count;

    zt_runtime_require_text(value, label);

    if (value->len > 0) {
        write_count = fwrite(value->data, 1, value->len, stream);
        if (write_count != value->len) {
            return zt_outcome_void_core_error_failure_message(strerror(errno));
        }
    }

    if (fflush(stream) != 0) {
        return zt_outcome_void_core_error_failure_message(strerror(errno));
    }

    return zt_outcome_void_core_error_success();
}

static zt_outcome_void_core_error zt_host_default_write_stdout(const zt_text *value) {
    return zt_host_default_write_stream(stdout, value, "zt_host_write_stdout requires text");
}

static zt_outcome_void_core_error zt_host_default_write_stderr(const zt_text *value) {
    return zt_host_default_write_stream(stderr, value, "zt_host_write_stderr requires text");
}

static zt_outcome_void_core_error zt_console_write_ansi(const char *sequence) {
    zt_text *value = zt_text_from_utf8_literal(sequence);
    zt_outcome_void_core_error outcome = zt_host_write_stdout(value);
    zt_release(value);
    return outcome;
}

static int zt_console_stream_fd(const zt_text *stream) {
    if (zt_text_equals_literal(stream, "stdin")) {
        return 0;
    }
    if (zt_text_equals_literal(stream, "stderr")) {
        return 2;
    }
    return 1;
}

zt_bool zt_host_console_is_terminal(const zt_text *stream) {
    int fd = zt_console_stream_fd(stream);
#ifdef _WIN32
    return (zt_bool)_isatty(fd);
#else
    return (zt_bool)isatty(fd);
#endif
}

static int zt_console_env_positive(const char *name) {
    const char *raw = getenv(name);
    char *end = NULL;
    long value;
    if (raw == NULL || raw[0] == '\0') {
        return 0;
    }
    value = strtol(raw, &end, 10);
    if (end == raw || value <= 0 || value > INT_MAX) {
        return 0;
    }
    return (int)value;
}

zt_int zt_host_console_columns(void) {
#ifdef _WIN32
    CONSOLE_SCREEN_BUFFER_INFO info;
    HANDLE handle = GetStdHandle(STD_OUTPUT_HANDLE);
    if (handle != INVALID_HANDLE_VALUE && GetConsoleScreenBufferInfo(handle, &info)) {
        return (zt_int)(info.srWindow.Right - info.srWindow.Left + 1);
    }
#else
    struct winsize ws;
    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == 0 && ws.ws_col > 0) {
        return (zt_int)ws.ws_col;
    }
#endif
    return (zt_int)zt_console_env_positive("COLUMNS");
}

zt_int zt_host_console_rows(void) {
#ifdef _WIN32
    CONSOLE_SCREEN_BUFFER_INFO info;
    HANDLE handle = GetStdHandle(STD_OUTPUT_HANDLE);
    if (handle != INVALID_HANDLE_VALUE && GetConsoleScreenBufferInfo(handle, &info)) {
        return (zt_int)(info.srWindow.Bottom - info.srWindow.Top + 1);
    }
#else
    struct winsize ws;
    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == 0 && ws.ws_row > 0) {
        return (zt_int)ws.ws_row;
    }
#endif
    return (zt_int)zt_console_env_positive("LINES");
}

zt_outcome_void_core_error zt_host_console_clear(void) {
    return zt_console_write_ansi("\x1b[2J\x1b[H");
}

static const char *zt_console_color_sequence(const zt_text *name) {
    if (zt_text_equals_literal(name, "black")) return "\x1b[30m";
    if (zt_text_equals_literal(name, "red")) return "\x1b[31m";
    if (zt_text_equals_literal(name, "green")) return "\x1b[32m";
    if (zt_text_equals_literal(name, "yellow")) return "\x1b[33m";
    if (zt_text_equals_literal(name, "blue")) return "\x1b[34m";
    if (zt_text_equals_literal(name, "magenta")) return "\x1b[35m";
    if (zt_text_equals_literal(name, "cyan")) return "\x1b[36m";
    if (zt_text_equals_literal(name, "white")) return "\x1b[37m";
    if (zt_text_equals_literal(name, "bright_black")) return "\x1b[90m";
    if (zt_text_equals_literal(name, "bright_red")) return "\x1b[91m";
    if (zt_text_equals_literal(name, "bright_green")) return "\x1b[92m";
    if (zt_text_equals_literal(name, "bright_yellow")) return "\x1b[93m";
    if (zt_text_equals_literal(name, "bright_blue")) return "\x1b[94m";
    if (zt_text_equals_literal(name, "bright_magenta")) return "\x1b[95m";
    if (zt_text_equals_literal(name, "bright_cyan")) return "\x1b[96m";
    if (zt_text_equals_literal(name, "bright_white")) return "\x1b[97m";
    return "\x1b[39m";
}

zt_outcome_void_core_error zt_host_console_set_color(const zt_text *name) {
    return zt_console_write_ansi(zt_console_color_sequence(name));
}

static const char *zt_console_style_sequence(const zt_text *name) {
    if (zt_text_equals_literal(name, "bold")) return "\x1b[1m";
    if (zt_text_equals_literal(name, "dim")) return "\x1b[2m";
    if (zt_text_equals_literal(name, "italic")) return "\x1b[3m";
    if (zt_text_equals_literal(name, "underline")) return "\x1b[4m";
    if (zt_text_equals_literal(name, "reverse")) return "\x1b[7m";
    return "\x1b[0m";
}

zt_outcome_void_core_error zt_host_console_set_style(const zt_text *name) {
    return zt_console_write_ansi(zt_console_style_sequence(name));
}

zt_outcome_void_core_error zt_host_console_reset_style(void) {
    return zt_console_write_ansi("\x1b[0m");
}

zt_outcome_optional_text_core_error zt_host_console_read_key(void) {
    unsigned char ch;
    char buffer[2];

#ifdef _WIN32
    if (!_isatty(0) || !_kbhit()) {
        return zt_outcome_optional_text_core_error_success(zt_optional_text_empty());
    }
    ch = (unsigned char)_getch();
    if (ch == 0 || ch == 224) {
        (void)_getch();
        return zt_outcome_optional_text_core_error_success(zt_optional_text_empty());
    }
#else
    struct termios old_term;
    struct termios raw_term;
    int old_flags;
    int read_count;

    if (!isatty(STDIN_FILENO)) {
        return zt_outcome_optional_text_core_error_success(zt_optional_text_empty());
    }
    if (tcgetattr(STDIN_FILENO, &old_term) != 0) {
        return zt_outcome_optional_text_core_error_failure_message(strerror(errno));
    }
    raw_term = old_term;
    raw_term.c_lflag &= (tcflag_t)~(ICANON | ECHO);
    raw_term.c_cc[VMIN] = 0;
    raw_term.c_cc[VTIME] = 0;
    if (tcsetattr(STDIN_FILENO, TCSANOW, &raw_term) != 0) {
        return zt_outcome_optional_text_core_error_failure_message(strerror(errno));
    }
    old_flags = fcntl(STDIN_FILENO, F_GETFL, 0);
    if (old_flags >= 0) {
        (void)fcntl(STDIN_FILENO, F_SETFL, old_flags | O_NONBLOCK);
    }
    read_count = (int)read(STDIN_FILENO, &ch, 1);
    if (old_flags >= 0) {
        (void)fcntl(STDIN_FILENO, F_SETFL, old_flags);
    }
    (void)tcsetattr(STDIN_FILENO, TCSANOW, &old_term);
    if (read_count <= 0) {
        return zt_outcome_optional_text_core_error_success(zt_optional_text_empty());
    }
#endif

    buffer[0] = (char)ch;
    buffer[1] = '\0';
    if (!zt_utf8_validate((const uint8_t *)buffer, 1, NULL, NULL)) {
        return zt_outcome_optional_text_core_error_failure_message("console key is not valid UTF-8");
    }
    {
        zt_text *key = zt_text_from_utf8_unchecked(buffer, 1);
        zt_optional_text value = zt_optional_text_present(key);
        zt_outcome_optional_text_core_error outcome;
        zt_release(key);
        outcome = zt_outcome_optional_text_core_error_success(value);
        return outcome;
    }
}

static zt_int zt_host_default_time_now_unix_ms(void) {
    struct timespec ts;
    long long millis;

    if (timespec_get(&ts, TIME_UTC) == 0) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to read system time");
    }

    millis = (long long)ts.tv_sec * 1000LL + (long long)(ts.tv_nsec / 1000000L);
    return (zt_int)millis;
}

static zt_outcome_void_core_error zt_host_default_time_sleep_ms(zt_int duration_ms) {
    if (duration_ms < 0) {
        return zt_outcome_void_core_error_failure_message("time.sleep requires non-negative duration");
    }

#ifdef _WIN32
    Sleep((DWORD)duration_ms);
    return zt_outcome_void_core_error_success();
#else
    struct timespec req;
    struct timespec rem;

    req.tv_sec = (time_t)(duration_ms / 1000);
    req.tv_nsec = (long)((duration_ms % 1000) * 1000000L);

    while (nanosleep(&req, &rem) != 0) {
        if (errno != EINTR) {
            return zt_outcome_void_core_error_failure_message(strerror(errno));
        }
        req = rem;
    }

    return zt_outcome_void_core_error_success();
#endif
}

static uint64_t zt_host_random_state = UINT64_C(0x9e3779b97f4a7c15);

static uint64_t zt_host_random_next_u64(void) {
    uint64_t x = zt_host_random_state;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    zt_host_random_state = x;
    return x * UINT64_C(2685821657736338717);
}

static void zt_host_default_random_seed(zt_int seed) {
    uint64_t state = (uint64_t)seed;
    if (state == 0) {
        state = UINT64_C(0x9e3779b97f4a7c15);
    }
    zt_host_random_state = state;
    (void)zt_host_random_next_u64();
}

static zt_int zt_host_default_random_next_i64(void) {
    uint64_t value = zt_host_random_next_u64();
    return (zt_int)(value & UINT64_C(0x7fffffffffffffff));
}

static char *zt_host_strdup_text(const zt_text *value, const char *label) {
    char *copy;
    size_t byte_count;
    if (value == NULL) {
        zt_runtime_error(ZT_ERR_PANIC, label);
    }
    byte_count = zt_require_added_size(value->len, 1, "host string size overflow");
    copy = (char *)malloc(byte_count);
    if (copy == NULL) {
        zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate host string");
    }
    memcpy(copy, value->data, value->len);
    copy[value->len] = '\0';
    return copy;
}

static zt_bool zt_host_path_char_eq(char left, char right) {
#ifdef _WIN32
    return (char)tolower((unsigned char)left) == (char)tolower((unsigned char)right);
#else
    return left == right;
#endif
}

static zt_bool zt_host_path_is_within_root(const zt_text *path, const zt_text *root) {
    size_t index;

    if (path == NULL || root == NULL) return false;
    if (root->len == 0) return false;

    if (root->len == 1 && root->data[0] == '/') {
        return path->len > 0 && path->data[0] == '/';
    }

    if (path->len < root->len) return false;
    for (index = 0; index < root->len; index += 1) {
        if (!zt_host_path_char_eq(path->data[index], root->data[index])) {
            return false;
        }
    }

    if (path->len == root->len) return true;
    if (root->data[root->len - 1] == '/') return true;
    return path->data[root->len] == '/';
}

static char *zt_host_prepare_path_copy(const zt_text *path, const char *label, zt_core_error *out_error) {
    zt_outcome_text_core_error cwd_now;
    zt_text *normalized = NULL;
    zt_text *absolute = NULL;
    zt_text *root_text = NULL;
    zt_text *root_absolute = NULL;
    const char *root_env;
    char *copy = NULL;

    if (out_error == NULL) return NULL;
    memset(out_error, 0, sizeof(*out_error));

    zt_runtime_require_text(path, label);

    if (memchr(path->data, '\0', path->len) != NULL) {
        *out_error = zt_fs_core_error_from_code_message("fs.invalid_path", "path contains NUL byte");
        return NULL;
    }

    cwd_now = zt_host_default_os_current_dir();
    if (!cwd_now.is_success) {
        *out_error = zt_core_error_clone(cwd_now.error);
        zt_core_error_dispose(&cwd_now.error);
        return NULL;
    }

    normalized = zt_path_normalize(path);
    absolute = zt_path_absolute(normalized, cwd_now.value);

    root_env = getenv("ZENITH_HOST_FS_ROOT");
    if (root_env != NULL && root_env[0] != '\0') {
        root_text = zt_text_from_utf8_literal(root_env);
        root_absolute = zt_path_absolute(root_text, cwd_now.value);
        if (!zt_host_path_is_within_root(absolute, root_absolute)) {
            *out_error = zt_fs_core_error_from_code_message(
                "fs.permission_denied",
                "path escapes configured host fs root");
            goto cleanup;
        }
    }

    copy = zt_host_strdup_text(absolute, label);

cleanup:
    if (root_absolute != NULL) zt_release(root_absolute);
    if (root_text != NULL) zt_release(root_text);
    if (absolute != NULL) zt_release(absolute);
    if (normalized != NULL) zt_release(normalized);
    zt_release(cwd_now.value);
    return copy;
}

static char *zt_host_try_strdup_text(const zt_text *value) {
    char *copy;
    size_t byte_count;

    if (value == NULL) {
        return NULL;
    }

    if (!zt_try_add_size(value->len, 1, &byte_count)) {
        return NULL;
    }

    copy = (char *)malloc(byte_count);
    if (copy == NULL) {
        return NULL;
    }

    memcpy(copy, value->data, value->len);
    copy[value->len] = '\0';
    return copy;
}

static zt_outcome_text_core_error zt_host_default_os_current_dir(void) {
    size_t capacity = 256;
    char *buffer = NULL;

    while (1) {
        char *grown = (char *)realloc(buffer, capacity);
        if (grown == NULL) {
            free(buffer);
            return zt_outcome_text_core_error_failure_message("os.current_dir allocation failed");
        }
        buffer = grown;

#ifdef _WIN32
        if (_getcwd(buffer, (int)capacity) != NULL) {
            break;
        }
#else
        if (getcwd(buffer, capacity) != NULL) {
            break;
        }
#endif

        if (errno != ERANGE) {
            zt_outcome_text_core_error failure = zt_outcome_text_core_error_failure_message(strerror(errno));
            free(buffer);
            return failure;
        }

        if (capacity > (SIZE_MAX / 2)) {
            free(buffer);
            return zt_outcome_text_core_error_failure_message("os.current_dir path too long");
        }
        capacity *= 2;
    }

    {
        zt_text *text = zt_text_from_utf8_literal(buffer);
        zt_outcome_text_core_error outcome = zt_outcome_text_core_error_success(text);
        zt_release(text);
        free(buffer);
        return outcome;
    }
}

static zt_outcome_void_core_error zt_host_default_os_change_dir(const zt_text *path) {
    zt_core_error path_error;
    char *path_copy = zt_host_prepare_path_copy(path, "os.change_dir requires path text", &path_error);
    int rc;

    if (path_copy == NULL) {
        return zt_outcome_void_core_error_failure(path_error);
    }
#ifdef _WIN32
    rc = _chdir(path_copy);
#else
    rc = chdir(path_copy);
#endif
    free(path_copy);
    if (rc != 0) {
        return zt_outcome_void_core_error_failure_message(strerror(errno));
    }
    return zt_outcome_void_core_error_success();
}

static zt_optional_text zt_host_default_os_env(const zt_text *name) {
    zt_optional_text empty = zt_optional_text_empty();
    char *name_copy = zt_host_strdup_text(name, "os.env requires variable name text");
    const char *value = getenv(name_copy);
    free(name_copy);

    if (value == NULL) {
        return empty;
    }

    {
        zt_text *text_value = zt_text_from_utf8_literal(value);
        zt_optional_text result = zt_optional_text_present(text_value);
        zt_release(text_value);
        return result;
    }
}

static zt_int zt_host_default_os_pid(void) {
#ifdef _WIN32
    return (zt_int)_getpid();
#else
    return (zt_int)getpid();
#endif
}

static zt_text *zt_host_default_os_platform(void) {
#if defined(_WIN32)
    return zt_text_from_utf8_literal("windows");
#elif defined(__APPLE__)
    return zt_text_from_utf8_literal("macos");
#elif defined(__linux__)
    return zt_text_from_utf8_literal("linux");
#else
    return zt_text_from_utf8_literal("unknown");
#endif
}

static zt_text *zt_host_default_os_arch(void) {
#if defined(__x86_64__) || defined(_M_X64)
    return zt_text_from_utf8_literal("x64");
#elif defined(__i386__) || defined(_M_IX86)
    return zt_text_from_utf8_literal("x86");
#elif defined(__aarch64__) || defined(_M_ARM64)
    return zt_text_from_utf8_literal("arm64");
#elif defined(__arm__) || defined(_M_ARM)
    return zt_text_from_utf8_literal("arm");
#elif defined(__riscv) && __riscv_xlen == 64
    return zt_text_from_utf8_literal("riscv64");
#else
    return zt_text_from_utf8_literal("unknown");
#endif
}

static int zt_host_stream_fd(FILE *stream) {
    if (stream == NULL) {
        return -1;
    }
#ifdef _WIN32
    return _fileno(stream);
#else
    return fileno(stream);
#endif
}

static int zt_host_dup_fd(int fd) {
#ifdef _WIN32
    return _dup(fd);
#else
    return dup(fd);
#endif
}

static int zt_host_dup2_fd(int source_fd, int target_fd) {
#ifdef _WIN32
    return _dup2(source_fd, target_fd);
#else
    return dup2(source_fd, target_fd);
#endif
}

static void zt_host_close_fd(int fd) {
    if (fd < 0) {
        return;
    }
#ifdef _WIN32
    _close(fd);
#else
    close(fd);
#endif
}

static void zt_process_capture_redirect_init(zt_process_capture_redirect *redirect) {
    if (redirect == NULL) {
        return;
    }
    redirect->saved_stdout_fd = -1;
    redirect->saved_stderr_fd = -1;
    redirect->active = false;
#ifdef _WIN32
    redirect->saved_stdout_handle = NULL;
    redirect->saved_stderr_handle = NULL;
#endif
}

static zt_outcome_void_core_error zt_host_redirect_process_stdio(
        FILE *stdout_stream,
        FILE *stderr_stream,
        zt_process_capture_redirect *redirect) {
    int stdout_fd;
    int stderr_fd;
    int capture_stdout_fd;
    int capture_stderr_fd;

    if (stdout_stream == NULL || stderr_stream == NULL || redirect == NULL) {
        return zt_outcome_void_core_error_failure_message("process.run_capture requires valid capture streams");
    }

    stdout_fd = zt_host_stream_fd(stdout);
    stderr_fd = zt_host_stream_fd(stderr);
    capture_stdout_fd = zt_host_stream_fd(stdout_stream);
    capture_stderr_fd = zt_host_stream_fd(stderr_stream);
    if (stdout_fd < 0 || stderr_fd < 0 || capture_stdout_fd < 0 || capture_stderr_fd < 0) {
        return zt_outcome_void_core_error_failure_message("process.run_capture failed to resolve stdio file descriptors");
    }

    fflush(stdout);
    fflush(stderr);

    zt_process_capture_redirect_init(redirect);
    redirect->saved_stdout_fd = zt_host_dup_fd(stdout_fd);
    if (redirect->saved_stdout_fd < 0) {
        return zt_outcome_void_core_error_failure_message(strerror(errno));
    }

    redirect->saved_stderr_fd = zt_host_dup_fd(stderr_fd);
    if (redirect->saved_stderr_fd < 0) {
        int saved_errno = errno;
        zt_host_close_fd(redirect->saved_stdout_fd);
        redirect->saved_stdout_fd = -1;
        return zt_outcome_void_core_error_failure_message(strerror(saved_errno));
    }

#ifdef _WIN32
    redirect->saved_stdout_handle = GetStdHandle(STD_OUTPUT_HANDLE);
    redirect->saved_stderr_handle = GetStdHandle(STD_ERROR_HANDLE);
#endif

    if (zt_host_dup2_fd(capture_stdout_fd, stdout_fd) < 0) {
        int saved_errno = errno;
        zt_host_close_fd(redirect->saved_stdout_fd);
        zt_host_close_fd(redirect->saved_stderr_fd);
        zt_process_capture_redirect_init(redirect);
        return zt_outcome_void_core_error_failure_message(strerror(saved_errno));
    }

    if (zt_host_dup2_fd(capture_stderr_fd, stderr_fd) < 0) {
        int saved_errno = errno;
        (void)zt_host_dup2_fd(redirect->saved_stdout_fd, stdout_fd);
        zt_host_close_fd(redirect->saved_stdout_fd);
        zt_host_close_fd(redirect->saved_stderr_fd);
        zt_process_capture_redirect_init(redirect);
        return zt_outcome_void_core_error_failure_message(strerror(saved_errno));
    }

    redirect->active = true;

#ifdef _WIN32
    if (!SetStdHandle(STD_OUTPUT_HANDLE, (HANDLE)_get_osfhandle(capture_stdout_fd)) ||
            !SetStdHandle(STD_ERROR_HANDLE, (HANDLE)_get_osfhandle(capture_stderr_fd))) {
        zt_outcome_void_core_error restore_outcome = zt_host_restore_process_stdio(redirect);
        if (!restore_outcome.is_success) {
            return restore_outcome;
        }
        return zt_outcome_void_core_error_failure_message("process.run_capture failed to redirect Windows stdio handles");
    }
#endif

    return zt_outcome_void_core_error_success();
}

static zt_outcome_void_core_error zt_host_restore_process_stdio(zt_process_capture_redirect *redirect) {
    int stdout_fd;
    int stderr_fd;
    int saved_errno = 0;
    zt_bool handle_error = false;

    if (redirect == NULL || !redirect->active) {
        return zt_outcome_void_core_error_success();
    }

    stdout_fd = zt_host_stream_fd(stdout);
    stderr_fd = zt_host_stream_fd(stderr);
    if (stdout_fd < 0 || stderr_fd < 0) {
        saved_errno = errno != 0 ? errno : EINVAL;
    } else {
        fflush(stdout);
        fflush(stderr);
        if (zt_host_dup2_fd(redirect->saved_stdout_fd, stdout_fd) < 0 && saved_errno == 0) {
            saved_errno = errno;
        }
        if (zt_host_dup2_fd(redirect->saved_stderr_fd, stderr_fd) < 0 && saved_errno == 0) {
            saved_errno = errno;
        }
    }

#ifdef _WIN32
    if (redirect->saved_stdout_handle != NULL &&
            !SetStdHandle(STD_OUTPUT_HANDLE, redirect->saved_stdout_handle)) {
        handle_error = true;
    }
    if (redirect->saved_stderr_handle != NULL &&
            !SetStdHandle(STD_ERROR_HANDLE, redirect->saved_stderr_handle)) {
        handle_error = true;
    }
#endif

    zt_host_close_fd(redirect->saved_stdout_fd);
    zt_host_close_fd(redirect->saved_stderr_fd);
    zt_process_capture_redirect_init(redirect);

    if (saved_errno != 0) {
        return zt_outcome_void_core_error_failure_message(strerror(saved_errno));
    }
    if (handle_error) {
        return zt_outcome_void_core_error_failure_message("process.run_capture failed to restore stdio handles");
    }
    return zt_outcome_void_core_error_success();
}

static zt_outcome_text_core_error zt_host_read_stream_text(FILE *stream, const char *empty_label) {
    long size_long;
    size_t size;
    char *buffer;
    size_t read_count = 0;
    zt_text *value;
    zt_outcome_text_core_error outcome;
    size_t error_index = 0;
    const char *error_reason = NULL;

    if (stream == NULL) {
        return zt_outcome_text_core_error_failure_message(empty_label);
    }

    if (fseek(stream, 0, SEEK_END) != 0) {
        return zt_outcome_text_core_error_failure_message(strerror(errno));
    }
    size_long = ftell(stream);
    if (size_long < 0) {
        return zt_outcome_text_core_error_failure_message(strerror(errno));
    }
    if (fseek(stream, 0, SEEK_SET) != 0) {
        return zt_outcome_text_core_error_failure_message(strerror(errno));
    }

    size = (size_t)size_long;
    buffer = (char *)malloc(zt_require_added_size(size, 1, "process capture buffer overflow"));
    if (buffer == NULL) {
        return zt_outcome_text_core_error_failure_message("process.run_capture buffer allocation failed");
    }

    if (size > 0) {
        read_count = fread(buffer, 1, size, stream);
        if (read_count != size) {
            zt_outcome_text_core_error failure = zt_outcome_text_core_error_failure_message(
                ferror(stream) ? strerror(errno) : "failed to read captured process output"
            );
            free(buffer);
            return failure;
        }
    }
    buffer[size] = '\0';

    if (!zt_utf8_validate((const uint8_t *)buffer, size, &error_index, &error_reason)) {
        char decode_message[256];
        zt_core_error error;

        snprintf(
            decode_message,
            sizeof(decode_message),
            "captured process output is not valid UTF-8 at byte %zu (%s)",
            error_index,
            error_reason != NULL ? error_reason : "invalid encoding"
        );
        error = zt_core_error_from_message("process.decode", decode_message);
        outcome = zt_outcome_text_core_error_failure(error);
        zt_core_error_dispose(&error);
        free(buffer);
        return outcome;
    }

    value = zt_text_from_utf8(buffer, size);
    free(buffer);
    outcome = zt_outcome_text_core_error_success(value);
    zt_release(value);
    return outcome;
}

static void zt_process_captured_run_retain(zt_process_captured_run value) {
    if (value.stdout_text != NULL) {
        zt_retain(value.stdout_text);
    }
    if (value.stderr_text != NULL) {
        zt_retain(value.stderr_text);
    }
}

static void zt_process_captured_run_dispose(zt_process_captured_run *value) {
    if (value == NULL) {
        return;
    }
    if (value->stdout_text != NULL) {
        zt_release(value->stdout_text);
        value->stdout_text = NULL;
    }
    if (value->stderr_text != NULL) {
        zt_release(value->stderr_text);
        value->stderr_text = NULL;
    }
    value->status.code = 0;
}

void zt_runtime_capture_process_args(int argc, char **argv) {
    zt_list_text *captured;
    int index;

    if (zt_host_captured_process_args != NULL) {
        zt_release(zt_host_captured_process_args);
        zt_host_captured_process_args = NULL;
    }

    captured = zt_list_text_new();
    for (index = 0; index < argc; index += 1) {
        const char *arg_value = (argv != NULL && argv[index] != NULL) ? argv[index] : "";
        zt_text *arg_text = zt_text_from_utf8_literal(arg_value);
        zt_list_text_push(captured, arg_text);
        zt_release(arg_text);
    }

    zt_host_captured_process_args = captured;
}

static zt_list_text *zt_host_default_os_args(void) {
    zt_list_text *copy = zt_list_text_new();
    size_t index;

    if (zt_host_captured_process_args == NULL) {
        return copy;
    }

    for (index = 0; index < zt_host_captured_process_args->len; index += 1) {
        zt_list_text_push(copy, zt_host_captured_process_args->data[index]);
    }

    return copy;
}

static void zt_host_free_process_argv(char **argv, size_t count) {
    size_t index;

    if (argv == NULL) {
        return;
    }

    for (index = 0; index < count; index += 1) {
        free(argv[index]);
    }

    free(argv);
}

static void zt_host_restore_cwd_ignored(const char *saved_cwd) {
    zt_text *saved;
    zt_outcome_void_core_error restore_ignored;

    if (saved_cwd == NULL) {
        return;
    }

    saved = zt_text_from_utf8_literal(saved_cwd);
    restore_ignored = zt_host_default_os_change_dir(saved);
    if (!restore_ignored.is_success) {
        zt_core_error_dispose(&restore_ignored.error);
    }
    zt_release(saved);
}

static zt_outcome_i64_core_error zt_host_default_process_run(const zt_text *program, const zt_list_text *args, zt_optional_text cwd) {
    char **argv = NULL;
    size_t arg_count = 0;
    size_t argv_capacity = 0;
    size_t copied = 0;
    int exit_code;
    char *saved_cwd = NULL;
    zt_bool cwd_changed = false;

    if (program == NULL || program->len == 0) {
        return zt_outcome_i64_core_error_failure_message("process.run requires non-empty program");
    }

    if (args == NULL) {
        return zt_outcome_i64_core_error_failure_message("process.run requires args list");
    }

    if (!zt_try_add_size(args->len, 1, &arg_count) ||
        !zt_try_add_size(arg_count, 1, &argv_capacity)) {
        return zt_outcome_i64_core_error_failure_message("process.run args too large");
    }

    argv = (char **)calloc(argv_capacity, sizeof(char *));
    if (argv == NULL) {
        return zt_outcome_i64_core_error_failure_message("process.run command allocation failed");
    }

    argv[0] = zt_host_try_strdup_text(program);
    if (argv[0] == NULL) {
        zt_host_free_process_argv(argv, copied);
        return zt_outcome_i64_core_error_failure_message("process.run command allocation failed");
    }
    copied = 1;

    {
        size_t i;
        for (i = 0; i < args->len; i += 1) {
            zt_text *arg = args->data[i];
            if (arg == NULL) {
                zt_host_free_process_argv(argv, copied);
                return zt_outcome_i64_core_error_failure_message("process.run args cannot contain null text");
            }
            argv[copied] = zt_host_try_strdup_text(arg);
            if (argv[copied] == NULL) {
                zt_host_free_process_argv(argv, copied);
                return zt_outcome_i64_core_error_failure_message("process.run command allocation failed");
            }
            copied += 1;
        }
    }

    if (cwd.is_present) {
        zt_outcome_text_core_error cwd_now = zt_host_default_os_current_dir();
        if (!cwd_now.is_success) {
            {
                zt_outcome_i64_core_error fail_outcome = zt_outcome_i64_core_error_failure(cwd_now.error);
                zt_core_error_dispose(&cwd_now.error);
                zt_host_free_process_argv(argv, copied);
                return fail_outcome;
            }
        }
        saved_cwd = zt_host_strdup_text(cwd_now.value, "process.run failed to copy cwd");
        zt_release(cwd_now.value);

        if (cwd.value == NULL) {
            zt_host_free_process_argv(argv, copied);
            free(saved_cwd);
            return zt_outcome_i64_core_error_failure_message("process.run cwd present with null text");
        }

        {
            zt_outcome_void_core_error cd_outcome = zt_host_default_os_change_dir(cwd.value);
            if (!cd_outcome.is_success) {
                zt_host_free_process_argv(argv, copied);
                free(saved_cwd);
                {
                    zt_outcome_i64_core_error fail_outcome = zt_outcome_i64_core_error_failure(cd_outcome.error);
                    zt_core_error_dispose(&cd_outcome.error);
                    return fail_outcome;
                }
            }
            cwd_changed = true;
        }
    }

    #ifdef _WIN32
    {
        intptr_t spawn_status = _spawnvp(_P_WAIT, argv[0], (const char * const *)argv);
        if (spawn_status == -1) {
            int saved_errno = errno;
            if (cwd_changed) {
                zt_host_restore_cwd_ignored(saved_cwd);
            }
            free(saved_cwd);
            zt_host_free_process_argv(argv, copied);
            return zt_outcome_i64_core_error_failure_message(strerror(saved_errno));
        }
        exit_code = (int)spawn_status;
    }
    #else
    {
        int error_pipe[2] = { -1, -1 };
        int status = 0;
        int child_errno = 0;
        ssize_t child_error_size = 0;
        pid_t pid;

        if (pipe(error_pipe) != 0) {
            int saved_errno = errno;
            if (cwd_changed) {
                zt_host_restore_cwd_ignored(saved_cwd);
            }
            free(saved_cwd);
            zt_host_free_process_argv(argv, copied);
            return zt_outcome_i64_core_error_failure_message(strerror(saved_errno));
        }

        (void)fcntl(error_pipe[1], F_SETFD, FD_CLOEXEC);
        pid = fork();
        if (pid < 0) {
            int saved_errno = errno;
            close(error_pipe[0]);
            close(error_pipe[1]);
            if (cwd_changed) {
                zt_host_restore_cwd_ignored(saved_cwd);
            }
            free(saved_cwd);
            zt_host_free_process_argv(argv, copied);
            return zt_outcome_i64_core_error_failure_message(strerror(saved_errno));
        }

        if (pid == 0) {
            close(error_pipe[0]);
            execvp(argv[0], argv);
            child_errno = errno;
            (void)write(error_pipe[1], &child_errno, sizeof(child_errno));
            close(error_pipe[1]);
            _exit(127);
        }

        close(error_pipe[1]);
        if (waitpid(pid, &status, 0) < 0) {
            int saved_errno = errno;
            close(error_pipe[0]);
            if (cwd_changed) {
                zt_host_restore_cwd_ignored(saved_cwd);
            }
            free(saved_cwd);
            zt_host_free_process_argv(argv, copied);
            return zt_outcome_i64_core_error_failure_message(strerror(saved_errno));
        }

        child_error_size = read(error_pipe[0], &child_errno, sizeof(child_errno));
        close(error_pipe[0]);

        if (child_error_size == (ssize_t)sizeof(child_errno)) {
            if (cwd_changed) {
                zt_host_restore_cwd_ignored(saved_cwd);
            }
            free(saved_cwd);
            zt_host_free_process_argv(argv, copied);
            return zt_outcome_i64_core_error_failure_message(strerror(child_errno));
        }

        if (WIFEXITED(status)) {
            exit_code = WEXITSTATUS(status);
        } else if (WIFSIGNALED(status)) {
            exit_code = 128 + WTERMSIG(status);
        } else {
            exit_code = status;
        }
    }
    #endif

    if (cwd_changed) {
        zt_text *saved = zt_text_from_utf8_literal(saved_cwd);
        zt_outcome_void_core_error restore = zt_host_default_os_change_dir(saved);
        zt_release(saved);
        free(saved_cwd);
        if (!restore.is_success) {
            {
                zt_outcome_i64_core_error fail_outcome = zt_outcome_i64_core_error_failure(restore.error);
                zt_core_error_dispose(&restore.error);
                zt_host_free_process_argv(argv, copied);
                return fail_outcome;
            }
        }
    } else {
        free(saved_cwd);
    }

    zt_host_free_process_argv(argv, copied);
    return zt_outcome_i64_core_error_success((zt_int)exit_code);
}

static zt_outcome_process_captured_run_core_error zt_host_default_process_run_capture(
        const zt_text *program,
        const zt_list_text *args,
        zt_optional_text cwd) {
    FILE *stdout_capture = NULL;
    FILE *stderr_capture = NULL;
    zt_process_capture_redirect redirect;
    zt_outcome_void_core_error redirect_outcome;
    zt_outcome_i64_core_error run_outcome;
    zt_outcome_void_core_error restore_outcome;
    zt_outcome_text_core_error stdout_outcome;
    zt_outcome_text_core_error stderr_outcome;
    zt_text *stdout_text = NULL;
    zt_text *stderr_text = NULL;
    zt_process_captured_run captured;
    zt_int exit_code;
    zt_outcome_process_captured_run_core_error result;

    zt_process_capture_redirect_init(&redirect);
    captured.status.code = 0;
    captured.stdout_text = NULL;
    captured.stderr_text = NULL;

    stdout_capture = tmpfile();
    if (stdout_capture == NULL) {
        return zt_outcome_process_captured_run_core_error_failure_message(strerror(errno));
    }

    stderr_capture = tmpfile();
    if (stderr_capture == NULL) {
        int saved_errno = errno;
        fclose(stdout_capture);
        return zt_outcome_process_captured_run_core_error_failure_message(strerror(saved_errno));
    }

    redirect_outcome = zt_host_redirect_process_stdio(stdout_capture, stderr_capture, &redirect);
    if (!redirect_outcome.is_success) {
        result = zt_outcome_process_captured_run_core_error_failure(redirect_outcome.error);
        zt_outcome_void_core_error_dispose(&redirect_outcome);
        fclose(stdout_capture);
        fclose(stderr_capture);
        return result;
    }

    run_outcome = zt_host_default_process_run(program, args, cwd);
    restore_outcome = zt_host_restore_process_stdio(&redirect);
    if (!restore_outcome.is_success) {
        result = zt_outcome_process_captured_run_core_error_failure(restore_outcome.error);
        zt_outcome_void_core_error_dispose(&restore_outcome);
        zt_outcome_i64_core_error_dispose(&run_outcome);
        fclose(stdout_capture);
        fclose(stderr_capture);
        return result;
    }

    if (!run_outcome.is_success) {
        result = zt_outcome_process_captured_run_core_error_failure(run_outcome.error);
        zt_outcome_i64_core_error_dispose(&run_outcome);
        fclose(stdout_capture);
        fclose(stderr_capture);
        return result;
    }

    exit_code = zt_outcome_i64_core_error_value(run_outcome);
    zt_outcome_i64_core_error_dispose(&run_outcome);

    stdout_outcome = zt_host_read_stream_text(stdout_capture, "process.run_capture missing stdout capture");
    if (!stdout_outcome.is_success) {
        result = zt_outcome_process_captured_run_core_error_failure(stdout_outcome.error);
        zt_outcome_text_core_error_dispose(&stdout_outcome);
        fclose(stdout_capture);
        fclose(stderr_capture);
        return result;
    }

    stderr_outcome = zt_host_read_stream_text(stderr_capture, "process.run_capture missing stderr capture");
    if (!stderr_outcome.is_success) {
        result = zt_outcome_process_captured_run_core_error_failure(stderr_outcome.error);
        zt_outcome_text_core_error_dispose(&stdout_outcome);
        zt_outcome_text_core_error_dispose(&stderr_outcome);
        fclose(stdout_capture);
        fclose(stderr_capture);
        return result;
    }

    stdout_text = zt_outcome_text_core_error_value(stdout_outcome);
    stderr_text = zt_outcome_text_core_error_value(stderr_outcome);
    zt_outcome_text_core_error_dispose(&stdout_outcome);
    zt_outcome_text_core_error_dispose(&stderr_outcome);
    fclose(stdout_capture);
    fclose(stderr_capture);

    captured.status.code = exit_code;
    captured.stdout_text = stdout_text;
    captured.stderr_text = stderr_text;
    result = zt_outcome_process_captured_run_core_error_success(captured);
    zt_process_captured_run_dispose(&captured);
    return result;
}

void zt_host_set_api(const zt_host_api *api) {
    if (api == NULL) {
        zt_host_api_state.read_file = zt_host_default_read_file;
        zt_host_api_state.write_file = zt_host_default_write_file;
        zt_host_api_state.path_exists = zt_host_default_path_exists;
        zt_host_api_state.read_line_stdin = zt_host_default_read_line_stdin;
        zt_host_api_state.read_all_stdin = zt_host_default_read_all_stdin;
        zt_host_api_state.write_stdout = zt_host_default_write_stdout;
        zt_host_api_state.write_stderr = zt_host_default_write_stderr;
        zt_host_api_state.time_now_unix_ms = zt_host_default_time_now_unix_ms;
        zt_host_api_state.time_sleep_ms = zt_host_default_time_sleep_ms;
        zt_host_api_state.random_seed = zt_host_default_random_seed;
        zt_host_api_state.random_next_i64 = zt_host_default_random_next_i64;
        zt_host_api_state.os_current_dir = zt_host_default_os_current_dir;
        zt_host_api_state.os_change_dir = zt_host_default_os_change_dir;
        zt_host_api_state.os_args = zt_host_default_os_args;
        zt_host_api_state.os_env = zt_host_default_os_env;
        zt_host_api_state.os_pid = zt_host_default_os_pid;
        zt_host_api_state.os_platform = zt_host_default_os_platform;
        zt_host_api_state.os_arch = zt_host_default_os_arch;
        zt_host_api_state.process_run = zt_host_default_process_run;
        zt_host_api_state.process_run_capture = zt_host_default_process_run_capture;
        return;
    }

    zt_host_api_state.read_file = api->read_file != NULL ? api->read_file : zt_host_default_read_file;
    zt_host_api_state.write_file = api->write_file != NULL ? api->write_file : zt_host_default_write_file;
    zt_host_api_state.path_exists = api->path_exists != NULL ? api->path_exists : zt_host_default_path_exists;
    zt_host_api_state.read_line_stdin = api->read_line_stdin != NULL ? api->read_line_stdin : zt_host_default_read_line_stdin;
    zt_host_api_state.read_all_stdin = api->read_all_stdin != NULL ? api->read_all_stdin : zt_host_default_read_all_stdin;
    zt_host_api_state.write_stdout = api->write_stdout != NULL ? api->write_stdout : zt_host_default_write_stdout;
    zt_host_api_state.write_stderr = api->write_stderr != NULL ? api->write_stderr : zt_host_default_write_stderr;
    zt_host_api_state.time_now_unix_ms = api->time_now_unix_ms != NULL ? api->time_now_unix_ms : zt_host_default_time_now_unix_ms;
    zt_host_api_state.time_sleep_ms = api->time_sleep_ms != NULL ? api->time_sleep_ms : zt_host_default_time_sleep_ms;
    zt_host_api_state.random_seed = api->random_seed != NULL ? api->random_seed : zt_host_default_random_seed;
    zt_host_api_state.random_next_i64 = api->random_next_i64 != NULL ? api->random_next_i64 : zt_host_default_random_next_i64;
    zt_host_api_state.os_current_dir = api->os_current_dir != NULL ? api->os_current_dir : zt_host_default_os_current_dir;
    zt_host_api_state.os_change_dir = api->os_change_dir != NULL ? api->os_change_dir : zt_host_default_os_change_dir;
    zt_host_api_state.os_args = api->os_args != NULL ? api->os_args : zt_host_default_os_args;
    zt_host_api_state.os_env = api->os_env != NULL ? api->os_env : zt_host_default_os_env;
    zt_host_api_state.os_pid = api->os_pid != NULL ? api->os_pid : zt_host_default_os_pid;
    zt_host_api_state.os_platform = api->os_platform != NULL ? api->os_platform : zt_host_default_os_platform;
    zt_host_api_state.os_arch = api->os_arch != NULL ? api->os_arch : zt_host_default_os_arch;
    zt_host_api_state.process_run = api->process_run != NULL ? api->process_run : zt_host_default_process_run;
    zt_host_api_state.process_run_capture = api->process_run_capture != NULL ? api->process_run_capture : zt_host_default_process_run_capture;
}

const zt_host_api *zt_host_get_api(void) {
    return &zt_host_api_state;
}

zt_outcome_text_core_error zt_host_read_file(const zt_text *path) {
    return zt_host_api_state.read_file(path);
}


zt_outcome_void_core_error zt_host_write_file(const zt_text *path, const zt_text *value) {
    return zt_host_api_state.write_file(path, value);
}

zt_bool zt_host_path_exists(const zt_text *path) {
    return zt_host_api_state.path_exists(path);
}
zt_outcome_optional_text_core_error zt_host_read_line_stdin(void) {
    return zt_host_api_state.read_line_stdin();
}

zt_outcome_text_core_error zt_host_read_all_stdin(void) {
    return zt_host_api_state.read_all_stdin();
}

zt_outcome_void_core_error zt_host_write_stdout(const zt_text *value) {
    return zt_host_api_state.write_stdout(value);
}

zt_outcome_void_core_error zt_host_write_stderr(const zt_text *value) {
    return zt_host_api_state.write_stderr(value);
}

zt_int zt_host_time_now_unix_ms(void) {
    return zt_host_api_state.time_now_unix_ms();
}

zt_outcome_void_core_error zt_host_time_sleep_ms(zt_int duration_ms) {
    return zt_host_api_state.time_sleep_ms(duration_ms);
}

void zt_host_random_seed(zt_int seed) {
    zt_host_api_state.random_seed(seed);
}

zt_int zt_host_random_next_i64(void) {
    return zt_host_api_state.random_next_i64();
}

zt_outcome_text_core_error zt_host_os_current_dir(void) {
    return zt_host_api_state.os_current_dir();
}

zt_outcome_void_core_error zt_host_os_change_dir(const zt_text *path) {
    return zt_host_api_state.os_change_dir(path);
}

zt_outcome_text_core_error zt_host_os_current_dir_core(void) {
    return zt_host_os_current_dir();
}

zt_outcome_void_core_error zt_host_os_change_dir_core(const zt_text *path) {
    return zt_host_os_change_dir(path);
}

zt_outcome_void_core_error zt_host_fs_append_text_core(const zt_text *path, const zt_text *value) {
    return zt_host_default_fs_append_text(path, value);
}

zt_outcome_bytes_core_error zt_host_fs_read_bytes_core(const zt_text *path) {
    return zt_host_default_fs_read_bytes(path);
}

zt_outcome_void_core_error zt_host_fs_write_bytes_core(const zt_text *path, const zt_bytes *value) {
    return zt_host_default_fs_write_bytes(path, value);
}

zt_outcome_bool_core_error zt_host_fs_is_file_core(const zt_text *path) {
    return zt_host_default_fs_is_file(path);
}

zt_outcome_bool_core_error zt_host_fs_is_dir_core(const zt_text *path) {
    return zt_host_default_fs_is_dir(path);
}

zt_outcome_void_core_error zt_host_fs_create_dir_core(const zt_text *path) {
    return zt_host_default_fs_create_dir(path);
}

zt_outcome_void_core_error zt_host_fs_create_dir_all_core(const zt_text *path) {
    return zt_host_default_fs_create_dir_all(path);
}

zt_outcome_list_text_core_error zt_host_fs_list_core(const zt_text *path) {
    return zt_host_default_fs_list(path);
}

zt_outcome_list_text_core_error zt_host_fs_walk_dir_core(const zt_text *path) {
    return zt_host_default_fs_walk_dir(path);
}

zt_outcome_void_core_error zt_host_fs_remove_file_core(const zt_text *path) {
    return zt_host_default_fs_remove_file(path);
}

zt_outcome_void_core_error zt_host_fs_remove_dir_core(const zt_text *path) {
    return zt_host_default_fs_remove_dir(path);
}

zt_outcome_void_core_error zt_host_fs_remove_dir_all_core(const zt_text *path) {
    return zt_host_default_fs_remove_dir_all(path);
}

zt_outcome_void_core_error zt_host_fs_copy_file_core(const zt_text *from_path, const zt_text *to_path) {
    return zt_host_default_fs_copy_file(from_path, to_path);
}

zt_outcome_void_core_error zt_host_fs_move_core(const zt_text *from_path, const zt_text *to_path) {
    return zt_host_default_fs_move(from_path, to_path);
}

zt_outcome_i64_core_error zt_host_fs_size_core(const zt_text *path) {
    return zt_host_default_fs_size(path);
}

zt_outcome_i64_core_error zt_host_fs_modified_at_core(const zt_text *path) {
    return zt_host_default_fs_modified_at(path);
}

zt_outcome_optional_i64_core_error zt_host_fs_created_at_core(const zt_text *path) {
    return zt_host_default_fs_created_at(path);
}

zt_list_text *zt_host_os_args(void) {
    return zt_host_api_state.os_args();
}

zt_optional_text zt_host_os_env(const zt_text *name) {
    return zt_host_api_state.os_env(name);
}

zt_int zt_host_os_pid(void) {
    return zt_host_api_state.os_pid();
}

zt_text *zt_host_os_platform(void) {
    return zt_host_api_state.os_platform();
}

zt_text *zt_host_os_arch(void) {
    return zt_host_api_state.os_arch();
}

zt_outcome_i64_core_error zt_host_process_run(const zt_text *program, const zt_list_text *args, zt_optional_text cwd) {
    return zt_host_api_state.process_run(program, args, cwd);
}

zt_outcome_process_captured_run_core_error zt_host_process_run_capture(
        const zt_text *program,
        const zt_list_text *args,
        zt_optional_text cwd) {
    return zt_host_api_state.process_run_capture(program, args, cwd);
}

zt_outcome_i64_core_error zt_host_process_run_core(const zt_text *program, const zt_list_text *args, zt_optional_text cwd) {
    return zt_host_process_run(program, args, cwd);
}

zt_outcome_process_captured_run_core_error zt_host_process_run_capture_core(
        const zt_text *program,
        const zt_list_text *args,
        zt_optional_text cwd) {
    return zt_host_process_run_capture(program, args, cwd);
}

