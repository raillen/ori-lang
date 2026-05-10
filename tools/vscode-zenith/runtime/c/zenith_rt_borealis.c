#define ZT_BOREALIS_BACKEND_STUB 0
#define ZT_BOREALIS_BACKEND_RAYLIB 1
#define ZT_BOREALIS_STUB_WINDOW_ID (-1)
#define ZT_BOREALIS_RAYLIB_WINDOW_ID 1
#define ZT_BOREALIS_MAX_WINDOWS 8
#define ZT_BOREALIS_MAX_KEYS_PER_WINDOW 64
#define ZT_BOREALIS_MAX_RAYLIB_TEXTURES 256
#define ZT_BOREALIS_MAX_RAYLIB_SOUNDS 128
#define ZT_BOREALIS_MAX_RAYLIB_MODELS 128
#define ZT_BOREALIS_PATH_CAPACITY 4096

#ifdef _WIN32
#define ZT_BOREALIS_RAYLIB_PLATFORM_DIR "windows-x64"
#define ZT_BOREALIS_RAYLIB_OS_DIR "windows"
#else
#ifdef __APPLE__
#if defined(__aarch64__) || defined(__arm64__)
#define ZT_BOREALIS_RAYLIB_PLATFORM_DIR "macos-arm64"
#else
#define ZT_BOREALIS_RAYLIB_PLATFORM_DIR "macos-x64"
#endif
#define ZT_BOREALIS_RAYLIB_OS_DIR "macos"
#else
#if defined(__aarch64__)
#define ZT_BOREALIS_RAYLIB_PLATFORM_DIR "linux-arm64"
#else
#define ZT_BOREALIS_RAYLIB_PLATFORM_DIR "linux-x64"
#endif
#define ZT_BOREALIS_RAYLIB_OS_DIR "linux"
#endif
#endif

typedef struct zt_borealis_key_state {
    zt_bool used;
    zt_int input_code;
    zt_bool raw_down;
    zt_bool down;
    zt_bool prev_down;
} zt_borealis_key_state;

typedef struct zt_borealis_window_state {
    zt_bool used;
    zt_int window_id;
    zt_bool is_stub;
    zt_borealis_key_state keys[ZT_BOREALIS_MAX_KEYS_PER_WINDOW];
} zt_borealis_window_state;

static zt_borealis_window_state zt_borealis_window_states[ZT_BOREALIS_MAX_WINDOWS];
static const zt_borealis_desktop_api *zt_borealis_desktop_api_state = NULL;


static zt_core_error zt_borealis_backend_missing_error(void) {
    return zt_core_error_from_message(
        "borealis.backend_not_linked",
        "Borealis backend not linked. Configure [build].linker_flags in zenith.ztproj.");
}

static zt_outcome_i64_core_error zt_borealis_backend_missing_i64(void) {
    zt_core_error error = zt_borealis_backend_missing_error();
    zt_outcome_i64_core_error outcome = zt_outcome_i64_core_error_failure(error);
    zt_core_error_dispose(&error);
    return outcome;
}

static zt_outcome_void_core_error zt_borealis_backend_missing_void(void) {
    zt_core_error error = zt_borealis_backend_missing_error();
    zt_outcome_void_core_error outcome = zt_outcome_void_core_error_failure(error);
    zt_core_error_dispose(&error);
    return outcome;
}

static zt_borealis_window_state *zt_borealis_find_window_state(zt_int window_id) {
    size_t index;
    for (index = 0; index < ZT_BOREALIS_MAX_WINDOWS; index += 1) {
        if (zt_borealis_window_states[index].used &&
            zt_borealis_window_states[index].window_id == window_id) {
            return &zt_borealis_window_states[index];
        }
    }
    return NULL;
}

static zt_borealis_window_state *zt_borealis_alloc_window_state(zt_int window_id) {
    size_t index;
    zt_borealis_window_state *state = zt_borealis_find_window_state(window_id);
    if (state != NULL) {
        return state;
    }
    for (index = 0; index < ZT_BOREALIS_MAX_WINDOWS; index += 1) {
        if (!zt_borealis_window_states[index].used) {
            memset(&zt_borealis_window_states[index], 0, sizeof(zt_borealis_window_state));
            zt_borealis_window_states[index].used = true;
            zt_borealis_window_states[index].window_id = window_id;
            return &zt_borealis_window_states[index];
        }
    }
    return NULL;
}

static void zt_borealis_free_window_state(zt_int window_id) {
    zt_borealis_window_state *state = zt_borealis_find_window_state(window_id);
    if (state != NULL) {
        memset(state, 0, sizeof(zt_borealis_window_state));
    }
}

static zt_bool zt_borealis_is_stub_window(zt_int window_id) {
    zt_borealis_window_state *state = zt_borealis_find_window_state(window_id);
    return state != NULL && state->is_stub;
}

static zt_outcome_i64_core_error zt_borealis_open_stub_window(void) {
    zt_borealis_window_state *window_state = zt_borealis_alloc_window_state(ZT_BOREALIS_STUB_WINDOW_ID);
    if (window_state == NULL) {
        return zt_outcome_i64_core_error_failure_message("borealis: no free window slots");
    }
    window_state->is_stub = true;
    return zt_outcome_i64_core_error_success(ZT_BOREALIS_STUB_WINDOW_ID);
}

void zt_borealis_set_desktop_api(const zt_borealis_desktop_api *api) {
    zt_borealis_desktop_api_state = api;
}

const zt_borealis_desktop_api *zt_borealis_get_desktop_api(void) {
    return zt_borealis_desktop_api_state;
}

typedef struct zt_borealis_raylib_color {
    unsigned char r;
    unsigned char g;
    unsigned char b;
    unsigned char a;
} zt_borealis_raylib_color;

typedef struct zt_borealis_raylib_vector2 {
    float x;
    float y;
} zt_borealis_raylib_vector2;

typedef struct zt_borealis_raylib_rectangle {
    float x;
    float y;
    float width;
    float height;
} zt_borealis_raylib_rectangle;

typedef struct zt_borealis_raylib_vector3 {
    float x;
    float y;
    float z;
} zt_borealis_raylib_vector3;

typedef struct zt_borealis_raylib_vector4 {
    float x;
    float y;
    float z;
    float w;
} zt_borealis_raylib_vector4;

typedef zt_borealis_raylib_vector4 zt_borealis_raylib_quaternion;

typedef struct zt_borealis_raylib_matrix {
    float m0;
    float m4;
    float m8;
    float m12;
    float m1;
    float m5;
    float m9;
    float m13;
    float m2;
    float m6;
    float m10;
    float m14;
    float m3;
    float m7;
    float m11;
    float m15;
} zt_borealis_raylib_matrix;

typedef struct zt_borealis_raylib_texture {
    unsigned int id;
    int width;
    int height;
    int mipmaps;
    int format;
} zt_borealis_raylib_texture;

typedef struct zt_borealis_raylib_audio_stream {
    void *buffer;
    void *processor;
    unsigned int sampleRate;
    unsigned int sampleSize;
    unsigned int channels;
} zt_borealis_raylib_audio_stream;

typedef struct zt_borealis_raylib_sound {
    zt_borealis_raylib_audio_stream stream;
    unsigned int frameCount;
} zt_borealis_raylib_sound;

typedef struct zt_borealis_raylib_camera3d {
    zt_borealis_raylib_vector3 position;
    zt_borealis_raylib_vector3 target;
    zt_borealis_raylib_vector3 up;
    float fovy;
    int projection;
} zt_borealis_raylib_camera3d;

typedef struct zt_borealis_raylib_mesh {
    int vertexCount;
    int triangleCount;
    float *vertices;
    float *texcoords;
    float *texcoords2;
    float *normals;
    float *tangents;
    unsigned char *colors;
    unsigned short *indices;
    int boneCount;
    unsigned char *boneIndices;
    float *boneWeights;
    float *animVertices;
    float *animNormals;
    unsigned int vaoId;
    unsigned int *vboId;
} zt_borealis_raylib_mesh;

typedef struct zt_borealis_raylib_shader {
    unsigned int id;
    int *locs;
} zt_borealis_raylib_shader;

typedef struct zt_borealis_raylib_material_map {
    zt_borealis_raylib_texture texture;
    zt_borealis_raylib_color color;
    float value;
} zt_borealis_raylib_material_map;

typedef struct zt_borealis_raylib_material {
    zt_borealis_raylib_shader shader;
    zt_borealis_raylib_material_map *maps;
    float params[4];
} zt_borealis_raylib_material;

typedef struct zt_borealis_raylib_transform {
    zt_borealis_raylib_vector3 translation;
    zt_borealis_raylib_quaternion rotation;
    zt_borealis_raylib_vector3 scale;
} zt_borealis_raylib_transform;

typedef zt_borealis_raylib_transform *zt_borealis_raylib_model_anim_pose;

typedef struct zt_borealis_raylib_bone_info {
    char name[32];
    int parent;
} zt_borealis_raylib_bone_info;

typedef struct zt_borealis_raylib_model_skeleton {
    int boneCount;
    zt_borealis_raylib_bone_info *bones;
    zt_borealis_raylib_model_anim_pose bindPose;
} zt_borealis_raylib_model_skeleton;

typedef struct zt_borealis_raylib_model {
    zt_borealis_raylib_matrix transform;
    int meshCount;
    int materialCount;
    zt_borealis_raylib_mesh *meshes;
    zt_borealis_raylib_material *materials;
    int *meshMaterial;
    zt_borealis_raylib_model_skeleton skeleton;
    zt_borealis_raylib_model_anim_pose currentPose;
    zt_borealis_raylib_matrix *boneMatrices;
} zt_borealis_raylib_model;

typedef void (*zt_borealis_raylib_init_window_fn)(int width, int height, const char *title);
typedef void (*zt_borealis_raylib_close_window_fn)(void);
typedef int (*zt_borealis_raylib_window_should_close_fn)(void);
typedef int (*zt_borealis_raylib_is_window_ready_fn)(void);
typedef void (*zt_borealis_raylib_set_target_fps_fn)(int fps);
typedef void (*zt_borealis_raylib_begin_drawing_fn)(void);
typedef void (*zt_borealis_raylib_end_drawing_fn)(void);
typedef void (*zt_borealis_raylib_begin_mode3d_fn)(zt_borealis_raylib_camera3d camera);
typedef void (*zt_borealis_raylib_end_mode3d_fn)(void);
typedef void (*zt_borealis_raylib_clear_background_fn)(zt_borealis_raylib_color color);
typedef void (*zt_borealis_raylib_draw_rectangle_fn)(int pos_x, int pos_y, int width, int height, zt_borealis_raylib_color color);
typedef void (*zt_borealis_raylib_draw_rectangle_lines_fn)(int pos_x, int pos_y, int width, int height, zt_borealis_raylib_color color);
typedef void (*zt_borealis_raylib_draw_line_fn)(int start_x, int start_y, int end_x, int end_y, zt_borealis_raylib_color color);
typedef void (*zt_borealis_raylib_draw_circle_fn)(int center_x, int center_y, float radius, zt_borealis_raylib_color color);
typedef void (*zt_borealis_raylib_draw_circle_lines_fn)(int center_x, int center_y, float radius, zt_borealis_raylib_color color);
typedef void (*zt_borealis_raylib_draw_text_fn)(const char *text, int pos_x, int pos_y, int font_size, zt_borealis_raylib_color color);
typedef int (*zt_borealis_raylib_is_key_fn)(int key);
typedef void (*zt_borealis_raylib_draw_triangle_fn)(zt_borealis_raylib_vector2 v1, zt_borealis_raylib_vector2 v2, zt_borealis_raylib_vector2 v3, zt_borealis_raylib_color color);
typedef void (*zt_borealis_raylib_draw_ellipse_fn)(int center_x, int center_y, float radius_h, float radius_v, zt_borealis_raylib_color color);
typedef void (*zt_borealis_raylib_draw_cube_v_fn)(zt_borealis_raylib_vector3 position, zt_borealis_raylib_vector3 size, zt_borealis_raylib_color color);
typedef void (*zt_borealis_raylib_draw_grid_fn)(int slices, float spacing);
typedef void (*zt_borealis_raylib_draw_billboard_rec_fn)(
    zt_borealis_raylib_camera3d camera,
    zt_borealis_raylib_texture texture,
    zt_borealis_raylib_rectangle source,
    zt_borealis_raylib_vector3 position,
    zt_borealis_raylib_vector2 size,
    zt_borealis_raylib_color tint);
