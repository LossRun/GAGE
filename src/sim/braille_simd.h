#ifndef GAGE_BRAILLE_SIMD_H
#define GAGE_BRAILLE_SIMD_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <unistd.h>
#include <termios.h>
#include <fcntl.h>
#include <time.h>

// ==========================================
// 1. HARDWARE SIMD VECTORS (ARM NEON)
// ==========================================
typedef float gage_vec2 __attribute__((ext_vector_type(2)));
typedef float gage_vec4 __attribute__((ext_vector_type(4)));

static inline gage_vec4 gage_vec4_new(float x, float y, float z, float w) {
    gage_vec4 v = {x, y, z, w};
    return v;
}

static inline gage_vec4 gage_vec4_add(gage_vec4 a, gage_vec4 b) {
    return a + b; // Compiles directly to ARM NEON fadd.4s
}

static inline gage_vec4 gage_vec4_scale(gage_vec4 a, float s) {
    return a * s; // Compiles directly to ARM NEON fmul.4s
}

static inline float gage_vec4_dot(gage_vec4 a, gage_vec4 b) {
    gage_vec4 prod = a * b;
    return prod[0] + prod[1] + prod[2] + prod[3];
}

// Swizzling: wzyx reverse vector components
static inline gage_vec4 gage_vec4_swizzle_wzyx(gage_vec4 v) {
    gage_vec4 out = {v[3], v[2], v[1], v[0]};
    return out;
}

// ==========================================
// 2. 60 FPS BRAILLE SUBPIXEL CANVAS (2x4)
// ==========================================
// Each Braille character represents a 2x4 dot matrix:
// [0,0] (dot 1: 0x01)  [1,0] (dot 4: 0x08)
// [0,1] (dot 2: 0x02)  [1,1] (dot 5: 0x10)
// [0,2] (dot 3: 0x04)  [1,2] (dot 6: 0x20)
// [0,3] (dot 7: 0x40)  [1,3] (dot 8: 0x80)

typedef struct {
    int cell_width;
    int cell_height;
    int pixel_width;
    int pixel_height;
    unsigned char* dots;
    char* output_buffer;
} GageBrailleCanvas;

static inline GageBrailleCanvas* gage_canvas_create(int cell_w, int cell_h) {
    GageBrailleCanvas* c = (GageBrailleCanvas*)malloc(sizeof(GageBrailleCanvas));
    c->cell_width = cell_w;
    c->cell_height = cell_h;
    c->pixel_width = cell_w * 2;
    c->pixel_height = cell_h * 4;
    c->dots = (unsigned char*)calloc(cell_w * cell_h, sizeof(unsigned char));
    // Each UTF-8 braille char is 3 bytes + newline + formatting
    c->output_buffer = (char*)malloc(cell_w * cell_h * 4 + cell_h + 128);
    return c;
}

static inline void gage_canvas_clear(GageBrailleCanvas* c) {
    memset(c->dots, 0, c->cell_width * c->cell_height);
}

static inline void gage_canvas_set_pixel(GageBrailleCanvas* c, int x, int y) {
    if (x < 0 || x >= c->pixel_width || y < 0 || y >= c->pixel_height) return;
    int cell_x = x / 2;
    int cell_y = y / 4;
    int sub_x = x % 2;
    int sub_y = y % 4;

    static const unsigned char dot_map[4][2] = {
        {0x01, 0x08},
        {0x02, 0x10},
        {0x04, 0x20},
        {0x40, 0x80}
    };

    c->dots[cell_y * c->cell_width + cell_x] |= dot_map[sub_y][sub_x];
}

// High-speed Bresenham line drawing in subpixel space
static inline void gage_canvas_draw_line(GageBrailleCanvas* c, int x0, int y0, int x1, int y1) {
    int dx = abs(x1 - x0), sx = x0 < x1 ? 1 : -1;
    int dy = -abs(y1 - y0), sy = y0 < y1 ? 1 : -1;
    int err = dx + dy, e2;
    while (1) {
        gage_canvas_set_pixel(c, x0, y0);
        if (x0 == x1 && y0 == y1) break;
        e2 = 2 * err;
        if (e2 >= dy) { err += dy; x0 += sx; }
        if (e2 <= dx) { err += dx; y0 += sy; }
    }
}

// Zero-flicker double-buffered render flush
static inline void gage_canvas_flush(GageBrailleCanvas* c) {
    char* ptr = c->output_buffer;
    ptr += sprintf(ptr, "\033[H"); // Reset cursor to top-left

    for (int cy = 0; cy < c->cell_height; cy++) {
        for (int cx = 0; cx < c->cell_width; cx++) {
            unsigned int code = 0x2800 + c->dots[cy * c->cell_width + cx];
            // Encode Unicode Braille character to UTF-8
            *ptr++ = (char)(0xE0 | ((code >> 12) & 0x0F));
            *ptr++ = (char)(0x80 | ((code >> 6) & 0x3F));
            *ptr++ = (char)(0x80 | (code & 0x3F));
        }
        *ptr++ = '\n';
    }
    *ptr = '\0';
    write(STDOUT_FILENO, c->output_buffer, ptr - c->output_buffer);
}

static inline void gage_canvas_free(GageBrailleCanvas* c) {
    if (c) {
        free(c->dots);
        free(c->output_buffer);
        free(c);
    }
}

// ==========================================
// 3. NON-BLOCKING RAW TERMINAL INPUT
// ==========================================
static struct termios orig_termios;

static inline void gage_terminal_raw_enable(void) {
    tcgetattr(STDIN_FILENO, &orig_termios);
    struct termios raw = orig_termios;
    raw.c_lflag &= ~(ECHO | ICANON);
    raw.c_cc[VMIN] = 0;
    raw.c_cc[VTIME] = 0;
    tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw);
    printf("\033[?25l\033[2J"); // Hide cursor, clear screen
    fflush(stdout);
}

static inline void gage_terminal_raw_disable(void) {
    tcsetattr(STDIN_FILENO, TCSAFLUSH, &orig_termios);
    printf("\033[?25h\n"); // Restore cursor
    fflush(stdout);
}

static inline char gage_terminal_poll_key(void) {
    char ch = 0;
    if (read(STDIN_FILENO, &ch, 1) > 0) {
        return ch;
    }
    return 0;
}

#endif // GAGE_BRAILLE_SIMD_H
