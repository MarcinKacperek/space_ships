use amethyst::{
    core::nalgebra::Vector2,
    ecs::{
        Component,
        DenseVecStorage
    }
};

pub mod tags;

pub struct Rect {
    pub width: f32,
    pub height: f32
}

impl Component for Rect {
    type Storage = DenseVecStorage<Self>;
}

pub struct Moveable {
    pub move_speed: f32,
    pub direction: Vector2<f32>
}

impl Component for Moveable {
    type Storage = DenseVecStorage<Self>;
}

pub struct Killable {
    health: i32
}

impl Killable {
    pub fn new(health: i32) -> Self {
        return Self {
            health
        };
    }

    pub fn deal_damage(&mut self) {
        self.health = self.health - 1;
    }

    pub fn get_health(&self) -> i32 {
        return self.health;
    }

    pub fn is_alive(&self) -> bool {
        return self.health > 0;
    }
}

impl Component for Killable {
    type Storage = DenseVecStorage<Self>;
}

pub struct SpaceShip {
    pub is_attacking: bool
}

impl Component for SpaceShip {
    type Storage = DenseVecStorage<Self>;
}

pub struct Missile {
    belongs_to_player: bool
}

impl Missile {
    pub fn new(belongs_to_player: bool) -> Self {
        return Self {
            belongs_to_player
        };
    }
    pub fn belongs_to_player(&self) -> bool {
        return self.belongs_to_player;
    }
}

impl Component for Missile {
    type Storage = DenseVecStorage<Self>;
}

pub struct Cannon {
    pub x_offset: f32,
    pub y_offset: f32,
    pub attack_cooldown: f64,
    pub last_attack_time: f64,
    pub missile_width: f32,
    pub missile_height: f32,
    pub missile_speed: f32,
    pub missile_sprite_index: usize
}

impl Component for Cannon {
    type Storage = DenseVecStorage<Self>;
}