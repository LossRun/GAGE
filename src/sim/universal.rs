
/// --- 1. CONTINUOUS PHYSICS WORLD ---
#[derive(Debug, Clone)]
pub struct Body {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub mass: f64,
}

pub struct PhysicsEngine {
    pub bodies: Vec<Body>,
    pub gravity_y: f64,
    pub time: f64,
}

impl PhysicsEngine {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            gravity_y: -9.81,
            time: 0.0,
        }
    }

    pub fn add_body(&mut self, name: &str, x: f64, y: f64, vx: f64, vy: f64, mass: f64) {
        self.bodies.push(Body {
            name: name.to_string(),
            x,
            y,
            vx,
            vy,
            mass,
        });
    }

    pub fn step(&mut self, dt: f64) {
        for b in &mut self.bodies {
            // Semi-implicit Euler integration
            b.vy += self.gravity_y * dt;
            b.x += b.vx * dt;
            b.y += b.vy * dt;

            // Elastic ground bounce at y = 0
            if b.y < 0.0 {
                b.y = 0.0;
                b.vy = -b.vy * 0.82; // Coefficient of restitution
                if b.vy.abs() < 0.2 {
                    b.vy = 0.0;
                }
            }
        }
        self.time += dt;
    }

    pub fn render_ascii(&self, width: usize, height: usize) -> String {
        let mut grid = vec![vec![' '; width]; height];

        // Draw ground line
        for x in 0..width {
            grid[height - 1][x] = '=';
        }

        // Project bodies
        for b in &self.bodies {
            let px = ((b.x / 40.0) * (width as f64 - 1.0)).clamp(0.0, width as f64 - 1.0) as usize;
            let py = ((1.0 - (b.y / 25.0)) * (height as f64 - 2.0)).clamp(0.0, height as f64 - 2.0) as usize;
            grid[py][px] = '●';
        }

        let mut out = String::new();
        out.push_str(&format!("\n\x1b[1;36m  PHYSICS SIMULATION [t = {:.2}s, g = {:.2}m/s²]\x1b[0m\n", self.time, self.gravity_y));
        out.push_str("  ┌");
        for _ in 0..width { out.push('─'); }
        out.push_str("┐\n");

        for row in grid {
            out.push_str("  │");
            for ch in row {
                if ch == '●' {
                    out.push_str("\x1b[38;2;80;250;130m●\x1b[0m");
                } else if ch == '=' {
                    out.push_str("\x1b[90m=\x1b[0m");
                } else {
                    out.push(' ');
                }
            }
            out.push_str("│\n");
        }

        out.push_str("  └");
        for _ in 0..width { out.push('─'); }
        out.push_str("┘\n");

        for b in &self.bodies {
            out.push_str(&format!("  • \x1b[1;37m{}\x1b[0m: pos=({:.2}, {:.2}) vel=({:.2}, {:.2})\n", b.name, b.x, b.y, b.vx, b.vy));
        }
        out
    }
}

/// --- 2. 2D CELLULAR AUTOMATA & GRID WORLD ---
pub struct GridEngine {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<bool>,
    pub generation: usize,
}

impl GridEngine {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = vec![false; width * height];
        // Default glider & blinker pattern
        let mid_x = width / 2;
        let mid_y = height / 2;
        // Glider
        cells[mid_y * width + mid_x + 1] = true;
        cells[(mid_y + 1) * width + mid_x + 2] = true;
        cells[(mid_y + 2) * width + mid_x] = true;
        cells[(mid_y + 2) * width + mid_x + 1] = true;
        cells[(mid_y + 2) * width + mid_x + 2] = true;

        Self {
            width,
            height,
            cells,
            generation: 0,
        }
    }

    pub fn step(&mut self) {
        let mut next = vec![false; self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let mut live_neighbors = 0;
                for dy in [-1, 0, 1] {
                    for dx in [-1, 0, 1] {
                        if dx == 0 && dy == 0 { continue; }
                        let nx = (x as isize + dx).rem_euclid(self.width as isize) as usize;
                        let ny = (y as isize + dy).rem_euclid(self.height as isize) as usize;
                        if self.cells[ny * self.width + nx] {
                            live_neighbors += 1;
                        }
                    }
                }

                let idx = y * self.width + x;
                let is_alive = self.cells[idx];
                next[idx] = match (is_alive, live_neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }
        self.cells = next;
        self.generation += 1;
    }

    pub fn render_ascii(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("\n\x1b[1;35m  CELLULAR AUTOMATA GRID [Gen: {}]\x1b[0m\n", self.generation));
        out.push_str("  ┌");
        for _ in 0..self.width { out.push('─'); }
        out.push_str("┐\n");

        for y in 0..self.height {
            out.push_str("  │");
            for x in 0..self.width {
                if self.cells[y * self.width + x] {
                    out.push_str("\x1b[38;2;80;250;130m■\x1b[0m");
                } else {
                    out.push(' ');
                }
            }
            out.push_str("│\n");
        }

        out.push_str("  └");
        for _ in 0..self.width { out.push('─'); }
        out.push_str("┘\n");
        out
    }
}
