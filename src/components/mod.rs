use amethyst::ecs::{
    Component,
    DenseVecStorage
};
use amethyst::core::nalgebra::Vector2;

pub mod tags;
pub mod data;

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

    pub fn deal_damage(&mut self, damage: i32) {
        if damage > 0 {
            self.health = self.health - damage;
        }
    }

    pub fn is_alive(&self) -> bool {
        return self.health > 0;
    }
}

impl Component for Killable {
    type Storage = DenseVecStorage<Self>;
}

pub struct SpaceShip {
    pub attack_cooldown: f64,
    pub last_attack_time: f64,
    pub is_attacking: bool
    // Missile (color)? Damage? Prefab?
}

impl Component for SpaceShip {
    type Storage = DenseVecStorage<Self>;
}

pub struct Missile {
    damage: i32,
    belongs_to_player: bool
}

impl Missile {
    pub fn new(damage: i32, belongs_to_player: bool) -> Self {
        return Self {
            damage,
            belongs_to_player
        };
    }

    pub fn get_damage(&self) -> i32 {
        return self.damage;
    }

    pub fn belongs_to_player(&self) -> bool {
        return self.belongs_to_player;
    }
}

impl Component for Missile {
    type Storage = DenseVecStorage<Self>;
}