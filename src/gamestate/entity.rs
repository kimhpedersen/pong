use tetra::graphics::{Texture, Rectangle};
use tetra::math::Vec2;
use tetra::Context;

use crate::gamestate::{WINDOW_HEIGHT, WINDOW_WIDTH, PADDLE_SPEED, PADDLE_SPIN, BALL_ACC};

use super::BALL_SPEED;

pub struct Entity {
    texture: Texture,
    position: Vec2<f32>,
}

impl Entity {
    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity { texture, position}
    }

    pub fn width(&self) -> f32 {
        self.texture.width() as f32
    }
    
    pub fn height(&self) -> f32 {
        self.texture.height() as f32
    }
    
    pub fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }
    pub fn centre(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.width() / 2.0),
            self.position.y + (self.height() / 2.0),
        )
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        self.texture.draw(ctx, self.position);
    }
}

pub struct Player {
    paddle: Entity,
    score: i32,
}

pub struct Ball {
    puck: Entity,
    velocity: Vec2<f32>,
}

impl Player {
    pub fn new(texture: Texture, position: Vec2<f32>) -> Player {
        Player { paddle: Entity::new(texture, position), score: 0 as i32}
    }

    pub fn move_up(&mut self) {
        self.paddle.position.y -= PADDLE_SPEED;
        if self.paddle.position.y < 0.0 {
            self.paddle.position.y = 0.0;
        }
    }

    pub fn move_down(&mut self) {
        self.paddle.position.y += PADDLE_SPEED;
        if self.paddle.position.y + self.paddle.height() > WINDOW_HEIGHT {
            self.paddle.position.y = WINDOW_HEIGHT - self.paddle.height();
        }
    }

    pub fn score(&mut self) {
        self.score += 1;
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        self.paddle.draw(ctx);
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn reset(&mut self) {
        self.paddle.position = Vec2::new(
            self.paddle.position.x,
            WINDOW_HEIGHT / 2.0 - self.paddle.height() / 2.0,
        );
    }
}

impl Ball {
    pub fn new(texture: Texture, position: Vec2<f32>, velovity: Vec2<f32>) -> Ball {
        Ball { puck: Entity::new(texture, position), velocity: velovity}
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        self.puck.draw(ctx);
    }

    pub fn move_puck(&mut self) {
        self.puck.position += self.velocity;
        if self.puck.position.y <= 0.0 || self.puck.position.y + self.puck.height() >= WINDOW_HEIGHT {
            self.velocity.y = -self.velocity.y;
        }
    }

    pub fn reset(&mut self) {
        self.puck.position = Vec2::new(
            WINDOW_WIDTH / 2.0 - self.puck.width() / 2.0,
            WINDOW_HEIGHT / 2.0 - self.puck.height() / 2.0,
        );
        self.velocity = Vec2::new(BALL_SPEED, 0.0);
    }

    pub fn is_in_p1_goal(&self) -> bool {
        self.puck.position.x < 0.0
    }

    pub fn is_in_p2_goal(&self) -> bool {
        self.puck.position.x + self.puck.width() > WINDOW_WIDTH
    }

    pub fn hit(&mut self, player: &Player) {
        if self.puck.bounds().intersects(&player.paddle.bounds()) {
            // Increase the ball's velocity, then flip it.
            self.velocity.x = -(self.velocity.x + (BALL_ACC * self.velocity.x.signum()));
            // Calculate the offset between the paddle and the ball,
            // as a number between -1.0 and 1.0.
            let offset = (player.paddle.centre().y - self.puck.centre().y) / player.paddle.height();
            // Apply the spin to the ball.
            self.velocity.y += PADDLE_SPIN * -offset;
        }
    }
}