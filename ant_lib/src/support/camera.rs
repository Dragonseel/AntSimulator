use glium::{Frame, Surface};

pub struct Camera {
    pub view_mat: [[f32; 4]; 4],
    pub proj_mat: [[f32; 4]; 4],

    pub position: [f32; 3],
    direction: [f32; 3],
    up: [f32; 3],
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            view_mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            proj_mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            position: [0.0, 0.0, -2.0],
            direction: [0.0, 0.0, 1.0],
            up: [0.0, 1.0, 0.0],
        }
    }

    pub fn move_forwards(&mut self) {
        self.position = [
            self.position[0] + 10.0 * self.direction[0],
            self.position[1] + 10.0 * self.direction[1],
            self.position[2] + 10.0 * self.direction[2],
        ];
    }

    pub fn move_backwards(&mut self) {
        self.position = [
            self.position[0] - 10.0 * self.direction[0],
            self.position[1] - 10.0 * self.direction[1],
            self.position[2] - 10.0 * self.direction[2],
        ];
    }

    pub fn move_up(&mut self) {
        self.position = [
            self.position[0] + 10.0 * self.up[0],
            self.position[1] + 10.0 * self.up[1],
            self.position[2] + 10.0 * self.up[2],
        ];
    }

    pub fn move_down(&mut self) {
        self.position = [
            self.position[0] - 10.0 * self.up[0],
            self.position[1] - 10.0 * self.up[1],
            self.position[2] - 10.0 * self.up[2],
        ];
    }

    pub fn move_left(&mut self) {
        let cross = [
            self.up[1] * self.direction[2] - self.up[2] * self.direction[1],
            self.up[2] * self.direction[0] - self.up[0] * self.direction[2],
            self.up[0] * self.direction[1] - self.up[1] * self.direction[0],
        ];

        self.position = [
            self.position[0] - 10.0 * cross[0],
            self.position[1] - 10.0 * cross[1],
            self.position[2] - 10.0 * cross[2],
        ];
    }

    pub fn move_right(&mut self) {
        let cross = [
            self.up[1] * self.direction[2] - self.up[2] * self.direction[1],
            self.up[2] * self.direction[0] - self.up[0] * self.direction[2],
            self.up[0] * self.direction[1] - self.up[1] * self.direction[0],
        ];

        self.position = [
            self.position[0] + 10.0 * cross[0],
            self.position[1] + 10.0 * cross[1],
            self.position[2] + 10.0 * cross[2],
        ];
    }

    pub fn update_view(&mut self) {
        let f = {
            let f = self.direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };

        let s = [
            self.up[1] * f[2] - self.up[2] * f[1],
            self.up[2] * f[0] - self.up[0] * f[2],
            self.up[0] * f[1] - self.up[1] * f[0],
        ];

        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };

        let u = [
            f[1] * s_norm[2] - f[2] * s_norm[1],
            f[2] * s_norm[0] - f[0] * s_norm[2],
            f[0] * s_norm[1] - f[1] * s_norm[0],
        ];

        let p = [
            -self.position[0] * s_norm[0]
                - self.position[1] * s_norm[1]
                - self.position[2] * s_norm[2],
            -self.position[0] * u[0] - self.position[1] * u[1] - self.position[2] * u[2],
            -self.position[0] * f[0] - self.position[1] * f[1] - self.position[2] * f[2],
        ];

        self.view_mat = [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }

    pub fn update_proj(&mut self, target: &mut Frame) {
        self.proj_mat = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
            ]
        };
    }
}
