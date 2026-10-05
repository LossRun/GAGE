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

// 1. ARM NEON Hardware SIMD Types
typedef float gage_vec2 __attribute__((ext_vector_type(2)));
typedef float gage_vec4 __attribute__((ext_vector_type(4)));

static inline gage_vec4 gage_vec4_create(float x, float y, float z, float w) {
    gage_vec4 v = {x, y, z, w};
    return v;
}

static inline gage_vec4 gage_vec4_add(gage_vec4 a, gage_vec4 b) {
    return a + b;
}

static inline gage_vec4 gage_vec4_scale(gage_vec4 a, float s) {
    return a * s;
}

static inline float gage_vec4_dot(gage_vec4 a, gage_vec4 b) {
    gage_vec4 p = a * b;
    return p[0] + p[1] + p[2] + p[3];
}

// 2. Non-blocking Terminal Input & Game Loop Helpers
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

#endif // GAGE_BRAILLE_SIMD_H