typedef int (*zt_borealis_raylib_measure_text_fn)(const char *text, int font_size);
typedef zt_borealis_raylib_texture (*zt_borealis_raylib_load_texture_fn)(const char *file_name);
typedef void (*zt_borealis_raylib_unload_texture_fn)(zt_borealis_raylib_texture texture);
typedef void (*zt_borealis_raylib_draw_texture_fn)(zt_borealis_raylib_texture texture, int pos_x, int pos_y, zt_borealis_raylib_color tint);
typedef void (*zt_borealis_raylib_draw_texture_ex_fn)(zt_borealis_raylib_texture texture, zt_borealis_raylib_vector2 position, float rotation, float scale, zt_borealis_raylib_color tint);
typedef void (*zt_borealis_raylib_init_audio_device_fn)(void);
typedef void (*zt_borealis_raylib_close_audio_device_fn)(void);
typedef int (*zt_borealis_raylib_is_audio_device_ready_fn)(void);
typedef void (*zt_borealis_raylib_set_master_volume_fn)(float volume);
typedef zt_borealis_raylib_sound (*zt_borealis_raylib_load_sound_fn)(const char *file_name);
typedef void (*zt_borealis_raylib_unload_sound_fn)(zt_borealis_raylib_sound sound);
typedef void (*zt_borealis_raylib_play_sound_fn)(zt_borealis_raylib_sound sound);
typedef void (*zt_borealis_raylib_stop_sound_fn)(zt_borealis_raylib_sound sound);
typedef void (*zt_borealis_raylib_set_sound_volume_fn)(zt_borealis_raylib_sound sound, float volume);
typedef zt_borealis_raylib_model (*zt_borealis_raylib_load_model_fn)(const char *file_name);
typedef int (*zt_borealis_raylib_is_model_valid_fn)(zt_borealis_raylib_model model);
typedef void (*zt_borealis_raylib_unload_model_fn)(zt_borealis_raylib_model model);
typedef void (*zt_borealis_raylib_draw_model_ex_fn)(
    zt_borealis_raylib_model model,
    zt_borealis_raylib_vector3 position,
    zt_borealis_raylib_vector3 rotation_axis,
    float rotation_angle,
    zt_borealis_raylib_vector3 scale,
    zt_borealis_raylib_color tint);

typedef struct zt_borealis_raylib_runtime {
    zt_bool load_attempted;
    zt_bool loaded;
    zt_bool window_open;
    zt_bool frame_open;
    zt_bool mode3d_open;
    zt_int window_id;
    void *library;
    char loaded_path[ZT_BOREALIS_PATH_CAPACITY];
    zt_borealis_raylib_init_window_fn init_window;
    zt_borealis_raylib_close_window_fn close_window;
    zt_borealis_raylib_window_should_close_fn window_should_close;
    zt_borealis_raylib_is_window_ready_fn is_window_ready;
    zt_borealis_raylib_set_target_fps_fn set_target_fps;
    zt_borealis_raylib_begin_drawing_fn begin_drawing;
    zt_borealis_raylib_end_drawing_fn end_drawing;
    zt_borealis_raylib_begin_mode3d_fn begin_mode3d;
    zt_borealis_raylib_end_mode3d_fn end_mode3d;
    zt_borealis_raylib_clear_background_fn clear_background;
    zt_borealis_raylib_draw_rectangle_fn draw_rectangle;
    zt_borealis_raylib_draw_rectangle_lines_fn draw_rectangle_lines;
    zt_borealis_raylib_draw_line_fn draw_line;
    zt_borealis_raylib_draw_circle_fn draw_circle;
    zt_borealis_raylib_draw_circle_lines_fn draw_circle_lines;
    zt_borealis_raylib_draw_text_fn draw_text;
    zt_borealis_raylib_is_key_fn is_key_down;
    zt_borealis_raylib_is_key_fn is_key_pressed;
    zt_borealis_raylib_is_key_fn is_key_released;
    zt_borealis_raylib_draw_triangle_fn draw_triangle;
    zt_borealis_raylib_draw_ellipse_fn draw_ellipse;
    zt_borealis_raylib_draw_cube_v_fn draw_cube_v;
    zt_borealis_raylib_draw_grid_fn draw_grid;
    zt_borealis_raylib_draw_billboard_rec_fn draw_billboard_rec;
    zt_borealis_raylib_measure_text_fn measure_text;
    zt_borealis_raylib_load_texture_fn load_texture;
    zt_borealis_raylib_unload_texture_fn unload_texture;
    zt_borealis_raylib_draw_texture_fn draw_texture;
    zt_borealis_raylib_draw_texture_ex_fn draw_texture_ex;
    zt_borealis_raylib_init_audio_device_fn init_audio_device;
    zt_borealis_raylib_close_audio_device_fn close_audio_device;
    zt_borealis_raylib_is_audio_device_ready_fn is_audio_device_ready;
    zt_borealis_raylib_set_master_volume_fn set_master_volume;
    zt_borealis_raylib_load_sound_fn load_sound;
    zt_borealis_raylib_unload_sound_fn unload_sound;
    zt_borealis_raylib_play_sound_fn play_sound;
    zt_borealis_raylib_stop_sound_fn stop_sound;
    zt_borealis_raylib_set_sound_volume_fn set_sound_volume;
    zt_borealis_raylib_load_model_fn load_model;
    zt_borealis_raylib_is_model_valid_fn is_model_valid;
    zt_borealis_raylib_unload_model_fn unload_model;
    zt_borealis_raylib_draw_model_ex_fn draw_model_ex;
} zt_borealis_raylib_runtime;

typedef struct zt_borealis_raylib_texture_slot {
    zt_bool used;
    zt_int handle;
    zt_borealis_raylib_texture texture;
} zt_borealis_raylib_texture_slot;

typedef struct zt_borealis_raylib_sound_slot {
    zt_bool used;
    zt_int handle;
    zt_borealis_raylib_sound sound;
} zt_borealis_raylib_sound_slot;

typedef struct zt_borealis_raylib_model_slot {
    zt_bool used;
    zt_int handle;
    zt_borealis_raylib_model model;
} zt_borealis_raylib_model_slot;

static zt_borealis_raylib_runtime zt_borealis_raylib = {0};
static zt_borealis_raylib_texture_slot zt_borealis_raylib_textures[ZT_BOREALIS_MAX_RAYLIB_TEXTURES];
static zt_borealis_raylib_sound_slot zt_borealis_raylib_sounds[ZT_BOREALIS_MAX_RAYLIB_SOUNDS];
static zt_borealis_raylib_model_slot zt_borealis_raylib_models[ZT_BOREALIS_MAX_RAYLIB_MODELS];
static zt_int zt_borealis_raylib_next_texture_handle = 1;
static zt_int zt_borealis_raylib_next_sound_handle = 1;
static zt_int zt_borealis_raylib_next_model_handle = 1;

static void *zt_borealis_dynlib_open(const char *name) {
#ifdef _WIN32
    return (void *)LoadLibraryA(name);
#else
    return dlopen(name, RTLD_NOW | RTLD_LOCAL);
#endif
}

static void *zt_borealis_dynlib_symbol(void *library, const char *name) {
    if (library == NULL || name == NULL) {
        return NULL;
    }
#ifdef _WIN32
    return (void *)GetProcAddress((HMODULE)library, name);
#else
    return dlsym(library, name);
#endif
}

static void zt_borealis_dynlib_close(void *library) {
    if (library == NULL) {
        return;
    }
#ifdef _WIN32
    FreeLibrary((HMODULE)library);
#else
    dlclose(library);
#endif
}

static zt_bool zt_borealis_copy_cstr(char *dest, size_t capacity, const char *source) {
    size_t length;
    if (dest == NULL || capacity == 0 || source == NULL) {
        return false;
    }
    length = strlen(source);
    if (length >= capacity) {
        return false;
    }
    memcpy(dest, source, length + 1);
    return true;
}

static zt_bool zt_borealis_path_is_sep(char value) {
    return value == '/' || value == '\\';
}

static zt_bool zt_borealis_path_join(char *dest, size_t capacity, const char *left, const char *right) {
    size_t left_len;
    size_t right_len;
    zt_bool needs_sep;

    if (dest == NULL || capacity == 0 || left == NULL || right == NULL || left[0] == '\0' || right[0] == '\0') {
        return false;
    }

    left_len = strlen(left);
    right_len = strlen(right);
    needs_sep = !zt_borealis_path_is_sep(left[left_len - 1]);
    if (left_len + (needs_sep ? 1u : 0u) + right_len >= capacity) {
        return false;
    }

    memcpy(dest, left, left_len);
    if (needs_sep) {
        dest[left_len] = '/';
        left_len += 1;
    }
    memcpy(dest + left_len, right, right_len + 1);
    return true;
}

static zt_bool zt_borealis_path_dirname_in_place(char *path) {
    size_t length;
    size_t index;

    if (path == NULL || path[0] == '\0') {
        return false;
    }

    length = strlen(path);
    while (length > 0 && zt_borealis_path_is_sep(path[length - 1])) {
        length -= 1;
        path[length] = '\0';
    }

    index = length;
    while (index > 0) {
        index -= 1;
        if (zt_borealis_path_is_sep(path[index])) {
            if (index == 0) {
                path[1] = '\0';
            } else {
                path[index] = '\0';
            }
            return true;
        }
    }

    return zt_borealis_copy_cstr(path, ZT_BOREALIS_PATH_CAPACITY, ".");
}

static zt_bool zt_borealis_get_cwd(char *dest, size_t capacity) {
    if (dest == NULL || capacity == 0) {
        return false;
    }
#ifdef _WIN32
    return _getcwd(dest, (int)capacity) != NULL;
#else
    return getcwd(dest, capacity) != NULL;
#endif
}

