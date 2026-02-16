use egui::Vec2;
use rand::seq::SliceRandom;

/// Sand-style simulation grid
pub struct Simulation {
    pub width: usize,
    pub height: usize,
    pub density: Vec<f32>, // 0 = empty, 1 = full
    pub gravity: Vec2,     // gravity vector
}

impl Simulation {
    /// Create a new simulation with top 2 rows filled
    pub fn new(width: usize, height: usize) -> Self {
        let mut density = vec![0.0; width * height];

        // Fill the top 2 rows
        for y in 0..2 {
            for x in 0..width {
                density[y * width + x] = 1.0;
            }
        }

        Self {
            width,
            height,
            density,
            gravity: Vec2::new(0.0, 1.0),
        }
    }
    pub fn reset(&mut self) {
        self.density.fill(0.0);
        for y in 0..2 {
            for x in 0..self.width {
                self.density[y * self.width + x] = 1.0;
            }
        }
    }

    pub fn set_gravity(&mut self, gravity: Vec2) {
        self.gravity = gravity;
    }

    /// Perform a sand-style simulation step
    pub fn step(&mut self) {
        let mut next = vec![0.0; self.width * self.height];
        
        let gx = self.gravity.x;
        let gy = self.gravity.y;
        
        let sign_x = if gx >= 0.0 { 1 } else { -1 };
        let sign_y = if gy >= 0.0 { 1 } else { -1 };
        
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let idx = y * self.width + x;
                if self.density[idx] < 1.0 { continue; }
            
                let mut moved = false;
            
                // Compute preferred neighbors in order
                let mut neighbors = Vec::new();
            
                // Vertical move first if gravity is vertical-dominant
                if gy.abs() >= gx.abs() {
                    neighbors.push((0, sign_y));              // vertical
                    // Diagonals
                    let diag1 = (sign_x, sign_y);
                    let diag2 = (-sign_x, sign_y);
                    // Randomize only between the two diagonals
                    if rand::random::<bool>() { neighbors.push(diag1); neighbors.push(diag2); }
                    else { neighbors.push(diag2); neighbors.push(diag1); }
                } else {
                    // Horizontal-dominant gravity
                    neighbors.push((sign_x, 0));              // horizontal
                    let diag1 = (sign_x, sign_y);
                    let diag2 = (sign_x, -sign_y);
                    if rand::random::<bool>() { neighbors.push(diag1); neighbors.push(diag2); }
                    else { neighbors.push(diag2); neighbors.push(diag1); }
                    neighbors.push((0, sign_y));              // vertical
                }
            
                // Try moving into neighbors in order
                for (dx, dy) in neighbors {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if self.in_bounds(nx, ny) {
                        let nidx = ny as usize * self.width + nx as usize;
                        if self.density[nidx] == 0.0 && next[nidx] == 0.0 {
                            next[nidx] = 1.0;
                            moved = true;
                            break;
                        }
                    }
                }
            
                if !moved {
                    next[idx] = 1.0; // stay in place
                }
            }
        }
    
        self.density = next;
}


    /// Returns neighbor directions in order based on gravity
    fn get_gravity_dirs(&self, gx: f32, gy: f32) -> Vec<(isize, isize)> {
        let mut dirs = Vec::new();
        let sign_x = if gx >= 0.0 { 1 } else { -1 };
        let sign_y = if gy >= 0.0 { 1 } else { -1 };

        if gy.abs() >= gx.abs() {
            // Vertical-dominant gravity
            dirs.push((0, sign_y));       // vertical
            dirs.push((sign_x, sign_y));  // diagonal
            dirs.push((-sign_x, sign_y));
            dirs.push((sign_x, 0));       // horizontal
            dirs.push((-sign_x, 0));
        } else {
            // Horizontal-dominant gravity
            dirs.push((sign_x, 0));       // horizontal
            dirs.push((sign_x, sign_y));  // diagonal
            dirs.push((sign_x, -sign_y));
            dirs.push((0, sign_y));       // vertical
            dirs.push((0, -sign_y));
        }

        dirs
    }

    /// Check if a cell is inside grid bounds
    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize
    }
}
