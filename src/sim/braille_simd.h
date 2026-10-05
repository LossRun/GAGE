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

// --- 1. ARM NEON Hardware SIMD Types ---
typedef float gage_vec4 __attribute__((ext_vector_type(4)));

static inline gage_vec4 gage_vec4_create(float x, float y, float z, float w) {
    gage_vec4 v = {x, y, z, w};
    return v;
}

// --- 2. Non-blocking Terminal Input ---
static struct termios orig_term_settings;
static int term_configured = 0;

static inline void gage_enable_raw_mode(void) {
    if (term_configured) return;
    tcgetattr(STDIN_FILENO, &orig_term_settings);
    struct termios raw = orig_term_settings;
    raw.c_lflag &= ~(ECHO | ICANON);
    raw.c_cc[VMIN] = 0;
    raw.c_cc[VTIME] = 0;
    tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw);
    term_configured = 1;
}

static inline void gage_disable_raw_mode(void) {
    if (!term_configured) return;
    tcsetattr(STDIN_FILENO, TCSAFLUSH, &orig_term_settings);
    term_configured = 0;
}

static inline int gage_poll_key(void) {
    gage_enable_raw_mode();
    char c = 0;
    if (read(STDIN_FILENO, &c, 1) > 0) {
        return (int)c;
    }
    return 0;
}

static inline void gage_sleep_ms(int ms) {
    struct timespec ts;
    ts.tv_sec = ms / 1000;
    ts.tv_nsec = (ms % 1000) * 1000000L;
    nanosleep(&ts, NULL);
}

// --- 3. Unicode Braille Subpixel Framebuffer (102 x 76) ---
#define BRAILLE_COLS 51
#define BRAILLE_ROWS 19
static unsigned char braille_fb[BRAILLE_ROWS][BRAILLE_COLS];

static inline void braille_clear(void) {
    memset(braille_fb, 0, sizeof(braille_fb));
}

static inline void braille_set_pixel(int px, int py) {
    if (px < 0 || px >= BRAILLE_COLS * 2 || py < 0 || py >= BRAILLE_ROWS * 4) return;
    int cell_x = px / 2;
    int cell_y = py / 4;
    int sub_x = px % 2;
    int sub_y = py % 4;

    // Unicode Braille dot bitmask mapping
    static const int dot_map[4][2] = {
        {0x01, 0x08},
        {0x02, 0x10},
        {0x04, 0x20},
        {0x40, 0x80}
    };
    braille_fb[cell_y][cell_x] |= dot_map[sub_y][sub_x];
}

static inline void braille_render_flush(void) {
    for (int y = 0; y < BRAILLE_ROWS; y++) {
        for (int x = 0; x < BRAILLE_COLS; x++) {
            unsigned char mask = braille_fb[y][x];
            if (mask == 0) {
                putchar(' ');
            } else {
                // Encode Unicode UTF-8 Braille (0x2800 + mask)
                int cp = 0x2800 + mask;
                putchar(0xE0 | ((cp >> 12) & 0x0F));
                putchar(0x80 | ((cp >> 6) & 0x3F));
                putchar(0x80 | (cp & 0x3F));
            }
        }
        putchar('\n');
    }
}

#endif // GAGE_BRAILLE_SIMD_H