static zt_bool zt_borealis_get_executable_dir(char *dest, size_t capacity) {
    if (dest == NULL || capacity == 0) {
        return false;
    }
#ifdef _WIN32
    {
        DWORD length = GetModuleFileNameA(NULL, dest, (DWORD)capacity);
        if (length == 0 || length >= capacity) {
            dest[0] = '\0';
            return false;
        }
        return zt_borealis_path_dirname_in_place(dest);
    }
#else
#ifdef __APPLE__
    {
        uint32_t length = (uint32_t)capacity;
        if (_NSGetExecutablePath(dest, &length) != 0) {
            dest[0] = '\0';
            return false;
        }
        return zt_borealis_path_dirname_in_place(dest);
    }
#else
    {
        ssize_t length = readlink("/proc/self/exe", dest, capacity - 1);
        if (length <= 0 || (size_t)length >= capacity) {
            dest[0] = '\0';
            return false;
        }
        dest[length] = '\0';
        return zt_borealis_path_dirname_in_place(dest);
    }
#endif
#endif
}

static unsigned char zt_borealis_color_u8(zt_int value) {
    if (value < 0) return 0;
    if (value > 255) return 255;
    return (unsigned char)value;
}

static zt_borealis_raylib_color zt_borealis_make_raylib_color(zt_int r, zt_int g, zt_int b, zt_int a) {
    zt_borealis_raylib_color color;
    color.r = zt_borealis_color_u8(r);
    color.g = zt_borealis_color_u8(g);
    color.b = zt_borealis_color_u8(b);
    color.a = zt_borealis_color_u8(a);
    return color;
}

static zt_borealis_raylib_vector3 zt_borealis_make_raylib_vector3(zt_float x, zt_float y, zt_float z) {
    zt_borealis_raylib_vector3 value;
    value.x = (float)x;
    value.y = (float)y;
    value.z = (float)z;
    return value;
}

static zt_borealis_raylib_rectangle zt_borealis_make_raylib_rectangle(
        zt_float x,
        zt_float y,
        zt_float width,
        zt_float height) {
    zt_borealis_raylib_rectangle value;
    value.x = (float)x;
    value.y = (float)y;
    value.width = (float)width;
    value.height = (float)height;
    return value;
}

static zt_borealis_raylib_camera3d zt_borealis_make_raylib_camera3d(
        zt_float position_x,
        zt_float position_y,
        zt_float position_z,
        zt_float target_x,
        zt_float target_y,
        zt_float target_z,
        zt_float up_x,
        zt_float up_y,
        zt_float up_z,
        zt_float fov_y,
        zt_int projection) {
    zt_borealis_raylib_camera3d camera;
    camera.position = zt_borealis_make_raylib_vector3(position_x, position_y, position_z);
    camera.target = zt_borealis_make_raylib_vector3(target_x, target_y, target_z);
    camera.up = zt_borealis_make_raylib_vector3(up_x, up_y, up_z);
    if (fabsf(camera.up.x) < 0.0001f &&
        fabsf(camera.up.y) < 0.0001f &&
        fabsf(camera.up.z) < 0.0001f) {
        camera.up = zt_borealis_make_raylib_vector3(0.0, 1.0, 0.0);
    }
    camera.fovy = (float)fov_y;
    camera.projection = (int)projection;
    return camera;
}

static zt_bool zt_borealis_raylib_mode3d_ready(zt_int window_id) {
    return zt_borealis_raylib.window_open &&
           window_id == zt_borealis_raylib.window_id &&
           zt_borealis_raylib.frame_open &&
           zt_borealis_raylib.mode3d_open;
}

static zt_bool zt_borealis_raylib_model_loaded(zt_borealis_raylib_model model) {
    if (zt_borealis_raylib.is_model_valid != NULL) {
        return zt_borealis_raylib.is_model_valid(model) ? true : false;
    }
    return model.meshCount > 0 ? true : false;
}

static zt_bool zt_borealis_raylib_assign_required_symbols(void) {
    if (zt_borealis_raylib.library == NULL) {
        return false;
    }

    zt_borealis_raylib.init_window = (zt_borealis_raylib_init_window_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "InitWindow");
    zt_borealis_raylib.close_window = (zt_borealis_raylib_close_window_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "CloseWindow");
    zt_borealis_raylib.window_should_close = (zt_borealis_raylib_window_should_close_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "WindowShouldClose");
    zt_borealis_raylib.is_window_ready = (zt_borealis_raylib_is_window_ready_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "IsWindowReady");
    zt_borealis_raylib.set_target_fps = (zt_borealis_raylib_set_target_fps_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "SetTargetFPS");
    zt_borealis_raylib.begin_drawing = (zt_borealis_raylib_begin_drawing_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "BeginDrawing");
    zt_borealis_raylib.end_drawing = (zt_borealis_raylib_end_drawing_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "EndDrawing");
    zt_borealis_raylib.begin_mode3d = (zt_borealis_raylib_begin_mode3d_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "BeginMode3D");
    zt_borealis_raylib.end_mode3d = (zt_borealis_raylib_end_mode3d_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "EndMode3D");
    zt_borealis_raylib.clear_background = (zt_borealis_raylib_clear_background_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "ClearBackground");
    zt_borealis_raylib.draw_rectangle = (zt_borealis_raylib_draw_rectangle_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "DrawRectangle");
    zt_borealis_raylib.draw_rectangle_lines = (zt_borealis_raylib_draw_rectangle_lines_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "DrawRectangleLines");
    zt_borealis_raylib.draw_line = (zt_borealis_raylib_draw_line_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "DrawLine");
    zt_borealis_raylib.draw_circle = (zt_borealis_raylib_draw_circle_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "DrawCircle");
    zt_borealis_raylib.draw_circle_lines = (zt_borealis_raylib_draw_circle_lines_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "DrawCircleLines");
    zt_borealis_raylib.draw_text = (zt_borealis_raylib_draw_text_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "DrawText");
    zt_borealis_raylib.is_key_down = (zt_borealis_raylib_is_key_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "IsKeyDown");
    zt_borealis_raylib.is_key_pressed = (zt_borealis_raylib_is_key_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "IsKeyPressed");
    zt_borealis_raylib.is_key_released = (zt_borealis_raylib_is_key_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "IsKeyReleased");
    zt_borealis_raylib.draw_triangle = (zt_borealis_raylib_draw_triangle_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "DrawTriangle");
    zt_borealis_raylib.draw_ellipse = (zt_borealis_raylib_draw_ellipse_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "DrawEllipse");
    zt_borealis_raylib.draw_cube_v = (zt_borealis_raylib_draw_cube_v_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "DrawCubeV");
    zt_borealis_raylib.draw_grid = (zt_borealis_raylib_draw_grid_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "DrawGrid");
    zt_borealis_raylib.draw_billboard_rec = (zt_borealis_raylib_draw_billboard_rec_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "DrawBillboardRec");
    zt_borealis_raylib.measure_text = (zt_borealis_raylib_measure_text_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "MeasureText");
    zt_borealis_raylib.load_texture = (zt_borealis_raylib_load_texture_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "LoadTexture");
    zt_borealis_raylib.unload_texture = (zt_borealis_raylib_unload_texture_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "UnloadTexture");
    zt_borealis_raylib.draw_texture = (zt_borealis_raylib_draw_texture_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "DrawTexture");
    zt_borealis_raylib.draw_texture_ex = (zt_borealis_raylib_draw_texture_ex_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "DrawTextureEx");
    zt_borealis_raylib.init_audio_device = (zt_borealis_raylib_init_audio_device_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "InitAudioDevice");
    zt_borealis_raylib.close_audio_device = (zt_borealis_raylib_close_audio_device_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "CloseAudioDevice");
    zt_borealis_raylib.is_audio_device_ready = (zt_borealis_raylib_is_audio_device_ready_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "IsAudioDeviceReady");
    zt_borealis_raylib.set_master_volume = (zt_borealis_raylib_set_master_volume_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "SetMasterVolume");
    zt_borealis_raylib.load_sound = (zt_borealis_raylib_load_sound_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "LoadSound");
    zt_borealis_raylib.unload_sound = (zt_borealis_raylib_unload_sound_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "UnloadSound");
    zt_borealis_raylib.play_sound = (zt_borealis_raylib_play_sound_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "PlaySound");
    zt_borealis_raylib.stop_sound = (zt_borealis_raylib_stop_sound_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "StopSound");
    zt_borealis_raylib.set_sound_volume = (zt_borealis_raylib_set_sound_volume_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "SetSoundVolume");
    zt_borealis_raylib.load_model = (zt_borealis_raylib_load_model_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "LoadModel");
    zt_borealis_raylib.is_model_valid = (zt_borealis_raylib_is_model_valid_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "IsModelValid");
    zt_borealis_raylib.unload_model = (zt_borealis_raylib_unload_model_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "UnloadModel");
    zt_borealis_raylib.draw_model_ex = (zt_borealis_raylib_draw_model_ex_fn)zt_borealis_dynlib_symbol(zt_borealis_raylib.library, "DrawModelEx");

    return zt_borealis_raylib.init_window != NULL &&
           zt_borealis_raylib.close_window != NULL &&
           zt_borealis_raylib.window_should_close != NULL &&
           zt_borealis_raylib.set_target_fps != NULL &&
           zt_borealis_raylib.begin_drawing != NULL &&
           zt_borealis_raylib.end_drawing != NULL &&
           zt_borealis_raylib.clear_background != NULL &&
           zt_borealis_raylib.draw_rectangle != NULL &&
           zt_borealis_raylib.draw_rectangle_lines != NULL &&
           zt_borealis_raylib.draw_line != NULL &&
           zt_borealis_raylib.draw_circle != NULL &&
           zt_borealis_raylib.draw_circle_lines != NULL &&
           zt_borealis_raylib.draw_text != NULL &&
           zt_borealis_raylib.is_key_down != NULL &&
           zt_borealis_raylib.is_key_pressed != NULL &&
           zt_borealis_raylib.is_key_released != NULL;
}

static void zt_borealis_raylib_reset_failed_candidate(void) {
    if (zt_borealis_raylib.library != NULL) {
        zt_borealis_dynlib_close(zt_borealis_raylib.library);
    }
    memset(&zt_borealis_raylib, 0, sizeof(zt_borealis_raylib));
    zt_borealis_raylib.load_attempted = true;
    zt_borealis_raylib.window_id = ZT_BOREALIS_RAYLIB_WINDOW_ID;
}

static zt_bool zt_borealis_raylib_open_candidate(const char *path) {
    if (path == NULL || path[0] == '\0') {
        return false;
    }

    zt_borealis_raylib.library = zt_borealis_dynlib_open(path);
    if (zt_borealis_raylib.library == NULL) {
        return false;
    }

    if (zt_borealis_raylib_assign_required_symbols()) {
        zt_borealis_raylib.loaded = true;
        zt_borealis_copy_cstr(zt_borealis_raylib.loaded_path, sizeof(zt_borealis_raylib.loaded_path), path);
        return true;
    }

    zt_borealis_raylib_reset_failed_candidate();
    return false;
}

static zt_bool zt_borealis_raylib_try_names_in_dir(const char *dir, const char *const *names) {
    size_t index;
    char candidate[ZT_BOREALIS_PATH_CAPACITY];

    if (dir == NULL || dir[0] == '\0' || names == NULL) {
        return false;
    }

    for (index = 0; names[index] != NULL; index += 1) {
        if (!zt_borealis_path_join(candidate, sizeof(candidate), dir, names[index])) {
            continue;
        }
        if (zt_borealis_raylib_open_candidate(candidate)) {
            return true;
        }
    }

    return false;
}

