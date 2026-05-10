zt_text *zt_encoding_hex_encode(const zt_bytes *data) {
    static const char *hex = "0123456789abcdef";
    char *buffer;
    size_t i;
    zt_text *result;

    zt_runtime_require_bytes(data, "encoding.hex_encode requires bytes");
    buffer = (char *)malloc((data->len * 2u) + 1u);
    if (buffer == NULL) zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate hex output");

    for (i = 0; i < data->len; i += 1) {
        buffer[i * 2u] = hex[(data->data[i] >> 4u) & 0x0fu];
        buffer[(i * 2u) + 1u] = hex[data->data[i] & 0x0fu];
    }
    buffer[data->len * 2u] = '\0';
    result = zt_text_from_utf8(buffer, data->len * 2u);
    free(buffer);
    return result;
}

static int zt_encoding_hex_value(unsigned char ch) {
    if (ch >= '0' && ch <= '9') return (int)(ch - '0');
    if (ch >= 'a' && ch <= 'f') return (int)(ch - 'a') + 10;
    if (ch >= 'A' && ch <= 'F') return (int)(ch - 'A') + 10;
    return -1;
}

zt_outcome_bytes_core_error zt_encoding_hex_decode(const zt_text *text_value) {
    uint8_t *buffer;
    size_t clean_len = 0;
    size_t i;
    size_t out = 0;
    zt_bytes *bytes;
    zt_outcome_bytes_core_error result;

    zt_runtime_require_text(text_value, "encoding.hex_decode requires text");
    for (i = 0; i < text_value->len; i += 1) {
        unsigned char ch = (unsigned char)text_value->data[i];
        if (!isspace(ch) && ch != '_') clean_len += 1;
    }
    if ((clean_len % 2u) != 0) {
        return zt_outcome_bytes_core_error_failure_message("encoding.hex_decode expects an even hex digit count");
    }

    buffer = (uint8_t *)malloc(clean_len / 2u + 1u);
    if (buffer == NULL) zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate hex decode buffer");

    {
        int high = -1;
        for (i = 0; i < text_value->len; i += 1) {
            unsigned char ch = (unsigned char)text_value->data[i];
            int value;
            if (isspace(ch) || ch == '_') continue;
            value = zt_encoding_hex_value(ch);
            if (value < 0) {
                free(buffer);
                return zt_outcome_bytes_core_error_failure_message("encoding.hex_decode found invalid hex digit");
            }
            if (high < 0) {
                high = value;
            } else {
                buffer[out++] = (uint8_t)((high << 4) | value);
                high = -1;
            }
        }
    }

    bytes = zt_bytes_from_array(buffer, out);
    free(buffer);
    result = zt_outcome_bytes_core_error_success(bytes);
    zt_release(bytes);
    return result;
}

