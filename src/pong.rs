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
static BALL_SIZE: [f64; 2] = [8.0, 8.0];
static BALL_START_MAX_ANGLE: f64 = 60.0;
static BALL_START_SPEED: f64 = 250.0;

struct Pong {
    gl: GlGraphics,
    p1: GameObject,
    p2: GameObject,
    ball: GameObject,
    up: bool,
    down: bool,
    server: Player
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
        
        let p1_rect = [0.0, 0.0, self.p1.size[0], self.p1.size[1]]; 
        let p2_rect = [0.0, 0.0, self.p2.size[0], self.p2.size[1]]; 
        let ball_rect = [0.0, 0.0, self.ball.size[0], self.ball.size[1]]; 
        let ball = &mut self.ball;
        let p1 = &mut self.p1;
        let p2 = &mut self.p2;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            let transform = c.transform.trans(ball.pos[0], ball.pos[1])
                .trans(-ball.size[0]/2.0, -ball.size[1]/2.0);
            rectangle(WHITE, ball_rect, transform, gl);

            let transform = c.transform.trans(p1.pos[0], p1.pos[1])
                .trans(-p1.size[0]/2.0, -p1.size[1]/2.0);
            rectangle(WHITE, p1_rect, transform, gl);

            let transform = c.transform.trans(p2.pos[0], p2.pos[1])
                .trans(-p2.size[0]/2.0, -p2.size[1]/2.0);
            rectangle(WHITE, p2_rect, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let mut ai_up = false;
        let mut ai_down = false;

        if self.ball.vel[0] > 0.0 {
            if self.ball.pos[1] > self.p2.pos[1] { ai_down = true; }
            else if self.ball.pos[1] < self.p2.pos[1] { ai_up = true; }
        }

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
    }

    fn handle_ball(&mut self) {
        if self.ball.intersects(&self.p1) {
            self.ball.vel[0] *= -1.0;
            self.ball.vel[1] += self.p1.vel[1] * PADDLE_FRICTION;
        }

        if self.ball.intersects(&self.p2) {
            self.ball.vel[0] *= -1.0;
            self.ball.vel[1] += self.p2.vel[1] * PADDLE_FRICTION;
        }

        if self.ball.pos[0] > SIZE[0] as f64 { self.score(Player::Left); }
        else if self.ball.pos[0] < 0.0 { self.score(Player::Right); }
    }

    fn score(&mut self, player: Player) {
        match player { 
            Player::Left => { 
                self.server = Player::Right; 
            }

            Player::Right => { 
                self.server = Player::Left;
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
        server: Player::Left
    };

    pong.start();

    for e in window.events() {
        if let Some(r) = e.render_args() { pong.render(&r); }
        if let Some(u) = e.update_args() { pong.update(&u); }
        if let Some(Keyboard(key)) = e.press_args() { pong.key_press(key); }
        if let Some(Keyboard(key)) = e.release_args() { pong.key_release(key); }
    }
}