static zt_bool zt_borealis_raylib_try_relative_dir(
        const char *root,
        const char *relative_dir,
        const char *const *names) {
    char dir[ZT_BOREALIS_PATH_CAPACITY];
    if (!zt_borealis_path_join(dir, sizeof(dir), root, relative_dir)) {
        return false;
    }
    return zt_borealis_raylib_try_names_in_dir(dir, names);
}

static zt_bool zt_borealis_raylib_try_module_layout(const char *root, const char *const *names) {
    static const char *relative_dirs[] = {
        "packages/borealis/native/raylib/" ZT_BOREALIS_RAYLIB_PLATFORM_DIR,
        "packages/borealis/native/raylib/" ZT_BOREALIS_RAYLIB_PLATFORM_DIR "/lib",
        "packages/borealis/native/raylib/" ZT_BOREALIS_RAYLIB_OS_DIR,
        "packages/borealis/native/raylib/" ZT_BOREALIS_RAYLIB_OS_DIR "/lib",
        "packages/borealis/native/raylib",
        "packages/borealis/native/raylib/lib",
        "native/raylib/" ZT_BOREALIS_RAYLIB_PLATFORM_DIR,
        "native/raylib/" ZT_BOREALIS_RAYLIB_PLATFORM_DIR "/lib",
        "native/raylib/" ZT_BOREALIS_RAYLIB_OS_DIR,
        "native/raylib/" ZT_BOREALIS_RAYLIB_OS_DIR "/lib",
        "native/raylib",
        "native/raylib/lib",
        NULL
    };
    size_t index;

    if (root == NULL || root[0] == '\0') {
        return false;
    }

    for (index = 0; relative_dirs[index] != NULL; index += 1) {
        if (zt_borealis_raylib_try_relative_dir(root, relative_dirs[index], names)) {
            return true;
        }
    }

    return false;
}

static zt_bool zt_borealis_raylib_try_module_layout_upwards(const char *root, const char *const *names) {
    char current[ZT_BOREALIS_PATH_CAPACITY];
    size_t depth;

    if (!zt_borealis_copy_cstr(current, sizeof(current), root)) {
        return false;
    }

    for (depth = 0; depth < 6; depth += 1) {
        if (zt_borealis_raylib_try_module_layout(current, names)) {
            return true;
        }
        if (!zt_borealis_path_dirname_in_place(current)) {
            break;
        }
    }

    return false;
}

static zt_bool zt_borealis_raylib_try_env_path(const char *const *names) {
    const char *env_path = getenv("BOREALIS_RAYLIB_PATH");
    if (env_path == NULL || env_path[0] == '\0') {
        env_path = getenv("ZENITH_RAYLIB_PATH");
    }
    if (env_path == NULL || env_path[0] == '\0') {
        return false;
    }

    if (zt_borealis_raylib_open_candidate(env_path)) {
        return true;
    }
    return zt_borealis_raylib_try_names_in_dir(env_path, names);
}

static zt_bool zt_borealis_raylib_try_load(void) {
    size_t index;
#ifdef _WIN32
    const char *candidates[] = {"raylib.dll", "libraylib.dll", NULL};
#elif __APPLE__
    const char *candidates[] = {"libraylib.dylib", "raylib.dylib", NULL};
#else
    const char *candidates[] = {"libraylib.so", "libraylib.so.5", "libraylib.so.4", NULL};
#endif
    char root[ZT_BOREALIS_PATH_CAPACITY];

    if (zt_borealis_raylib.load_attempted) {
        return zt_borealis_raylib.loaded;
    }

    zt_borealis_raylib.load_attempted = true;
    zt_borealis_raylib.window_id = ZT_BOREALIS_RAYLIB_WINDOW_ID;

    if (zt_borealis_raylib_try_env_path(candidates)) {
        return true;
    }

    for (index = 0; candidates[index] != NULL; index += 1) {
        if (zt_borealis_raylib_open_candidate(candidates[index])) {
            return true;
        }
    }

    if (zt_borealis_get_executable_dir(root, sizeof(root))) {
        if (zt_borealis_raylib_try_names_in_dir(root, candidates)) {
            return true;
        }
        if (zt_borealis_raylib_try_module_layout_upwards(root, candidates)) {
            return true;
        }
    }

    if (zt_borealis_get_cwd(root, sizeof(root))) {
        if (zt_borealis_raylib_try_names_in_dir(root, candidates)) {
            return true;
        }
        if (zt_borealis_raylib_try_module_layout_upwards(root, candidates)) {
            return true;
        }
    }

    return false;
}

zt_bool zt_borealis_raylib_available(void) {
    return zt_borealis_raylib_try_load();
}

zt_text *zt_borealis_raylib_loaded_path(void) {
    if (!zt_borealis_raylib_try_load()) {
        return zt_text_from_utf8_literal("");
    }
    return zt_text_from_utf8(zt_borealis_raylib.loaded_path, strlen(zt_borealis_raylib.loaded_path));
}

static zt_borealis_raylib_texture_slot *zt_borealis_raylib_find_texture(zt_int handle) {
    size_t index;
    for (index = 0; index < ZT_BOREALIS_MAX_RAYLIB_TEXTURES; index += 1) {
        if (zt_borealis_raylib_textures[index].used &&
            zt_borealis_raylib_textures[index].handle == handle) {
            return &zt_borealis_raylib_textures[index];
        }
    }
    return NULL;
}

static zt_borealis_raylib_texture_slot *zt_borealis_raylib_alloc_texture(void) {
    size_t index;
    for (index = 0; index < ZT_BOREALIS_MAX_RAYLIB_TEXTURES; index += 1) {
        if (!zt_borealis_raylib_textures[index].used) {
            memset(&zt_borealis_raylib_textures[index], 0, sizeof(zt_borealis_raylib_texture_slot));
            zt_borealis_raylib_textures[index].used = true;
            zt_borealis_raylib_textures[index].handle = zt_borealis_raylib_next_texture_handle;
            zt_borealis_raylib_next_texture_handle += 1;
            if (zt_borealis_raylib_next_texture_handle <= 0) {
                zt_borealis_raylib_next_texture_handle = 1;
            }
            return &zt_borealis_raylib_textures[index];
        }
    }
    return NULL;
}

static zt_borealis_raylib_sound_slot *zt_borealis_raylib_find_sound(zt_int handle) {
    size_t index;
    for (index = 0; index < ZT_BOREALIS_MAX_RAYLIB_SOUNDS; index += 1) {
        if (zt_borealis_raylib_sounds[index].used &&
            zt_borealis_raylib_sounds[index].handle == handle) {
            return &zt_borealis_raylib_sounds[index];
        }
    }
    return NULL;
}

static zt_borealis_raylib_sound_slot *zt_borealis_raylib_alloc_sound(void) {
    size_t index;
    for (index = 0; index < ZT_BOREALIS_MAX_RAYLIB_SOUNDS; index += 1) {
        if (!zt_borealis_raylib_sounds[index].used) {
            memset(&zt_borealis_raylib_sounds[index], 0, sizeof(zt_borealis_raylib_sound_slot));
            zt_borealis_raylib_sounds[index].used = true;
            zt_borealis_raylib_sounds[index].handle = zt_borealis_raylib_next_sound_handle;
            zt_borealis_raylib_next_sound_handle += 1;
            if (zt_borealis_raylib_next_sound_handle <= 0) {
                zt_borealis_raylib_next_sound_handle = 1;
            }
            return &zt_borealis_raylib_sounds[index];
        }
    }
    return NULL;
}

static zt_borealis_raylib_model_slot *zt_borealis_raylib_find_model(zt_int handle) {
    size_t index;
    for (index = 0; index < ZT_BOREALIS_MAX_RAYLIB_MODELS; index += 1) {
        if (zt_borealis_raylib_models[index].used &&
            zt_borealis_raylib_models[index].handle == handle) {
            return &zt_borealis_raylib_models[index];
        }
    }
    return NULL;
}

static zt_borealis_raylib_model_slot *zt_borealis_raylib_alloc_model(void) {
    size_t index;
    for (index = 0; index < ZT_BOREALIS_MAX_RAYLIB_MODELS; index += 1) {
        if (!zt_borealis_raylib_models[index].used) {
            memset(&zt_borealis_raylib_models[index], 0, sizeof(zt_borealis_raylib_model_slot));
            zt_borealis_raylib_models[index].used = true;
            zt_borealis_raylib_models[index].handle = zt_borealis_raylib_next_model_handle;
            zt_borealis_raylib_next_model_handle += 1;
            if (zt_borealis_raylib_next_model_handle <= 0) {
                zt_borealis_raylib_next_model_handle = 1;
            }
            return &zt_borealis_raylib_models[index];
        }
    }
    return NULL;
}

static void zt_borealis_raylib_release_all_textures(void) {
    size_t index;
    for (index = 0; index < ZT_BOREALIS_MAX_RAYLIB_TEXTURES; index += 1) {
        if (!zt_borealis_raylib_textures[index].used) {
            continue;
        }
        if (zt_borealis_raylib.unload_texture != NULL) {
            zt_borealis_raylib.unload_texture(zt_borealis_raylib_textures[index].texture);
        }
        memset(&zt_borealis_raylib_textures[index], 0, sizeof(zt_borealis_raylib_texture_slot));
    }
}

static void zt_borealis_raylib_release_all_sounds(void) {
    size_t index;
    for (index = 0; index < ZT_BOREALIS_MAX_RAYLIB_SOUNDS; index += 1) {
        if (!zt_borealis_raylib_sounds[index].used) {
            continue;
        }
        if (zt_borealis_raylib.unload_sound != NULL) {
            zt_borealis_raylib.unload_sound(zt_borealis_raylib_sounds[index].sound);
        }
        memset(&zt_borealis_raylib_sounds[index], 0, sizeof(zt_borealis_raylib_sound_slot));
    }
}

static void zt_borealis_raylib_release_all_models(void) {
    size_t index;
    for (index = 0; index < ZT_BOREALIS_MAX_RAYLIB_MODELS; index += 1) {
        if (!zt_borealis_raylib_models[index].used) {
            continue;
        }
        if (zt_borealis_raylib.unload_model != NULL) {
            zt_borealis_raylib.unload_model(zt_borealis_raylib_models[index].model);
        }
        memset(&zt_borealis_raylib_models[index], 0, sizeof(zt_borealis_raylib_model_slot));
    }
}

static zt_outcome_i64_core_error zt_borealis_raylib_open_window(
        const zt_text *title,
        zt_int width,
        zt_int height,
        zt_int target_fps,
        zt_int backend_id) {
    const char *title_text;

    if (backend_id != ZT_BOREALIS_BACKEND_RAYLIB) {
        return zt_outcome_i64_core_error_failure_message("borealis: unsupported desktop backend id");
    }

    if (!zt_borealis_raylib_try_load()) {
        return zt_borealis_backend_missing_i64();
    }

    if (zt_borealis_raylib.window_open) {
        return zt_outcome_i64_core_error_failure_message("borealis: desktop window already open");
    }

    title_text = title != NULL ? zt_text_data(title) : "Borealis";
    zt_borealis_raylib.init_window((int)width, (int)height, title_text);

    if (zt_borealis_raylib.is_window_ready != NULL && !zt_borealis_raylib.is_window_ready()) {
        zt_borealis_raylib.close_window();
        return zt_outcome_i64_core_error_failure_message("borealis: failed to initialize raylib window");
    }

    zt_borealis_raylib.set_target_fps((int)target_fps);
    zt_borealis_raylib.window_open = true;
    zt_borealis_raylib.frame_open = false;
    zt_borealis_raylib.mode3d_open = false;
    return zt_outcome_i64_core_error_success(zt_borealis_raylib.window_id);
}