zt_text *zt_encoding_base64_encode(const zt_bytes *data) {
    static const char table[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    char *buffer;
    size_t out_len;
    size_t i;
    size_t out = 0;
    zt_text *result;

    zt_runtime_require_bytes(data, "encoding.base64_encode requires bytes");
    out_len = ((data->len + 2u) / 3u) * 4u;
    buffer = (char *)malloc(out_len + 1u);
    if (buffer == NULL) zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate base64 output");

    for (i = 0; i < data->len; i += 3u) {
        uint32_t a = data->data[i];
        uint32_t b = (i + 1u < data->len) ? data->data[i + 1u] : 0u;
        uint32_t c = (i + 2u < data->len) ? data->data[i + 2u] : 0u;
        uint32_t triple = (a << 16u) | (b << 8u) | c;
        buffer[out++] = table[(triple >> 18u) & 0x3fu];
        buffer[out++] = table[(triple >> 12u) & 0x3fu];
        buffer[out++] = (i + 1u < data->len) ? table[(triple >> 6u) & 0x3fu] : '=';
        buffer[out++] = (i + 2u < data->len) ? table[triple & 0x3fu] : '=';
    }

    buffer[out] = '\0';
    result = zt_text_from_utf8(buffer, out);
    free(buffer);
    return result;
}

static int zt_encoding_base64_value(unsigned char ch) {
    if (ch >= 'A' && ch <= 'Z') return (int)(ch - 'A');
    if (ch >= 'a' && ch <= 'z') return (int)(ch - 'a') + 26;
    if (ch >= '0' && ch <= '9') return (int)(ch - '0') + 52;
    if (ch == '+') return 62;
    if (ch == '/') return 63;
    return -1;
}

zt_outcome_bytes_core_error zt_encoding_base64_decode(const zt_text *text_value) {
    uint8_t *buffer;
    size_t capacity;
    size_t out = 0;
    size_t i;
    int quad[4];
    size_t q = 0;
    zt_bytes *bytes;
    zt_outcome_bytes_core_error result;

    zt_runtime_require_text(text_value, "encoding.base64_decode requires text");
    capacity = (text_value->len / 4u + 1u) * 3u;
    buffer = (uint8_t *)malloc(capacity);
    if (buffer == NULL) zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate base64 decode buffer");

    for (i = 0; i < text_value->len; i += 1) {
        unsigned char ch = (unsigned char)text_value->data[i];
        if (isspace(ch)) continue;
        if (ch == '=') {
            quad[q++] = -2;
        } else {
            int value = zt_encoding_base64_value(ch);
            if (value < 0) {
                free(buffer);
                return zt_outcome_bytes_core_error_failure_message("encoding.base64_decode found invalid character");
            }
            quad[q++] = value;
        }
        if (q == 4u) {
            uint32_t triple;
            if (quad[0] < 0 || quad[1] < 0) {
                free(buffer);
                return zt_outcome_bytes_core_error_failure_message("encoding.base64_decode found invalid padding");
            }
            triple = ((uint32_t)quad[0] << 18u) | ((uint32_t)quad[1] << 12u) |
                     ((uint32_t)(quad[2] < 0 ? 0 : quad[2]) << 6u) |
                     (uint32_t)(quad[3] < 0 ? 0 : quad[3]);
            buffer[out++] = (uint8_t)((triple >> 16u) & 0xffu);
            if (quad[2] != -2) buffer[out++] = (uint8_t)((triple >> 8u) & 0xffu);
            if (quad[3] != -2) buffer[out++] = (uint8_t)(triple & 0xffu);
            q = 0;
        }
    }

    if (q != 0u) {
        free(buffer);
        return zt_outcome_bytes_core_error_failure_message("encoding.base64_decode found incomplete input");
    }

    bytes = zt_bytes_from_array(buffer, out);
    free(buffer);
    result = zt_outcome_bytes_core_error_success(bytes);
    zt_release(bytes);
    return result;
}

static zt_text *zt_hash_hex_from_bytes(const uint8_t *bytes, size_t len) {
    static const char *hex = "0123456789abcdef";
    char *buffer;
    size_t i;
    zt_text *result;

    buffer = (char *)malloc((len * 2u) + 1u);
    if (buffer == NULL) zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate hash text");
    for (i = 0; i < len; i += 1) {
        buffer[i * 2u] = hex[(bytes[i] >> 4u) & 0x0fu];
        buffer[(i * 2u) + 1u] = hex[bytes[i] & 0x0fu];
    }
    buffer[len * 2u] = '\0';
    result = zt_text_from_utf8(buffer, len * 2u);
    free(buffer);
    return result;
}

#define ZT_SHA256_ROTR(x, n) (((x) >> (n)) | ((x) << (32u - (n))))

static void zt_hash_sha256_digest(const uint8_t *data, size_t len, uint8_t out[32]) {
    static const uint32_t k[64] = {
        0x428a2f98u,0x71374491u,0xb5c0fbcfu,0xe9b5dba5u,0x3956c25bu,0x59f111f1u,0x923f82a4u,0xab1c5ed5u,
        0xd807aa98u,0x12835b01u,0x243185beu,0x550c7dc3u,0x72be5d74u,0x80deb1feu,0x9bdc06a7u,0xc19bf174u,
        0xe49b69c1u,0xefbe4786u,0x0fc19dc6u,0x240ca1ccu,0x2de92c6fu,0x4a7484aau,0x5cb0a9dcu,0x76f988dau,
        0x983e5152u,0xa831c66du,0xb00327c8u,0xbf597fc7u,0xc6e00bf3u,0xd5a79147u,0x06ca6351u,0x14292967u,
        0x27b70a85u,0x2e1b2138u,0x4d2c6dfcu,0x53380d13u,0x650a7354u,0x766a0abbu,0x81c2c92eu,0x92722c85u,
        0xa2bfe8a1u,0xa81a664bu,0xc24b8b70u,0xc76c51a3u,0xd192e819u,0xd6990624u,0xf40e3585u,0x106aa070u,
        0x19a4c116u,0x1e376c08u,0x2748774cu,0x34b0bcb5u,0x391c0cb3u,0x4ed8aa4au,0x5b9cca4fu,0x682e6ff3u,
        0x748f82eeu,0x78a5636fu,0x84c87814u,0x8cc70208u,0x90befffau,0xa4506cebu,0xbef9a3f7u,0xc67178f2u
    };
    uint32_t h[8] = {0x6a09e667u,0xbb67ae85u,0x3c6ef372u,0xa54ff53au,0x510e527fu,0x9b05688cu,0x1f83d9abu,0x5be0cd19u};
    uint64_t bit_len = (uint64_t)len * 8u;
    size_t new_len = len + 1u;
    uint8_t *msg;
    size_t offset;
    size_t i;

    while ((new_len % 64u) != 56u) new_len += 1u;
    msg = (uint8_t *)calloc(new_len + 8u, 1u);
    if (msg == NULL) zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate sha256 buffer");
    if (len > 0) memcpy(msg, data, len);
    msg[len] = 0x80u;
    for (i = 0; i < 8u; i += 1u) msg[new_len + i] = (uint8_t)(bit_len >> (56u - (i * 8u)));

    for (offset = 0; offset < new_len + 8u; offset += 64u) {
        uint32_t w[64];
        uint32_t a,b,c,d,e,f,g,hh;
        for (i = 0; i < 16u; i += 1u) {
            size_t j = offset + (i * 4u);
            w[i] = ((uint32_t)msg[j] << 24u) | ((uint32_t)msg[j + 1u] << 16u) | ((uint32_t)msg[j + 2u] << 8u) | msg[j + 3u];
        }
        for (i = 16u; i < 64u; i += 1u) {
            uint32_t s0 = ZT_SHA256_ROTR(w[i - 15u], 7u) ^ ZT_SHA256_ROTR(w[i - 15u], 18u) ^ (w[i - 15u] >> 3u);
            uint32_t s1 = ZT_SHA256_ROTR(w[i - 2u], 17u) ^ ZT_SHA256_ROTR(w[i - 2u], 19u) ^ (w[i - 2u] >> 10u);
            w[i] = w[i - 16u] + s0 + w[i - 7u] + s1;
        }
        a = h[0]; b = h[1]; c = h[2]; d = h[3]; e = h[4]; f = h[5]; g = h[6]; hh = h[7];
        for (i = 0; i < 64u; i += 1u) {
            uint32_t s1 = ZT_SHA256_ROTR(e, 6u) ^ ZT_SHA256_ROTR(e, 11u) ^ ZT_SHA256_ROTR(e, 25u);
            uint32_t ch = (e & f) ^ ((~e) & g);
            uint32_t temp1 = hh + s1 + ch + k[i] + w[i];
            uint32_t s0 = ZT_SHA256_ROTR(a, 2u) ^ ZT_SHA256_ROTR(a, 13u) ^ ZT_SHA256_ROTR(a, 22u);
            uint32_t maj = (a & b) ^ (a & c) ^ (b & c);
            uint32_t temp2 = s0 + maj;
            hh = g; g = f; f = e; e = d + temp1; d = c; c = b; b = a; a = temp1 + temp2;
        }
        h[0] += a; h[1] += b; h[2] += c; h[3] += d; h[4] += e; h[5] += f; h[6] += g; h[7] += hh;
    }
    free(msg);
    for (i = 0; i < 8u; i += 1u) {
        out[i * 4u] = (uint8_t)(h[i] >> 24u);
        out[i * 4u + 1u] = (uint8_t)(h[i] >> 16u);
        out[i * 4u + 2u] = (uint8_t)(h[i] >> 8u);
        out[i * 4u + 3u] = (uint8_t)h[i];
    }
}

#define ZT_MD5_LEFTROTATE(x, c) (((x) << (c)) | ((x) >> (32u - (c))))

static void zt_hash_md5_digest(const uint8_t *initial_msg, size_t initial_len, uint8_t digest[16]) {
    static const uint32_t r[] = {
        7,12,17,22,7,12,17,22,7,12,17,22,7,12,17,22,
        5,9,14,20,5,9,14,20,5,9,14,20,5,9,14,20,
        4,11,16,23,4,11,16,23,4,11,16,23,4,11,16,23,
        6,10,15,21,6,10,15,21,6,10,15,21,6,10,15,21
    };
    uint32_t h0 = 0x67452301u, h1 = 0xefcdab89u, h2 = 0x98badcfeu, h3 = 0x10325476u;
    uint64_t bits_len = (uint64_t)initial_len * 8u;
    size_t new_len = initial_len + 1u;
    uint8_t *msg;
    size_t offset;
    size_t i;

    while ((new_len % 64u) != 56u) new_len += 1u;
    msg = (uint8_t *)calloc(new_len + 8u, 1u);
    if (msg == NULL) zt_runtime_error(ZT_ERR_PLATFORM, "failed to allocate md5 buffer");
    if (initial_len > 0) memcpy(msg, initial_msg, initial_len);
    msg[initial_len] = 0x80u;
    for (i = 0; i < 8u; i += 1u) msg[new_len + i] = (uint8_t)(bits_len >> (8u * i));

    for (offset = 0; offset < new_len; offset += 64u) {
        uint32_t *w = (uint32_t *)(void *)(msg + offset);
        uint32_t a = h0, b = h1, c = h2, d = h3;
        for (i = 0; i < 64u; i += 1u) {
            uint32_t f, g;
            uint32_t k = (uint32_t)(fabs(sin((double)i + 1.0)) * 4294967296.0);
            uint32_t temp;
            if (i < 16u) {
                f = (b & c) | ((~b) & d);
                g = (uint32_t)i;
            } else if (i < 32u) {
                f = (d & b) | ((~d) & c);
                g = (5u * (uint32_t)i + 1u) % 16u;
            } else if (i < 48u) {
                f = b ^ c ^ d;
                g = (3u * (uint32_t)i + 5u) % 16u;
            } else {
                f = c ^ (b | (~d));
                g = (7u * (uint32_t)i) % 16u;
            }
            temp = d;
            d = c;
            c = b;
            b = b + ZT_MD5_LEFTROTATE(a + f + k + w[g], r[i]);
            a = temp;
        }
        h0 += a; h1 += b; h2 += c; h3 += d;
    }
    free(msg);
    {
        uint32_t h[4] = {h0, h1, h2, h3};
        for (i = 0; i < 4u; i += 1u) {
            digest[i * 4u] = (uint8_t)h[i];
            digest[i * 4u + 1u] = (uint8_t)(h[i] >> 8u);
            digest[i * 4u + 2u] = (uint8_t)(h[i] >> 16u);
            digest[i * 4u + 3u] = (uint8_t)(h[i] >> 24u);
        }
    }
}

zt_text *zt_hash_sha256_bytes(const zt_bytes *value) {
    uint8_t digest[32];
    zt_runtime_require_bytes(value, "hash.sha256_bytes requires bytes");
    zt_hash_sha256_digest(value->data, value->len, digest);
    return zt_hash_hex_from_bytes(digest, sizeof(digest));
}

zt_text *zt_hash_md5_bytes(const zt_bytes *value) {
    uint8_t digest[16];
    zt_runtime_require_bytes(value, "hash.md5_bytes requires bytes");
    zt_hash_md5_digest(value->data, value->len, digest);
    return zt_hash_hex_from_bytes(digest, sizeof(digest));
}

zt_text *zt_hash_sha256_text(const zt_text *value) {
    uint8_t digest[32];
    zt_runtime_require_text(value, "hash.sha256 requires text");
    zt_hash_sha256_digest((const uint8_t *)value->data, value->len, digest);
    return zt_hash_hex_from_bytes(digest, sizeof(digest));
}

zt_text *zt_hash_md5_text(const zt_text *value) {
    uint8_t digest[16];
    zt_runtime_require_text(value, "hash.md5 requires text");
    zt_hash_md5_digest((const uint8_t *)value->data, value->len, digest);
    return zt_hash_hex_from_bytes(digest, sizeof(digest));
}
