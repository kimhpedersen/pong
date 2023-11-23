use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::graphics::text::{Font, Text};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

struct Entity {
    texture: Texture,
    position: Vec2<f32>,
}

struct Player {
    paddle: Entity,
    score: i32,
}

struct Ball {
    puck: Entity,
    velocity: Vec2<f32>,
}

impl Player {
    fn new(texture: Texture, position: Vec2<f32>) -> Player {
        Player { paddle: Entity::new(texture, position), score: 0 as i32}
    }
}

impl Ball {
    fn new(texture: Texture, position: Vec2<f32>, velovity: Vec2<f32>) -> Ball {
        Ball { puck: Entity::new(texture, position), velocity: velovity}
    }
}

impl Entity {
    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity { texture, position}
    }

    fn width(&self) -> f32 {
        self.texture.width() as f32
    }
    
    fn height(&self) -> f32 {
        self.texture.height() as f32
    }
    
    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }
    fn centre(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.width() / 2.0),
            self.position.y + (self.height() / 2.0),
        )
    }
}

struct GameState {
    player1: Player,
    player2: Player,
    ball: Ball,
    game_over: bool,
    p1score_text: Text,
    p2score_text: Text,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
        let player1_position = Vec2::new(
            16.0,
            (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0,
        );

        let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
        let player2_position = Vec2::new(
            WINDOW_WIDTH - player2_texture.width() as f32 - 16.0,
            (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0,
        );

        let ball_texture = Texture::new(ctx, "./resources/ball.png")?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - ball_texture.height() as f32 / 2.0,
        );
        let ball_velocity = Vec2::new(BALL_SPEED as f32, 0 as f32);
        let font = Font::vector(ctx, "./resources/DejaVuSansMono.ttf", 24.0)?;

        Ok(GameState {
            player1: Player::new(player1_texture, player1_position),
            player2: Player::new(player2_texture, player2_position),
            ball: Ball::new(ball_texture, ball_position, ball_velocity),
            game_over: false,
            p1score_text: Text::new(format!("Blue: {}", 0), font.clone()),
            p2score_text: Text::new(format!("Red: {}", 0), font),
        })
    }

    fn reset_game(&mut self) {
        self.game_over = false;
        self.ball.puck.position = Vec2::new(
            WINDOW_WIDTH / 2.0 - self.ball.puck.width() / 2.0,
            WINDOW_HEIGHT / 2.0 - self.ball.puck.height() / 2.0,
        );
        self.player1.paddle.position = Vec2::new(
            16.0,
            (WINDOW_HEIGHT - self.player1.paddle.height()) / 2.0,
        );
        self.player2.paddle.position = Vec2::new(
            WINDOW_WIDTH - self.player2.paddle.width() - 16.0,
            (WINDOW_HEIGHT - self.player2.paddle.height()) / 2.0,
        );
        self.ball.velocity = Vec2::new(BALL_SPEED, 0.0);
    }

    fn update_positions(&mut self, ctx: &mut Context) {
        if input::is_key_down(ctx, Key::W) {
            self.player1.paddle.position.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::S) {
            self.player1.paddle.position.y += PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::Up) {
            self.player2.paddle.position.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::Down) {
            self.player2.paddle.position.y += PADDLE_SPEED;
        }
        if self.player1.paddle.position.y < 0.0 {
            self.player1.paddle.position.y = 0.0;
        }
        if self.player1.paddle.position.y + self.player1.paddle.height() > WINDOW_HEIGHT {
            self.player1.paddle.position.y = WINDOW_HEIGHT - self.player1.paddle.height();
        }
        if self.player2.paddle.position.y < 0.0 {
            self.player2.paddle.position.y = 0.0;
        }
        if self.player2.paddle.position.y + self.player2.paddle.height() > WINDOW_HEIGHT {
            self.player2.paddle.position.y = WINDOW_HEIGHT - self.player2.paddle.height();
        }
        self.ball.puck.position += self.ball.velocity;
        if self.ball.puck.position.y <= 0.0 || self.ball.puck.position.y + self.ball.puck.height() >= WINDOW_HEIGHT {
            self.ball.velocity.y = -self.ball.velocity.y;
        }
    }

    fn check_paddle_hit(&mut self) {
        let player1_bounds = self.player1.paddle.bounds();
        let player2_bounds = self.player2.paddle.bounds();
        let ball_bounds = self.ball.puck.bounds();

        let paddle_hit = if ball_bounds.intersects(&player1_bounds) {
            Some(&self.player1.paddle)
        } else if ball_bounds.intersects(&player2_bounds) {
            Some(&self.player2.paddle)
        } else {
            None
        };

        if let Some(paddle) = paddle_hit {
            // Increase the ball's velocity, then flip it.
            self.ball.velocity.x =
                -(self.ball.velocity.x + (BALL_ACC * self.ball.velocity.x.signum()));

            // Calculate the offset between the paddle and the ball, as a number between
            // -1.0 and 1.0.
            let offset = (paddle.centre().y - self.ball.puck.centre().y) / paddle.height();

            // Apply the spin to the ball.
            self.ball.velocity.y += PADDLE_SPIN * -offset;
        }
    }

    fn check_game_over(&mut self) {
        if self.ball.puck.position.x < 0.0 {
            self.game_over = true;
            self.player2.score += 1;
            self.p2score_text.set_content(format!("Red: {}", self.player2.score));
        }

        if self.ball.puck.position.x > WINDOW_WIDTH {
            self.game_over = true;
            self.player1.score += 1;
            self.p1score_text.set_content(format!("Blue: {}", self.player1.score));
        }
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> Result<(), tetra::TetraError> {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        self.p1score_text.draw(ctx, Vec2::new(10.0, 10.0));
        self.p2score_text.draw(ctx, Vec2::new(WINDOW_WIDTH - 100 as f32, 10.0));
        if !self.game_over {
            self.player1.paddle.texture.draw(ctx, self.player1.paddle.position);
            self.player2.paddle.texture.draw(ctx, self.player2.paddle.position);
            self.ball.puck.texture.draw(ctx, self.ball.puck.position);
        }
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> Result<(), tetra::TetraError> {
        if self.game_over {
            if input::is_key_pressed(ctx, Key::R) {
                self.reset_game();
            }
            return Ok(());
        }
        self.update_positions(ctx);
        self.check_paddle_hit();
        self.check_game_over();
        Ok(())
    }
}

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PADDLE_SPEED: f32 = 8.0;
const BALL_SPEED: f32 = 5.0;
const PADDLE_SPIN: f32 = 4.0;
const BALL_ACC: f32 = 0.05;

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
    .quit_on_escape(true)
    .build()?
    .run(GameState::new)
}