static zt_outcome_void_core_error zt_borealis_raylib_close_window(zt_int window_id) {
    if (!zt_borealis_raylib.window_open) {
        return zt_outcome_void_core_error_success();
    }

    if (window_id != zt_borealis_raylib.window_id) {
        return zt_outcome_void_core_error_failure_message("borealis: unknown desktop window id");
    }

    if (zt_borealis_raylib.mode3d_open && zt_borealis_raylib.end_mode3d != NULL) {
        zt_borealis_raylib.end_mode3d();
        zt_borealis_raylib.mode3d_open = false;
    }
    if (zt_borealis_raylib.frame_open) {
        zt_borealis_raylib.end_drawing();
        zt_borealis_raylib.frame_open = false;
    }

    zt_borealis_raylib_release_all_textures();
    zt_borealis_raylib_release_all_sounds();
    zt_borealis_raylib_release_all_models();
    if (zt_borealis_raylib.close_audio_device != NULL &&
        zt_borealis_raylib.is_audio_device_ready != NULL &&
        zt_borealis_raylib.is_audio_device_ready()) {
        zt_borealis_raylib.close_audio_device();
    }

    zt_borealis_raylib.close_window();
    zt_borealis_raylib.window_open = false;
    zt_borealis_raylib.mode3d_open = false;
    return zt_outcome_void_core_error_success();
}

static zt_bool zt_borealis_raylib_window_should_close(zt_int window_id) {
    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return true;
    }
    return zt_borealis_raylib.window_should_close() ? true : false;
}

static zt_outcome_void_core_error zt_borealis_raylib_begin_frame(
        zt_int window_id,
        zt_int clear_r,
        zt_int clear_g,
        zt_int clear_b,
        zt_int clear_a) {
    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return zt_outcome_void_core_error_failure_message("borealis: desktop window is not open");
    }

    if (zt_borealis_raylib.frame_open) {
        return zt_outcome_void_core_error_failure_message("borealis: frame_begin called twice without frame_end");
    }

    zt_borealis_raylib.begin_drawing();
    zt_borealis_raylib.clear_background(zt_borealis_make_raylib_color(clear_r, clear_g, clear_b, clear_a));
    zt_borealis_raylib.frame_open = true;
    zt_borealis_raylib.mode3d_open = false;
    return zt_outcome_void_core_error_success();
}

static zt_outcome_void_core_error zt_borealis_raylib_end_frame(zt_int window_id) {
    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return zt_outcome_void_core_error_failure_message("borealis: desktop window is not open");
    }

    if (!zt_borealis_raylib.frame_open) {
        return zt_outcome_void_core_error_success();
    }

    if (zt_borealis_raylib.mode3d_open && zt_borealis_raylib.end_mode3d != NULL) {
        zt_borealis_raylib.end_mode3d();
        zt_borealis_raylib.mode3d_open = false;
    }
    zt_borealis_raylib.end_drawing();
    zt_borealis_raylib.frame_open = false;
    return zt_outcome_void_core_error_success();
}

static zt_outcome_void_core_error zt_borealis_raylib_draw_rect(
        zt_int window_id,
        zt_float x,
        zt_float y,
        zt_float width,
        zt_float height,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a) {
    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return zt_outcome_void_core_error_failure_message("borealis: desktop window is not open");
    }

    zt_borealis_raylib.draw_rectangle(
        (int)lround(x),
        (int)lround(y),
        (int)lround(width),
        (int)lround(height),
        zt_borealis_make_raylib_color(color_r, color_g, color_b, color_a));
    return zt_outcome_void_core_error_success();
}

static zt_outcome_void_core_error zt_borealis_raylib_draw_line(
        zt_int window_id,
        zt_float x1,
        zt_float y1,
        zt_float x2,
        zt_float y2,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a) {
    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return zt_outcome_void_core_error_failure_message("borealis: desktop window is not open");
    }

    zt_borealis_raylib.draw_line(
        (int)lround(x1),
        (int)lround(y1),
        (int)lround(x2),
        (int)lround(y2),
        zt_borealis_make_raylib_color(color_r, color_g, color_b, color_a));
    return zt_outcome_void_core_error_success();
}

static zt_outcome_void_core_error zt_borealis_raylib_draw_rect_outline(
        zt_int window_id,
        zt_float x,
        zt_float y,
        zt_float width,
        zt_float height,
        zt_float thickness,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a) {
    int step;
    int line_count;

    (void)thickness;

    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return zt_outcome_void_core_error_failure_message("borealis: desktop window is not open");
    }

    line_count = thickness <= 1.0 ? 1 : (int)lround(thickness);
    if (line_count < 1) line_count = 1;

    for (step = 0; step < line_count; step += 1) {
        zt_borealis_raylib.draw_rectangle_lines(
            (int)lround(x) - step,
            (int)lround(y) - step,
            (int)lround(width) + (step * 2),
            (int)lround(height) + (step * 2),
            zt_borealis_make_raylib_color(color_r, color_g, color_b, color_a));
    }

    return zt_outcome_void_core_error_success();
}

static zt_outcome_void_core_error zt_borealis_raylib_draw_circle(
        zt_int window_id,
        zt_float x,
        zt_float y,
        zt_float radius,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a) {
    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return zt_outcome_void_core_error_failure_message("borealis: desktop window is not open");
    }

    zt_borealis_raylib.draw_circle(
        (int)lround(x),
        (int)lround(y),
        (float)radius,
        zt_borealis_make_raylib_color(color_r, color_g, color_b, color_a));
    return zt_outcome_void_core_error_success();
}

static zt_outcome_void_core_error zt_borealis_raylib_draw_circle_outline(
        zt_int window_id,
        zt_float x,
        zt_float y,
        zt_float radius,
        zt_float thickness,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a) {
    int step;
    int line_count;
    zt_borealis_raylib_color color;

    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return zt_outcome_void_core_error_failure_message("borealis: desktop window is not open");
    }

    line_count = thickness <= 1.0 ? 1 : (int)lround(thickness);
    if (line_count < 1) line_count = 1;
    color = zt_borealis_make_raylib_color(color_r, color_g, color_b, color_a);

    for (step = 0; step < line_count; step += 1) {
        float ring = radius - (float)step;
        if (ring <= 0.0f) break;
        zt_borealis_raylib.draw_circle_lines((int)lround(x), (int)lround(y), ring, color);
    }

    return zt_outcome_void_core_error_success();
}

static zt_outcome_void_core_error zt_borealis_raylib_draw_text(
        zt_int window_id,
        const zt_text *value,
        zt_int x,
        zt_int y,
        zt_int size,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a) {
    const char *text;

    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return zt_outcome_void_core_error_failure_message("borealis: desktop window is not open");
    }

    text = value != NULL ? zt_text_data(value) : "";
    zt_borealis_raylib.draw_text(
        text,
        (int)x,
        (int)y,
        (int)size,
        zt_borealis_make_raylib_color(color_r, color_g, color_b, color_a));
    return zt_outcome_void_core_error_success();
}

static zt_bool zt_borealis_raylib_is_key_down(zt_int window_id, zt_int input_code) {
    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return false;
    }
    return zt_borealis_raylib.is_key_down((int)input_code) ? true : false;
}

static zt_bool zt_borealis_raylib_is_key_pressed(zt_int window_id, zt_int input_code) {
    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return false;
    }
    return zt_borealis_raylib.is_key_pressed((int)input_code) ? true : false;
}

static zt_bool zt_borealis_raylib_is_key_released(zt_int window_id, zt_int input_code) {
    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return false;
    }
    return zt_borealis_raylib.is_key_released((int)input_code) ? true : false;
}

zt_outcome_void_core_error zt_borealis_raylib_draw_triangle(
        zt_int window_id,
        zt_float x1,
        zt_float y1,
        zt_float x2,
        zt_float y2,
        zt_float x3,
        zt_float y3,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a) {
    zt_borealis_raylib_vector2 v1;
    zt_borealis_raylib_vector2 v2;
    zt_borealis_raylib_vector2 v3;

    if (zt_borealis_is_stub_window(window_id)) {
        return zt_outcome_void_core_error_success();
    }
    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return zt_outcome_void_core_error_failure_message("borealis: desktop window is not open");
    }
    if (zt_borealis_raylib.draw_triangle == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: Raylib DrawTriangle is not available");
    }

    v1.x = (float)x1;
    v1.y = (float)y1;
    v2.x = (float)x2;
    v2.y = (float)y2;
    v3.x = (float)x3;
    v3.y = (float)y3;
    zt_borealis_raylib.draw_triangle(v1, v2, v3, zt_borealis_make_raylib_color(color_r, color_g, color_b, color_a));
    return zt_outcome_void_core_error_success();
}

zt_outcome_void_core_error zt_borealis_raylib_draw_ellipse(
        zt_int window_id,
        zt_float x,
        zt_float y,
        zt_float radius_h,
        zt_float radius_v,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a) {
    if (zt_borealis_is_stub_window(window_id)) {
        return zt_outcome_void_core_error_success();
    }
    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return zt_outcome_void_core_error_failure_message("borealis: desktop window is not open");
    }
    if (zt_borealis_raylib.draw_ellipse == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: Raylib DrawEllipse is not available");
    }

    zt_borealis_raylib.draw_ellipse(
        (int)lround(x),
        (int)lround(y),
        (float)radius_h,
        (float)radius_v,
        zt_borealis_make_raylib_color(color_r, color_g, color_b, color_a));
    return zt_outcome_void_core_error_success();
}

zt_int zt_borealis_raylib_measure_text(const zt_text *value, zt_int font_size) {
    const char *text = value != NULL ? zt_text_data(value) : "";
    zt_int fallback = (zt_int)((strlen(text) * (size_t)(font_size > 0 ? font_size : 0)) / 2u);
    if (zt_borealis_raylib_try_load() && zt_borealis_raylib.measure_text != NULL) {
        zt_int measured = (zt_int)zt_borealis_raylib.measure_text(text, (int)font_size);
        if (measured > 0 || text[0] == '\0' || font_size <= 0) {
            return measured;
        }
    }
    return fallback;
}

zt_outcome_i64_core_error zt_borealis_raylib_load_texture(const zt_text *path) {
    const char *file_name;
    zt_borealis_raylib_texture texture;
    zt_borealis_raylib_texture_slot *slot;

    if (!zt_borealis_raylib_try_load() || zt_borealis_raylib.load_texture == NULL) {
        return zt_outcome_i64_core_error_failure_message("borealis: Raylib texture support is not available");
    }

    file_name = path != NULL ? zt_text_data(path) : "";
    texture = zt_borealis_raylib.load_texture(file_name);
    if (texture.id == 0) {
        return zt_outcome_i64_core_error_failure_message("borealis: failed to load Raylib texture");
    }

    slot = zt_borealis_raylib_alloc_texture();
    if (slot == NULL) {
        if (zt_borealis_raylib.unload_texture != NULL) {
            zt_borealis_raylib.unload_texture(texture);
        }
        return zt_outcome_i64_core_error_failure_message("borealis: no free Raylib texture slots");
    }

    slot->texture = texture;
    return zt_outcome_i64_core_error_success(slot->handle);
}

