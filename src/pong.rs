use piston::event::*;
use piston::input::Button::Keyboard;
use piston::input::keyboard::Key;
use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;
use opengl_graphics::{ GlGraphics, OpenGL };
use game_object::GameObject;

static OPENGL_VERSION: OpenGL = OpenGL::_3_2;
static SIZE: [u32; 2] = [512, 512];
static PADDLE_SIZE: [f64; 2] = [8.0, 32.0];
static PADDLE_ACCEL: f64 = 4000.0;
static PADDLE_FRICTION: f64 = 0.5;
static PADDLE_MAX_SPEED: f64 = 400.0;
static BALL_SIZE: [f64; 2] = [8.0, 8.0];
static BALL_START_MAX_ANGLE: f64 = 60.0;
static BALL_START_SPEED: f64 = 200.0;
static BALL_SPEED_INCREMENT: f64 = 25.0;

struct Pong {
    gl: GlGraphics,
    p1: GameObject,
    p2: GameObject,
    ball: GameObject,
    up: bool,
    down: bool,
    server: Player,
    p1_score: u32,
    p2_score: u32,
}

enum Player {
    Left,
    Right
}

impl Pong {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        
        let objs = [&self.p1, &self.p2, &self.ball];
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for obj in objs.iter() {
                let rect = [0.0, 0.0, obj.size[0], obj.size[1]];
                let transform = c.transform.trans(obj.pos[0], obj.pos[1])
                    .trans(-obj.size[0] / 2.0, -obj.size[1] / 2.0);
                rectangle(WHITE, rect, transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let (ai_up, ai_down) = self.handle_ai_paddle();

        Pong::handle_paddle(&mut self.p1, self.up, self.down, args.dt);
        Pong::handle_paddle(&mut self.p2, ai_up, ai_down, args.dt);
        Pong::handle_game_object(&mut self.p1, args.dt, false);
        Pong::handle_game_object(&mut self.p2, args.dt, false);
        Pong::handle_game_object(&mut self.ball, args.dt, true);
        self.handle_ball();
    }

    fn start(&mut self) {
        self.p1.pos = [self.p1.size[0] / 2.0 + 4.0, (SIZE[1] / 2) as f64];
        self.p2.pos = [SIZE[0] as f64 - self.p2.size[0] / 2.0 - 4.0,
            (SIZE[1] / 2) as f64];
        
        self.reset();
    }

    fn reset(&mut self) {
        use std::f64::consts::PI;
        use rand;
        use rand::Rng;

        self.ball.pos = [(SIZE[0] / 2) as f64, (SIZE[1] / 2) as f64];

        let mut rng = rand::thread_rng();
        let max_angle = 2.0 * BALL_START_MAX_ANGLE * PI / 180.0;
        let angle = rng.next_f64() * max_angle - max_angle / 2.0;
        self.ball.vel = [
            angle.cos() * BALL_START_SPEED * self.serve_direction(),
            angle.sin() * BALL_START_SPEED
        ];
    }

    fn serve_direction(&mut self) -> f64 {
        match self.server {
            Player::Left => { -1.0 }
            Player::Right => { 1.0 }
        }
    }

    fn key_press(&mut self, key: Key) {
        match key {
            Key::Up => { self.up = true; }
            Key::Down => { self.down = true; }
            _ => {}
        }
    }

    fn key_release(&mut self, key: Key) {
        match key {
            Key::Up => { self.up = false; }
            Key::Down => { self.down = false; }
            _ => {}
        }
    }

    fn handle_game_object(obj: &mut GameObject, dt: f64, bounce: bool) {
        obj.pos[0] += obj.vel[0] * dt;
        obj.pos[1] += obj.vel[1] * dt;

        if obj.pos[1] + obj.size[1] / 2.0 >= SIZE[1] as f64 {
            obj.pos[1] = SIZE[1] as f64 - obj.size[1] / 2.0;
            if bounce { obj.vel[1] *= -1.0; }
            else { obj.vel[1] = 0.0; }
        }
        
        if obj.pos[1] - obj.size[1] / 2.0 <= 0.0f64 {
            obj.pos[1] = obj.size[1] / 2.0;
            if bounce { obj.vel[1] *= -1.0; }
            else { obj.vel[1] = 0.0; }
        }
    }

    fn handle_paddle(paddle: &mut GameObject, up: bool, down: bool, dt: f64) {
        if up {
            paddle.vel[1] -= PADDLE_ACCEL * dt;
        } else if down { 
            paddle.vel[1] += PADDLE_ACCEL * dt;
        } else {
            let dv = -paddle.vel[1].signum() * PADDLE_ACCEL * dt;

            if dv.abs() >= paddle.vel[1].abs() { paddle.vel[1] = 0.0; }
            else { paddle.vel[1] += dv; }
        }
     
        if paddle.vel[1] > PADDLE_MAX_SPEED { 
            paddle.vel[1] = PADDLE_MAX_SPEED;
        } else if paddle.vel[1] < -PADDLE_MAX_SPEED {
            paddle.vel[1] = -PADDLE_MAX_SPEED;
        }
    }

    fn handle_ai_paddle(&self) -> (bool, bool) {
        let mut ai_up = false;
        let mut ai_down = false;

        if self.ball.vel[0] > 0.0 {
            let t = (self.p2.pos[0] - self.ball.pos[0]) / self.ball.vel[0];
            let target_y = self.ball.pos[1] + self.ball.vel[1] * t;

            if target_y > self.p2.pos[1] { ai_down = true; }
            else if target_y < self.p2.pos[1] { ai_up = true; }
        }

        (ai_up, ai_down)
    }

    fn handle_ball(&mut self) {
        for paddle in [&self.p1, &self.p2].iter() {
            match self.ball.collision_normal(paddle) {
                Some(normal) => {
                    let dot = self.ball.vel[0] * normal[0] + 
                        self.ball.vel[1] * normal[1];

                    // reflect ball's velocity off collision normal
                    self.ball.vel = [
                        self.ball.vel[0] - 2.0 * normal[0] * dot,
                        self.ball.vel[1] - 2.0 * normal[1] * dot
                    ];
                    
                    // apply a bit of paddle y velocity to ball
                    if normal[0] != 0.0 {
                        self.ball.vel[1] += paddle.vel[1] * PADDLE_FRICTION;
                    }

                    // increment ball x velocity a bit
                    self.ball.vel[0] += BALL_SPEED_INCREMENT *
                        self.ball.vel[0].signum();

                    Pong::correct_collision(&mut self.ball, paddle, normal);
                }

                None => {}
            }
        }

        if self.ball.pos[0] > SIZE[0] as f64 { self.score(Player::Left); }
        else if self.ball.pos[0] < 0.0 { self.score(Player::Right); }
    }

    fn correct_collision(a: &mut GameObject, b: &GameObject, normal: [f64; 2]) {
        if normal == [1.0, 0.0] {
            a.pos[0] = b.pos[0] + b.size[0] / 2.0 + a.size[0] / 2.0;
        } else if normal == [-1.0, 0.0] {
            a.pos[0] = b.pos[0] - b.size[0] / 2.0 - a.size[0] / 2.0;
        } else if normal == [0.0, 1.0] {
            a.pos[1] = b.pos[1] + b.size[1] / 2.0 + a.size[1] / 2.0;
        } else if normal == [0.0, -1.0] {
            a.pos[1] = b.pos[1] - b.size[1] / 2.0 - a.size[1] / 2.0;
        }
    }

    fn score(&mut self, player: Player) {
        match player { 
            Player::Left => { 
                self.server = Player::Right; 
                self.p1_score += 1;
                println!("Player 1 scored! {}-{}", self.p1_score,
                    self.p2_score);
            }

            Player::Right => { 
                self.server = Player::Left;
                self.p2_score += 1;
                println!("Player 2 scored! {}-{}", self.p1_score,
                    self.p2_score);
            }
        }

        self.reset();
    }
}

pub fn play() {
    let window = Window::new(OPENGL_VERSION,
        WindowSettings::new("pong", SIZE)
        .exit_on_esc(true));

    let mut pong = Pong {
        gl: GlGraphics::new(OPENGL_VERSION),
        p1: GameObject { size: PADDLE_SIZE, ..Default::default() },
        p2: GameObject { size: PADDLE_SIZE, ..Default::default() },
        ball: GameObject { size: BALL_SIZE, ..Default::default() },
        up: false,
        down: false,
        server: Player::Left,
        p1_score: 0,
        p2_score: 0,
    };

    pong.start();

    for e in window.events() {
        if let Some(r) = e.render_args() { pong.render(&r); }
        if let Some(u) = e.update_args() { pong.update(&u); }
        if let Some(Keyboard(key)) = e.press_args() { pong.key_press(key); }
        if let Some(Keyboard(key)) = e.release_args() { pong.key_release(key); }
    }
}
