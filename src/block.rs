use rand::prelude::*;

const BOUNCE_FACTOR = 0.25;
const GRAVITY = 1500;

pub struct Block {
    fill: f32,
    position: [f32; 3],
    velocity: [f32; 3],
    rotation: [f32; 3], // pitch, yaw, roll; degrees
    rotational_velocity: [f32; 3], // pitch, yaw, roll; degrees

    falling: bool,
    removal_flag: bool,
}

impl Block {
    pub fn new(init_color: Option<BlockColor>) -> Self {
        let color = match init_color {
            Some(c) => c,
            None => BlockColor::random(),
        };

        let pit_vel = (random() - 0.5) * 360 * 2;
        let yaw_vel = (random() - 0.5) * 360 * 2;
        let rol_vel = (random() - 0.5) * 360 * 2;

        Self {
            fill: 0
            position: [0.0, 0.0, 0.0],
            velocity: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            rotational_velocity: [pit_vel, yaw_vel, rol_vel],

            falling: false,
            removal_flag: false,
        }
    }

    fn fall(&self) {
        self.falling = true;
        self.velocity[1] = -500.0 * Math.random();
    }

    fn get_destination_y(&self) -> f32 {
        board::to_grid_y(self.row) + (if self.row >= 0 && self.color == BlockColor::Poison { board::BLOCK_GAP } else { 0 });
    }

    fn block_logic(&self, delta: f32, row: i32, col: i32) {
        self.row = row;
        self.col = col;
        self.position[0] = board::to_grid_x(self.col);

        if (self.row == -1 && !self.falling) {
            self.position[1] = self.destination_y() + 10 * ((performance.now() / 1000 + (self.col * -std::f32::consts::PI / board::COLUMNS)).sin() - 1);
        } else {
            self.fill = 100;
            if (delta > 2) {
                self.position[1] = self.destination_y();
                self.velocity[1] = 0;
            } else
                while (delta > 0) {
                    let spu = 1 / UPDATES_PER_SECOND; // seconds per update
                    let time_slice = if delta < spu { delta } else { spu };

                    self.velocity[1] += GRAVITY * time_slice;
                    self.position[1] += self.velocity[1] * time_slice;

                    if (!self.falling && self.y >= self.destY) {
                        self.velocity[1] *= -BOUNCE_FACTOR;
                        self.position[1] = self.destination_y();
                    } else if (self.falling) {
                        self.rotation[0] += self.rotational_velocity[0] * time_slice;
                        self.rotation[1] += self.rotational_velocity[1] * time_slice;
                        self.rotation[2] += self.rotational_velocity[2] * time_slice;
                        self.position[2] += self.velocity[2] * time_slice;
                    }
                    delta -= timeSlice;
                }
        }
    }

    set_fill(val) {
        self._fill = val > 100 ? 100 : val;
    }

    get_fill() {
         self._fill
    }

    fn is_full(&self) {
        self.fill >= 100
    }

    fn is_gone(&self) {
        self.y > to_gl_y(window.inner_height - to_browser_y(global_y_offset)) * 1.3
    }

    fn render_block(&self, gl, program_info, y_offset) {
        let f = self._fill / 100;
        // Re-assign the fill variable for a cool cubic-easing animation effect ;)
        f--;
        f = (f * f * f + 1);
        let bw = Board.BLOCK_WIDTH;
        let w = Board.BLOCK_WIDTH * ((f * 5 > 1) ? 1 : (f * 5));
        let h = bw * f;

        cube_mesh::set_color(self.color, gl, program_info);
        cube_mesh::render(gl, self.x + (bw - w) / 2, self.y + bw - h + y_offset, self.z, w, h, w, self.pit, self.yaw, self.rol);
    }
}

pub enum BlockColor {
    Red,
    Orange,
    Green,
    Blue,
    Poison,
    Golden,
}

impl BlockColor {
    fn to_color_int(&self) -> i32 {
        match self {
            Self::Red => 0xff004c,
            Self::Orange => 0xffa530,
            Self::Green => 0x50ec8c,
            Self::Blue => 0x117cff,
            Self::Poison => 0x000000,
            Self::Golden => 0xffff00,
        }
    }

    fn random(&self) -> Self {
        match (random() * 4.0).floor() as i8 {
            0 => Self::Red,
            1 => Self::Orange,
            2 => Self::Green,
            _ => Self::Blue
        }
    }
}