zt_outcome_void_core_error zt_borealis_raylib_unload_texture(zt_int texture_handle) {
    zt_borealis_raylib_texture_slot *slot = zt_borealis_raylib_find_texture(texture_handle);
    if (slot == NULL) {
        return zt_outcome_void_core_error_success();
    }
    if (zt_borealis_raylib.unload_texture != NULL) {
        zt_borealis_raylib.unload_texture(slot->texture);
    }
    memset(slot, 0, sizeof(zt_borealis_raylib_texture_slot));
    return zt_outcome_void_core_error_success();
}

zt_int zt_borealis_raylib_texture_width(zt_int texture_handle) {
    zt_borealis_raylib_texture_slot *slot = zt_borealis_raylib_find_texture(texture_handle);
    return slot != NULL ? (zt_int)slot->texture.width : 0;
}

zt_int zt_borealis_raylib_texture_height(zt_int texture_handle) {
    zt_borealis_raylib_texture_slot *slot = zt_borealis_raylib_find_texture(texture_handle);
    return slot != NULL ? (zt_int)slot->texture.height : 0;
}

zt_outcome_void_core_error zt_borealis_raylib_draw_texture(
        zt_int window_id,
        zt_int texture_handle,
        zt_float x,
        zt_float y,
        zt_int tint_r,
        zt_int tint_g,
        zt_int tint_b,
        zt_int tint_a) {
    zt_borealis_raylib_texture_slot *slot;

    if (zt_borealis_is_stub_window(window_id)) {
        return zt_outcome_void_core_error_success();
    }
    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return zt_outcome_void_core_error_failure_message("borealis: desktop window is not open");
    }
    if (zt_borealis_raylib.draw_texture == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: Raylib DrawTexture is not available");
    }
    slot = zt_borealis_raylib_find_texture(texture_handle);
    if (slot == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: unknown Raylib texture handle");
    }

    zt_borealis_raylib.draw_texture(
        slot->texture,
        (int)lround(x),
        (int)lround(y),
        zt_borealis_make_raylib_color(tint_r, tint_g, tint_b, tint_a));
    return zt_outcome_void_core_error_success();
}

zt_outcome_void_core_error zt_borealis_raylib_draw_texture_ex(
        zt_int window_id,
        zt_int texture_handle,
        zt_float x,
        zt_float y,
        zt_float rotation,
        zt_float scale,
        zt_int tint_r,
        zt_int tint_g,
        zt_int tint_b,
        zt_int tint_a) {
    zt_borealis_raylib_texture_slot *slot;
    zt_borealis_raylib_vector2 position;

    if (zt_borealis_is_stub_window(window_id)) {
        return zt_outcome_void_core_error_success();
    }
    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return zt_outcome_void_core_error_failure_message("borealis: desktop window is not open");
    }
    if (zt_borealis_raylib.draw_texture_ex == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: Raylib DrawTextureEx is not available");
    }
    slot = zt_borealis_raylib_find_texture(texture_handle);
    if (slot == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: unknown Raylib texture handle");
    }

    position.x = (float)x;
    position.y = (float)y;
    zt_borealis_raylib.draw_texture_ex(
        slot->texture,
        position,
        (float)rotation,
        (float)scale,
        zt_borealis_make_raylib_color(tint_r, tint_g, tint_b, tint_a));
    return zt_outcome_void_core_error_success();
}

zt_outcome_void_core_error zt_borealis_raylib_init_audio_device(void) {
    if (!zt_borealis_raylib_try_load() || zt_borealis_raylib.init_audio_device == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: Raylib audio support is not available");
    }
    zt_borealis_raylib.init_audio_device();
    return zt_outcome_void_core_error_success();
}

zt_outcome_void_core_error zt_borealis_raylib_close_audio_device(void) {
    if (zt_borealis_raylib.close_audio_device != NULL) {
        zt_borealis_raylib.close_audio_device();
    }
    return zt_outcome_void_core_error_success();
}

zt_bool zt_borealis_raylib_is_audio_device_ready(void) {
    if (!zt_borealis_raylib_try_load() || zt_borealis_raylib.is_audio_device_ready == NULL) {
        return false;
    }
    return zt_borealis_raylib.is_audio_device_ready() ? true : false;
}

zt_outcome_void_core_error zt_borealis_raylib_set_master_volume(zt_float volume) {
    if (!zt_borealis_raylib_try_load() || zt_borealis_raylib.set_master_volume == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: Raylib audio support is not available");
    }
    zt_borealis_raylib.set_master_volume((float)volume);
    return zt_outcome_void_core_error_success();
}

zt_outcome_i64_core_error zt_borealis_raylib_load_sound(const zt_text *path) {
    const char *file_name;
    zt_borealis_raylib_sound sound;
    zt_borealis_raylib_sound_slot *slot;

    if (!zt_borealis_raylib_try_load() || zt_borealis_raylib.load_sound == NULL) {
        return zt_outcome_i64_core_error_failure_message("borealis: Raylib sound support is not available");
    }

    file_name = path != NULL ? zt_text_data(path) : "";
    sound = zt_borealis_raylib.load_sound(file_name);
    if (sound.frameCount == 0) {
        return zt_outcome_i64_core_error_failure_message("borealis: failed to load Raylib sound");
    }

    slot = zt_borealis_raylib_alloc_sound();
    if (slot == NULL) {
        if (zt_borealis_raylib.unload_sound != NULL) {
            zt_borealis_raylib.unload_sound(sound);
        }
        return zt_outcome_i64_core_error_failure_message("borealis: no free Raylib sound slots");
    }

    slot->sound = sound;
    return zt_outcome_i64_core_error_success(slot->handle);
}

zt_outcome_void_core_error zt_borealis_raylib_unload_sound(zt_int sound_handle) {
    zt_borealis_raylib_sound_slot *slot = zt_borealis_raylib_find_sound(sound_handle);
    if (slot == NULL) {
        return zt_outcome_void_core_error_success();
    }
    if (zt_borealis_raylib.unload_sound != NULL) {
        zt_borealis_raylib.unload_sound(slot->sound);
    }
    memset(slot, 0, sizeof(zt_borealis_raylib_sound_slot));
    return zt_outcome_void_core_error_success();
}

zt_outcome_void_core_error zt_borealis_raylib_play_sound(zt_int sound_handle) {
    zt_borealis_raylib_sound_slot *slot = zt_borealis_raylib_find_sound(sound_handle);
    if (slot == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: unknown Raylib sound handle");
    }
    if (zt_borealis_raylib.play_sound == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: Raylib PlaySound is not available");
    }
    zt_borealis_raylib.play_sound(slot->sound);
    return zt_outcome_void_core_error_success();
}

zt_outcome_void_core_error zt_borealis_raylib_stop_sound(zt_int sound_handle) {
    zt_borealis_raylib_sound_slot *slot = zt_borealis_raylib_find_sound(sound_handle);
    if (slot == NULL) {
        return zt_outcome_void_core_error_success();
    }
    if (zt_borealis_raylib.stop_sound != NULL) {
        zt_borealis_raylib.stop_sound(slot->sound);
    }
    return zt_outcome_void_core_error_success();
}

zt_outcome_void_core_error zt_borealis_raylib_set_sound_volume(zt_int sound_handle, zt_float volume) {
    zt_borealis_raylib_sound_slot *slot = zt_borealis_raylib_find_sound(sound_handle);
    if (slot == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: unknown Raylib sound handle");
    }
    if (zt_borealis_raylib.set_sound_volume == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: Raylib SetSoundVolume is not available");
    }
    zt_borealis_raylib.set_sound_volume(slot->sound, (float)volume);
    return zt_outcome_void_core_error_success();
}

zt_outcome_void_core_error zt_borealis_raylib_begin_mode3d(
        zt_int window_id,
        zt_float position_x,
        zt_float position_y,
        zt_float position_z,
        zt_float target_x,
        zt_float target_y,
        zt_float target_z,
        zt_float up_x,
        zt_float up_y,
        zt_float up_z,
        zt_float fov_y,
        zt_int projection) {
    if (zt_borealis_is_stub_window(window_id)) {
        return zt_outcome_void_core_error_success();
    }
    if (!zt_borealis_raylib_try_load() ||
        zt_borealis_raylib.begin_mode3d == NULL ||
        zt_borealis_raylib.end_mode3d == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: Raylib 3D support is not available");
    }
    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return zt_outcome_void_core_error_failure_message("borealis: desktop window is not open");
    }
    if (!zt_borealis_raylib.frame_open) {
        return zt_outcome_void_core_error_failure_message("borealis: frame_begin is required before BeginMode3D");
    }
    if (zt_borealis_raylib.mode3d_open) {
        return zt_outcome_void_core_error_failure_message("borealis: BeginMode3D called twice without EndMode3D");
    }

    zt_borealis_raylib.begin_mode3d(
        zt_borealis_make_raylib_camera3d(
            position_x,
            position_y,
            position_z,
            target_x,
            target_y,
            target_z,
            up_x,
            up_y,
            up_z,
            fov_y,
            projection));
    zt_borealis_raylib.mode3d_open = true;
    return zt_outcome_void_core_error_success();
}

zt_outcome_void_core_error zt_borealis_raylib_end_mode3d(zt_int window_id) {
    if (zt_borealis_is_stub_window(window_id)) {
        return zt_outcome_void_core_error_success();
    }
    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return zt_outcome_void_core_error_failure_message("borealis: desktop window is not open");
    }
    if (!zt_borealis_raylib.mode3d_open) {
        return zt_outcome_void_core_error_success();
    }
    if (zt_borealis_raylib.end_mode3d == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: Raylib EndMode3D is not available");
    }

    zt_borealis_raylib.end_mode3d();
    zt_borealis_raylib.mode3d_open = false;
    return zt_outcome_void_core_error_success();
}

zt_outcome_void_core_error zt_borealis_raylib_draw_cube(
        zt_int window_id,
        zt_float x,
        zt_float y,
        zt_float z,
        zt_float width,
        zt_float height,
        zt_float depth,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a) {
    if (zt_borealis_is_stub_window(window_id)) {
        return zt_outcome_void_core_error_success();
    }
    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return zt_outcome_void_core_error_failure_message("borealis: desktop window is not open");
    }
    if (!zt_borealis_raylib_mode3d_ready(window_id)) {
        return zt_outcome_void_core_error_failure_message("borealis: BeginMode3D is required before 3D drawing");
    }
    if (zt_borealis_raylib.draw_cube_v == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: Raylib DrawCubeV is not available");
    }

    zt_borealis_raylib.draw_cube_v(
        zt_borealis_make_raylib_vector3(x, y, z),
        zt_borealis_make_raylib_vector3(width, height, depth),
        zt_borealis_make_raylib_color(color_r, color_g, color_b, color_a));
    return zt_outcome_void_core_error_success();
}

