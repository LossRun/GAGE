#ifndef GAGE_SIM_RUNTIME_H
#define GAGE_SIM_RUNTIME_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

// ==========================================
// 1. CONTINUOUS PHYSICS RUNTIME
// ==========================================
typedef struct {
    char name[32];
    double x, y;
    double vx, vy;
    double mass;
} GageBody;

typedef struct {
    GageBody bodies[64];
    int count;
    double gravity_y;
    double time;
} GagePhysicsWorld;

static inline GagePhysicsWorld* gage_physics_new(double gravity_y) {
    GagePhysicsWorld* w = (GagePhysicsWorld*)malloc(sizeof(GagePhysicsWorld));
    w->count = 0;
    w->gravity_y = gravity_y;
    w->time = 0.0;
    return w;
}

static inline void gage_physics_add_body(GagePhysicsWorld* w, const char* name, double x, double y, double vx, double vy, double mass) {
    if (w->count < 64) {
        GageBody* b = &w->bodies[w->count++];
        strncpy(b->name, name, 31);
        b->name[31] = '\0';
        b->x = x;
        b->y = y;
        b->vx = vx;
        b->vy = vy;
        b->mass = mass;
    }
}

static inline void gage_physics_step(GagePhysicsWorld* w, double dt) {
    for (int i = 0; i < w->count; i++) {
        GageBody* b = &w->bodies[i];
        b->vy += w->gravity_y * dt;
        b->x += b->vx * dt;
        b->y += b->vy * dt;

        // Ground restitution bounce
        if (b->y < 0.0) {
            b->y = 0.0;
            b->vy = -b->vy * 0.82;
            if (fabs(b->vy) < 0.15) b->vy = 0.0;
        }
    }
    w->time += dt;
}

static inline void gage_physics_render(GagePhysicsWorld* w) {
    const int width = 38;
    const int height = 12;
    char grid[12][38];
    memset(grid, ' ', sizeof(grid));

    for (int x = 0; x < width; x++) grid[height - 1][x] = '=';

    for (int i = 0; i < w->count; i++) {
        GageBody* b = &w->bodies[i];
        int px = (int)((b->x / 40.0) * (width - 1));
        int py = (int)((1.0 - (b->y / 25.0)) * (height - 2));
        if (px < 0) px = 0;
        if (px >= width) px = width - 1;
        if (py < 0) py = 0;
        if (py >= height - 1) py = height - 2;
        grid[py][px] = 'O';
    }

    printf("\n\033[1;36m  PHYSICS SIMULATION [t = %.2fs, g = %.2fm/s²]\033[0m\n", w->time, w->gravity_y);
    printf("  ┌");
    for (int x = 0; x < width; x++) printf("─");
    printf("┐\n");

    for (int y = 0; y < height; y++) {
        printf("  │");
        for (int x = 0; x < width; x++) {
            if (grid[y][x] == 'O') {
                printf("\033[38;2;80;250;130m●\033[0m");
            } else if (grid[y][x] == '=') {
                printf("\033[90m=\033[0m");
            } else {
                putchar(' ');
            }
        }
        printf("│\n");
    }

    printf("  └");
    for (int x = 0; x < width; x++) printf("─");
    printf("┘\n");

    for (int i = 0; i < w->count; i++) {
        GageBody* b = &w->bodies[i];
        printf("  • \033[1m%s\033[0m: pos=(%.2f, %.2f) vel=(%.2f, %.2f)\n", b->name, b->x, b->y, b->vx, b->vy);
    }
    printf("\n");
}

static inline void gage_physics_free(GagePhysicsWorld* w) {
    if (w) free(w);
}

// ==========================================
// 2. 2D CELLULAR AUTOMATA GRID RUNTIME
// ==========================================
typedef struct {
    int width;
    int height;
    int generation;
    char cells[1024];
} GageGridWorld;

static inline GageGridWorld* gage_grid_new(int width, int height) {
    GageGridWorld* g = (GageGridWorld*)malloc(sizeof(GageGridWorld));
    g->width = (width > 32) ? 32 : width;
    g->height = (height > 16) ? 16 : height;
    g->generation = 0;
    memset(g->cells, 0, sizeof(g->cells));

    // Default glider seed in middle
    int mid_x = g->width / 2;
    int mid_y = g->height / 2;
    g->cells[mid_y * g->width + mid_x + 1] = 1;
    g->cells[(mid_y + 1) * g->width + mid_x + 2] = 1;
    g->cells[(mid_y + 2) * g->width + mid_x] = 1;
    g->cells[(mid_y + 2) * g->width + mid_x + 1] = 1;
    g->cells[(mid_y + 2) * g->width + mid_x + 2] = 1;

    return g;
}

static inline void gage_grid_step(GageGridWorld* g) {
    char next[1024];
    memset(next, 0, sizeof(next));

    for (int y = 0; y < g->height; y++) {
        for (int x = 0; x < g->width; x++) {
            int neighbors = 0;
            for (int dy = -1; dy <= 1; dy++) {
                for (int dx = -1; dx <= 1; dx++) {
                    if (dx == 0 && dy == 0) continue;
                    int nx = (x + dx + g->width) % g->width;
                    int ny = (y + dy + g->height) % g->height;
                    if (g->cells[ny * g->width + nx]) neighbors++;
                }
            }
            int idx = y * g->width + x;
            if (g->cells[idx] && (neighbors == 2 || neighbors == 3)) {
                next[idx] = 1;
            } else if (!g->cells[idx] && neighbors == 3) {
                next[idx] = 1;
            }
        }
    }
    memcpy(g->cells, next, sizeof(next));
    g->generation++;
}

static inline void gage_grid_render(GageGridWorld* g) {
    printf("\n\033[1;35m  CELLULAR AUTOMATA GRID [Gen: %d]\033[0m\n", g->generation);
    printf("  ┌");
    for (int x = 0; x < g->width; x++) printf("─");
    printf("┐\n");

    for (int y = 0; y < g->height; y++) {
        printf("  │");
        for (int x = 0; x < g->width; x++) {
            if (g->cells[y * g->width + x]) {
                printf("\033[38;2;80;250;130m■\033[0m");
            } else {
                putchar(' ');
            }
        }
        printf("│\n");
    }

    printf("  └");
    for (int x = 0; x < g->width; x++) printf("─");
    printf("┘\n\n");
}

static inline void gage_grid_free(GageGridWorld* g) {
    if (g) free(g);
}

#endif // GAGE_SIM_RUNTIME_H
