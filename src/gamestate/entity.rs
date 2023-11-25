use tetra::graphics::{Texture, Rectangle};
use tetra::math::Vec2;

pub struct Entity {
    pub texture: Texture,
    pub position: Vec2<f32>,
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
}

pub struct Player {
    pub paddle: Entity,
    pub score: i32,
}

pub struct Ball {
    pub puck: Entity,
    pub velocity: Vec2<f32>,
}

impl Player {
    pub fn new(texture: Texture, position: Vec2<f32>) -> Player {
        Player { paddle: Entity::new(texture, position), score: 0 as i32}
    }
}

impl Ball {
    pub fn new(texture: Texture, position: Vec2<f32>, velovity: Vec2<f32>) -> Ball {
        Ball { puck: Entity::new(texture, position), velocity: velovity}
    }
}