zt_outcome_void_core_error zt_borealis_raylib_draw_grid(
        zt_int window_id,
        zt_int slices,
        zt_float spacing) {
    if (zt_borealis_is_stub_window(window_id)) {
        return zt_outcome_void_core_error_success();
    }
    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return zt_outcome_void_core_error_failure_message("borealis: desktop window is not open");
    }
    if (!zt_borealis_raylib_mode3d_ready(window_id)) {
        return zt_outcome_void_core_error_failure_message("borealis: BeginMode3D is required before 3D drawing");
    }
    if (zt_borealis_raylib.draw_grid == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: Raylib DrawGrid is not available");
    }

    zt_borealis_raylib.draw_grid((int)slices, (float)spacing);
    return zt_outcome_void_core_error_success();
}

zt_outcome_i64_core_error zt_borealis_raylib_load_model(const zt_text *path) {
    const char *file_name;
    zt_borealis_raylib_model model;
    zt_borealis_raylib_model_slot *slot;

    if (!zt_borealis_raylib_try_load() || zt_borealis_raylib.load_model == NULL) {
        return zt_outcome_i64_core_error_failure_message("borealis: Raylib model support is not available");
    }

    file_name = path != NULL ? zt_text_data(path) : "";
    model = zt_borealis_raylib.load_model(file_name);
    if (!zt_borealis_raylib_model_loaded(model)) {
        return zt_outcome_i64_core_error_failure_message("borealis: failed to load Raylib model");
    }

    slot = zt_borealis_raylib_alloc_model();
    if (slot == NULL) {
        if (zt_borealis_raylib.unload_model != NULL) {
            zt_borealis_raylib.unload_model(model);
        }
        return zt_outcome_i64_core_error_failure_message("borealis: no free Raylib model slots");
    }

    slot->model = model;
    return zt_outcome_i64_core_error_success(slot->handle);
}

zt_outcome_void_core_error zt_borealis_raylib_unload_model(zt_int model_handle) {
    zt_borealis_raylib_model_slot *slot = zt_borealis_raylib_find_model(model_handle);
    if (slot == NULL) {
        return zt_outcome_void_core_error_success();
    }
    if (zt_borealis_raylib.unload_model != NULL) {
        zt_borealis_raylib.unload_model(slot->model);
    }
    memset(slot, 0, sizeof(zt_borealis_raylib_model_slot));
    return zt_outcome_void_core_error_success();
}

zt_outcome_void_core_error zt_borealis_raylib_draw_model(
        zt_int window_id,
        zt_int model_handle,
        zt_float position_x,
        zt_float position_y,
        zt_float position_z,
        zt_float rotation_x,
        zt_float rotation_y,
        zt_float rotation_z,
        zt_float scale_x,
        zt_float scale_y,
        zt_float scale_z,
        zt_int tint_r,
        zt_int tint_g,
        zt_int tint_b,
        zt_int tint_a) {
    zt_borealis_raylib_model_slot *slot;
    zt_borealis_raylib_vector3 position;
    zt_borealis_raylib_vector3 rotation_axis;
    zt_borealis_raylib_vector3 scale;
    float rotation_angle;

    if (zt_borealis_is_stub_window(window_id)) {
        return zt_outcome_void_core_error_success();
    }
    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return zt_outcome_void_core_error_failure_message("borealis: desktop window is not open");
    }
    if (!zt_borealis_raylib_mode3d_ready(window_id)) {
        return zt_outcome_void_core_error_failure_message("borealis: BeginMode3D is required before 3D drawing");
    }
    if (zt_borealis_raylib.draw_model_ex == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: Raylib DrawModelEx is not available");
    }
    slot = zt_borealis_raylib_find_model(model_handle);
    if (slot == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: unknown Raylib model handle");
    }

    position = zt_borealis_make_raylib_vector3(position_x, position_y, position_z);
    rotation_axis = zt_borealis_make_raylib_vector3(rotation_x, rotation_y, rotation_z);
    scale = zt_borealis_make_raylib_vector3(scale_x, scale_y, scale_z);
    if (fabsf(scale.x) < 0.0001f &&
        fabsf(scale.y) < 0.0001f &&
        fabsf(scale.z) < 0.0001f) {
        scale = zt_borealis_make_raylib_vector3(1.0, 1.0, 1.0);
    }

    rotation_angle = sqrtf(
        (rotation_axis.x * rotation_axis.x) +
        (rotation_axis.y * rotation_axis.y) +
        (rotation_axis.z * rotation_axis.z));
    if (rotation_angle < 0.0001f) {
        rotation_axis = zt_borealis_make_raylib_vector3(0.0, 1.0, 0.0);
        rotation_angle = 0.0f;
    } else {
        rotation_axis.x /= rotation_angle;
        rotation_axis.y /= rotation_angle;
        rotation_axis.z /= rotation_angle;
    }

    zt_borealis_raylib.draw_model_ex(
        slot->model,
        position,
        rotation_axis,
        rotation_angle,
        scale,
        zt_borealis_make_raylib_color(tint_r, tint_g, tint_b, tint_a));
    return zt_outcome_void_core_error_success();
}

zt_outcome_void_core_error zt_borealis_raylib_draw_billboard(
        zt_int window_id,
        zt_int texture_handle,
        zt_float camera_position_x,
        zt_float camera_position_y,
        zt_float camera_position_z,
        zt_float camera_target_x,
        zt_float camera_target_y,
        zt_float camera_target_z,
        zt_float camera_up_x,
        zt_float camera_up_y,
        zt_float camera_up_z,
        zt_float camera_fov_y,
        zt_int camera_projection,
        zt_float position_x,
        zt_float position_y,
        zt_float position_z,
        zt_float size_x,
        zt_float size_y,
        zt_int tint_r,
        zt_int tint_g,
        zt_int tint_b,
        zt_int tint_a) {
    zt_borealis_raylib_texture_slot *slot;
    zt_borealis_raylib_camera3d camera;
    zt_borealis_raylib_vector3 position;
    zt_borealis_raylib_vector2 size;
    zt_borealis_raylib_rectangle source;

    if (zt_borealis_is_stub_window(window_id)) {
        return zt_outcome_void_core_error_success();
    }
    if (!zt_borealis_raylib.window_open || window_id != zt_borealis_raylib.window_id) {
        return zt_outcome_void_core_error_failure_message("borealis: desktop window is not open");
    }
    if (!zt_borealis_raylib_mode3d_ready(window_id)) {
        return zt_outcome_void_core_error_failure_message("borealis: BeginMode3D is required before 3D drawing");
    }
    if (zt_borealis_raylib.draw_billboard_rec == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: Raylib DrawBillboardRec is not available");
    }
    slot = zt_borealis_raylib_find_texture(texture_handle);
    if (slot == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: unknown Raylib texture handle");
    }

    camera = zt_borealis_make_raylib_camera3d(
        camera_position_x,
        camera_position_y,
        camera_position_z,
        camera_target_x,
        camera_target_y,
        camera_target_z,
        camera_up_x,
        camera_up_y,
        camera_up_z,
        camera_fov_y,
        camera_projection);
    position = zt_borealis_make_raylib_vector3(position_x, position_y, position_z);
    size.x = (float)size_x;
    size.y = (float)size_y;
    source = zt_borealis_make_raylib_rectangle(
        0.0,
        0.0,
        (zt_float)slot->texture.width,
        (zt_float)slot->texture.height);

    zt_borealis_raylib.draw_billboard_rec(
        camera,
        slot->texture,
        source,
        position,
        size,
        zt_borealis_make_raylib_color(tint_r, tint_g, tint_b, tint_a));
    return zt_outcome_void_core_error_success();
}

zt_float zt_borealis_raylib_vector2_length(zt_float x, zt_float y) {
    return sqrt((x * x) + (y * y));
}

zt_float zt_borealis_raylib_vector2_distance(zt_float ax, zt_float ay, zt_float bx, zt_float by) {
    zt_float dx = bx - ax;
    zt_float dy = by - ay;
    return sqrt((dx * dx) + (dy * dy));
}

zt_float zt_borealis_raylib_lerp(zt_float start, zt_float finish, zt_float amount) {
    return start + ((finish - start) * amount);
}

zt_float zt_borealis_raylib_ease_linear(zt_float t, zt_float b, zt_float c, zt_float d) {
    if (d == 0.0) return b + c;
    return (c * t / d) + b;
}

zt_float zt_borealis_raylib_ease_sine_in(zt_float t, zt_float b, zt_float c, zt_float d) {
    const zt_float half_pi = 1.57079632679489661923;
    if (d == 0.0) return b + c;
    return (-c * cos(t / d * half_pi)) + c + b;
}

zt_float zt_borealis_raylib_ease_sine_out(zt_float t, zt_float b, zt_float c, zt_float d) {
    const zt_float half_pi = 1.57079632679489661923;
    if (d == 0.0) return b + c;
    return (c * sin(t / d * half_pi)) + b;
}

zt_float zt_borealis_raylib_ease_sine_in_out(zt_float t, zt_float b, zt_float c, zt_float d) {
    const zt_float pi = 3.14159265358979323846;
    if (d == 0.0) return b + c;
    return (-c / 2.0 * (cos(pi * t / d) - 1.0)) + b;
}

zt_float zt_borealis_raylib_ease_quad_in(zt_float t, zt_float b, zt_float c, zt_float d) {
    if (d == 0.0) return b + c;
    t = t / d;
    return (c * t * t) + b;
}

zt_float zt_borealis_raylib_ease_quad_out(zt_float t, zt_float b, zt_float c, zt_float d) {
    if (d == 0.0) return b + c;
    t = t / d;
    return (-c * t * (t - 2.0)) + b;
}

zt_float zt_borealis_raylib_ease_quad_in_out(zt_float t, zt_float b, zt_float c, zt_float d) {
    if (d == 0.0) return b + c;
    t = t / (d / 2.0);
    if (t < 1.0) {
        return (c / 2.0 * t * t) + b;
    }
    t = t - 1.0;
    return (-c / 2.0 * ((t * (t - 2.0)) - 1.0)) + b;
}

static const zt_borealis_desktop_api zt_borealis_raylib_desktop_api = {
    zt_borealis_raylib_open_window,
    zt_borealis_raylib_close_window,
    zt_borealis_raylib_window_should_close,
    zt_borealis_raylib_begin_frame,
    zt_borealis_raylib_end_frame,
    zt_borealis_raylib_draw_rect,
    zt_borealis_raylib_draw_line,
    zt_borealis_raylib_draw_rect_outline,
    zt_borealis_raylib_draw_circle,
    zt_borealis_raylib_draw_circle_outline,
    zt_borealis_raylib_draw_text,
    zt_borealis_raylib_is_key_down,
    zt_borealis_raylib_is_key_pressed,
    zt_borealis_raylib_is_key_released
};

static void zt_borealis_try_register_builtin_desktop_api(void) {
    if (zt_borealis_get_desktop_api() != NULL) {
        return;
    }

    if (zt_borealis_raylib_try_load()) {
        zt_borealis_set_desktop_api(&zt_borealis_raylib_desktop_api);
    }
}


