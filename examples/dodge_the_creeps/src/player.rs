use crate::extensions::NodeExt as _;
use gdnative::api::{AnimatedSprite, Area2D, CollisionShape2D, PhysicsBody2D};
use gdnative::prelude::*;

/// The player "class"
#[derive(NativeClass)]
#[inherit(Area2D)]
#[user_data(user_data::MutexData<Player>)]
#[register_with(Self::register_player)]
pub struct Player {
    #[property(default = 400.0)]
    speed: f32,

    screen_size: Vector2,
}

#[methods]
impl Player {
    fn register_player(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "hit",
            args: &[],
        });
    }

    fn new(_owner: &Area2D) -> Self {
        Player {
            speed: 400.0,
            screen_size: Vector2::new(0.0, 0.0),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Area2D) {
        let viewport = unsafe { owner.get_viewport().unwrap().assume_safe() };
        self.screen_size = viewport.size();
        owner.hide();
    }

    #[export]
    fn _process(&mut self, owner: &Area2D, delta: f32) {
        let animated_sprite =
            unsafe { owner.get_typed_node::<AnimatedSprite, _>("animated_sprite") };

        let input = Input::godot_singleton();
        let mut velocity = Vector2::new(0.0, 0.0);

        if Input::is_action_pressed(&input, "ui_right") {
            velocity.x += 1.0
        }
        if Input::is_action_pressed(&input, "ui_left") {
            velocity.x -= 1.0
        }
        if Input::is_action_pressed(&input, "ui_down") {
            velocity.y += 1.0
        }
        if Input::is_action_pressed(&input, "ui_up") {
            velocity.y -= 1.0
        }

        if velocity.length() > 0.0 {
            velocity = velocity.normalize() * self.speed;

            let animation;

            if velocity.x != 0.0 {
                animation = "right";

                animated_sprite.set_flip_v(false);
                animated_sprite.set_flip_h(velocity.x < 0.0)
            } else {
                animation = "up";

                animated_sprite.set_flip_v(velocity.y > 0.0)
            }

            animated_sprite.play(animation, false);
        } else {
            animated_sprite.stop();
        }

        let change = velocity * delta;
        let position =
            (owner.global_position() + change).clamp(Vector2::new(0.0, 0.0), self.screen_size);
        owner.set_global_position(position);
    }

    #[export]
    fn on_player_body_entered(&self, owner: &Area2D, _body: Ref<PhysicsBody2D>) {
        owner.hide();
        owner.emit_signal("hit", &[]);

        let collision_shape =
            unsafe { owner.get_typed_node::<CollisionShape2D, _>("collision_shape_2d") };

        collision_shape.set_deferred("disabled", true);
    }

    #[export]
    pub fn start(&self, owner: &Area2D, pos: Vector2) {
        owner.set_global_position(pos);
        owner.show();

        let collision_shape =
            unsafe { owner.get_typed_node::<CollisionShape2D, _>("collision_shape_2d") };

        collision_shape.set_disabled(false);
    }
}
