use tetra::graphics::{self, Color, Texture, ImageData};
use tetra::graphics::text::{Font, Text};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, State};

mod entity;

use entity::{Player, Ball};

const PADDLE_SPEED: f32 = 8.0;
const BALL_SPEED: f32 = 5.0;
const PADDLE_SPIN: f32 = 4.0;
const BALL_ACC: f32 = 0.05;
pub const WINDOW_WIDTH: f32 = 640.0;
pub const WINDOW_HEIGHT: f32 = 480.0;

pub struct GameState {
    player1: Player,
    player2: Player,
    ball: Ball,
    game_over: bool,
    p1score_text: Text,
    p2score_text: Text,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player1_imagedata = ImageData::from_encoded(include_bytes!("../resources/player1.png"))?;
        let player1_texture = Texture::from_image_data(ctx, &player1_imagedata)?;
        let player1_position = Vec2::new(
            16.0,
            (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0,
        );
        let player2_imagedata = ImageData::from_encoded(include_bytes!("../resources/player2.png"))?;
        let player2_texture = Texture::from_image_data(ctx, &player2_imagedata)?;
        let player2_position = Vec2::new(
            WINDOW_WIDTH - player2_texture.width() as f32 - 16.0,
            (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0,
        );
        let ball_imagedata = ImageData::from_encoded(include_bytes!("../resources/ball.png"))?;
        let ball_texture = Texture::from_image_data(ctx, &ball_imagedata)?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - ball_texture.height() as f32 / 2.0,
        );
        let ball_velocity = Vec2::new(BALL_SPEED as f32, 0 as f32);

        let font_bytes = include_bytes!("../resources/DejaVuSansMono.ttf");
        let font = Font::from_vector_file_data(ctx, font_bytes, 24.0)?;

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
        self.ball.reset();
        self.player1.reset();
        self.player2.reset();
    }

    fn update_positions(&mut self, ctx: &mut Context) {
        if input::is_key_down(ctx, Key::W) {
            self.player1.move_up()
        }
        if input::is_key_down(ctx, Key::S) {
            self.player1.move_down();
        }
        if input::is_key_down(ctx, Key::Up) {
            self.player2.move_up();
        }
        if input::is_key_down(ctx, Key::Down) {
            self.player2.move_down();
        }
        self.ball.move_puck();
    }

    fn check_paddle_hit(&mut self) {
        self.ball.hit(&self.player1);
        self.ball.hit(&self.player2);
    }

    fn check_game_over(&mut self) {
        if self.ball.is_in_p1_goal() {
            self.game_over = true;
            self.player2.score();
            self.p2score_text.set_content(format!("Red: {}", self.player2.get_score()));
        } else if self.ball.is_in_p2_goal(){
            self.game_over = true;
            self.player1.score();
            self.p1score_text.set_content(format!("Blue: {}", self.player1.get_score()));
        }
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> Result<(), tetra::TetraError> {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        self.p1score_text.draw(ctx, Vec2::new(10.0, 10.0));
        self.p2score_text.draw(ctx, Vec2::new(WINDOW_WIDTH - 100 as f32, 10.0));
        if !self.game_over {
            self.player1.draw(ctx);
            self.player2.draw(ctx);
            self.ball.draw(ctx);
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