static zt_borealis_key_state *zt_borealis_find_key_state(
        zt_borealis_window_state *window_state,
        zt_int input_code,
        zt_bool create_if_missing) {
    size_t index;
    zt_borealis_key_state *first_free = NULL;

    if (window_state == NULL) {
        return NULL;
    }

    for (index = 0; index < ZT_BOREALIS_MAX_KEYS_PER_WINDOW; index += 1) {
        zt_borealis_key_state *key = &window_state->keys[index];
        if (key->used && key->input_code == input_code) {
            return key;
        }
        if (!key->used && first_free == NULL) {
            first_free = key;
        }
    }

    if (!create_if_missing || first_free == NULL) {
        return NULL;
    }

    memset(first_free, 0, sizeof(zt_borealis_key_state));
    first_free->used = true;
    first_free->input_code = input_code;
    return first_free;
}

zt_outcome_i64_core_error zt_borealis_open_window(const zt_text *title, zt_int width, zt_int height, zt_int target_fps, zt_int backend_id) {
    const zt_borealis_desktop_api *desktop_api;

    if (backend_id == ZT_BOREALIS_BACKEND_STUB) {
        return zt_borealis_open_stub_window();
    }

    zt_borealis_try_register_builtin_desktop_api();
    desktop_api = zt_borealis_get_desktop_api();

    if (desktop_api != NULL && desktop_api->open_window != NULL) {
        return desktop_api->open_window(title, width, height, target_fps, backend_id);
    }

    return zt_borealis_open_stub_window();
}

zt_outcome_void_core_error zt_borealis_close_window(zt_int window_id) {
    const zt_borealis_desktop_api *desktop_api = zt_borealis_get_desktop_api();

    if (zt_borealis_is_stub_window(window_id)) {
        zt_borealis_free_window_state(window_id);
        return zt_outcome_void_core_error_success();
    }

    if (desktop_api != NULL && desktop_api->close_window != NULL) {
        return desktop_api->close_window(window_id);
    }

    return zt_borealis_backend_missing_void();
}

zt_bool zt_borealis_window_should_close(zt_int window_id) {
    const zt_borealis_desktop_api *desktop_api = zt_borealis_get_desktop_api();

    if (zt_borealis_is_stub_window(window_id)) {
        return true;
    }

    if (desktop_api != NULL && desktop_api->window_should_close != NULL) {
        return desktop_api->window_should_close(window_id);
    }

    return true;
}

zt_outcome_void_core_error zt_borealis_begin_frame(zt_int window_id, zt_int clear_r, zt_int clear_g, zt_int clear_b, zt_int clear_a) {
    const zt_borealis_desktop_api *desktop_api = zt_borealis_get_desktop_api();

    if (zt_borealis_is_stub_window(window_id)) {
        size_t index;
        zt_borealis_window_state *window_state = zt_borealis_alloc_window_state(window_id);
        if (window_state == NULL) {
            return zt_outcome_void_core_error_failure_message("borealis: no free input window slots");
        }
        for (index = 0; index < ZT_BOREALIS_MAX_KEYS_PER_WINDOW; index += 1) {
            zt_borealis_key_state *key = &window_state->keys[index];
            if (!key->used) {
                continue;
            }
            key->prev_down = key->down;
            key->down = key->raw_down;
        }
        return zt_outcome_void_core_error_success();
    }

    if (desktop_api != NULL && desktop_api->begin_frame != NULL) {
        return desktop_api->begin_frame(window_id, clear_r, clear_g, clear_b, clear_a);
    }

    return zt_borealis_backend_missing_void();
}

zt_outcome_void_core_error zt_borealis_end_frame(zt_int window_id) {
    const zt_borealis_desktop_api *desktop_api = zt_borealis_get_desktop_api();

    if (zt_borealis_is_stub_window(window_id)) {
        return zt_outcome_void_core_error_success();
    }

    if (desktop_api != NULL && desktop_api->end_frame != NULL) {
        return desktop_api->end_frame(window_id);
    }

    return zt_borealis_backend_missing_void();
}

zt_outcome_void_core_error zt_borealis_draw_rect(
        zt_int window_id,
        zt_float x,
        zt_float y,
        zt_float width,
        zt_float height,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a) {
    const zt_borealis_desktop_api *desktop_api = zt_borealis_get_desktop_api();

    if (zt_borealis_is_stub_window(window_id)) {
        return zt_outcome_void_core_error_success();
    }

    if (desktop_api != NULL && desktop_api->draw_rect != NULL) {
        return desktop_api->draw_rect(window_id, x, y, width, height, color_r, color_g, color_b, color_a);
    }

    return zt_borealis_backend_missing_void();
}

zt_outcome_void_core_error zt_borealis_draw_line(
        zt_int window_id,
        zt_float x1,
        zt_float y1,
        zt_float x2,
        zt_float y2,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a) {
    const zt_borealis_desktop_api *desktop_api = zt_borealis_get_desktop_api();

    if (zt_borealis_is_stub_window(window_id)) {
        return zt_outcome_void_core_error_success();
    }

    if (desktop_api != NULL && desktop_api->draw_line != NULL) {
        return desktop_api->draw_line(window_id, x1, y1, x2, y2, color_r, color_g, color_b, color_a);
    }

    return zt_borealis_backend_missing_void();
}

zt_outcome_void_core_error zt_borealis_draw_rect_outline(
        zt_int window_id,
        zt_float x,
        zt_float y,
        zt_float width,
        zt_float height,
        zt_float thickness,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a) {
    const zt_borealis_desktop_api *desktop_api = zt_borealis_get_desktop_api();

    if (zt_borealis_is_stub_window(window_id)) {
        return zt_outcome_void_core_error_success();
    }

    if (desktop_api != NULL && desktop_api->draw_rect_outline != NULL) {
        return desktop_api->draw_rect_outline(window_id, x, y, width, height, thickness, color_r, color_g, color_b, color_a);
    }

    return zt_borealis_backend_missing_void();
}

zt_outcome_void_core_error zt_borealis_draw_circle(
        zt_int window_id,
        zt_float x,
        zt_float y,
        zt_float radius,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a) {
    const zt_borealis_desktop_api *desktop_api = zt_borealis_get_desktop_api();

    if (zt_borealis_is_stub_window(window_id)) {
        return zt_outcome_void_core_error_success();
    }

    if (desktop_api != NULL && desktop_api->draw_circle != NULL) {
        return desktop_api->draw_circle(window_id, x, y, radius, color_r, color_g, color_b, color_a);
    }

    return zt_borealis_backend_missing_void();
}

zt_outcome_void_core_error zt_borealis_draw_circle_outline(
        zt_int window_id,
        zt_float x,
        zt_float y,
        zt_float radius,
        zt_float thickness,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a) {
    const zt_borealis_desktop_api *desktop_api = zt_borealis_get_desktop_api();

    if (zt_borealis_is_stub_window(window_id)) {
        return zt_outcome_void_core_error_success();
    }

    if (desktop_api != NULL && desktop_api->draw_circle_outline != NULL) {
        return desktop_api->draw_circle_outline(window_id, x, y, radius, thickness, color_r, color_g, color_b, color_a);
    }

    return zt_borealis_backend_missing_void();
}

zt_outcome_void_core_error zt_borealis_draw_text(
        zt_int window_id,
        const zt_text *value,
        zt_int x,
        zt_int y,
        zt_int size,
        zt_int color_r,
        zt_int color_g,
        zt_int color_b,
        zt_int color_a) {
    const zt_borealis_desktop_api *desktop_api = zt_borealis_get_desktop_api();

    if (zt_borealis_is_stub_window(window_id)) {
        return zt_outcome_void_core_error_success();
    }

    if (desktop_api != NULL && desktop_api->draw_text != NULL) {
        return desktop_api->draw_text(window_id, value, x, y, size, color_r, color_g, color_b, color_a);
    }

    return zt_borealis_backend_missing_void();
}

zt_bool zt_borealis_is_key_down(zt_int window_id, zt_int input_code) {
    const zt_borealis_desktop_api *desktop_api = zt_borealis_get_desktop_api();
    zt_borealis_window_state *window_state;
    zt_borealis_key_state *key;

    if (!zt_borealis_is_stub_window(window_id)) {
        if (desktop_api != NULL && desktop_api->is_key_down != NULL) {
            return desktop_api->is_key_down(window_id, input_code);
        }
        return false;
    }

    window_state = zt_borealis_find_window_state(window_id);
    if (window_state == NULL) {
        return false;
    }

    key = zt_borealis_find_key_state(window_state, input_code, false);
    if (key == NULL) {
        return false;
    }
    return key->down;
}

zt_bool zt_borealis_is_key_pressed(zt_int window_id, zt_int input_code) {
    const zt_borealis_desktop_api *desktop_api = zt_borealis_get_desktop_api();
    zt_borealis_window_state *window_state;
    zt_borealis_key_state *key;

    if (!zt_borealis_is_stub_window(window_id)) {
        if (desktop_api != NULL && desktop_api->is_key_pressed != NULL) {
            return desktop_api->is_key_pressed(window_id, input_code);
        }
        return false;
    }

    window_state = zt_borealis_find_window_state(window_id);
    if (window_state == NULL) {
        return false;
    }

    key = zt_borealis_find_key_state(window_state, input_code, false);
    if (key == NULL) {
        return false;
    }
    return key->down && !key->prev_down;
}

zt_bool zt_borealis_is_key_released(zt_int window_id, zt_int input_code) {
    const zt_borealis_desktop_api *desktop_api = zt_borealis_get_desktop_api();
    zt_borealis_window_state *window_state;
    zt_borealis_key_state *key;

    if (!zt_borealis_is_stub_window(window_id)) {
        if (desktop_api != NULL && desktop_api->is_key_released != NULL) {
            return desktop_api->is_key_released(window_id, input_code);
        }
        return false;
    }

    window_state = zt_borealis_find_window_state(window_id);
    if (window_state == NULL) {
        return false;
    }

    key = zt_borealis_find_key_state(window_state, input_code, false);
    if (key == NULL) {
        return false;
    }
    return !key->down && key->prev_down;
}

zt_outcome_void_core_error zt_borealis_stub_set_key_down(zt_int window_id, zt_int input_code, zt_bool is_down) {
    zt_borealis_window_state *window_state;
    zt_borealis_key_state *key;

    if (!zt_borealis_is_stub_window(window_id)) {
        return zt_borealis_backend_missing_void();
    }

    window_state = zt_borealis_alloc_window_state(window_id);
    if (window_state == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: no free input window slots");
    }

    key = zt_borealis_find_key_state(window_state, input_code, true);
    if (key == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: no free key slots in window input state");
    }

    key->raw_down = is_down;
    return zt_outcome_void_core_error_success();
}

zt_outcome_void_core_error zt_borealis_stub_reset_input(zt_int window_id) {
    zt_borealis_window_state *window_state;

    if (!zt_borealis_is_stub_window(window_id)) {
        return zt_borealis_backend_missing_void();
    }

    window_state = zt_borealis_alloc_window_state(window_id);
    if (window_state == NULL) {
        return zt_outcome_void_core_error_failure_message("borealis: no free input window slots");
    }
    memset(window_state->keys, 0, sizeof(window_state->keys));
    return zt_outcome_void_core_error_success();
}

