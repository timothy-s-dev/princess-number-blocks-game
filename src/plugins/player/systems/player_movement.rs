use crate::common::components::obstacle::Obstacle;
use crate::input_map::Action;
use crate::plugins::player::components::player::{Facing, Player, PlayerState};
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::{Query, Res, Sprite, Time, Transform, Vec3Swizzles, With, Without};
use leafwing_input_manager::prelude::ActionState;

const MOVE_SPEED: f32 = 150.;

type PlayerMovementQuery<'a> = (
    &'a ActionState<Action>,
    &'a mut Transform,
    &'a mut PlayerState,
    &'a mut Facing,
    &'a Sprite,
);

type ObstaclesQuery<'a> = (&'a Sprite, &'a Transform);
type ObstaclesFilter = (With<Obstacle>, Without<Player>);

/// This system uses the [ActionState] component from the player entity to update its state,
/// facing, and position. The key bindings are configured in
/// [get_input_map](crate::plugins::player::components::player::get_input_map).
pub fn player_movement_system(
    mut query: Query<PlayerMovementQuery, With<Player>>,
    obstacles: Query<ObstaclesQuery, ObstaclesFilter>,
    time: Res<Time>,
) {
    let (action_state, mut transform, mut player_state, mut facing, sprite) = query.single_mut();

    // Movement can only interrupt the idle and walking states, return early otherwise
    if *player_state != PlayerState::Walking && *player_state != PlayerState::Idle {
        return;
    }

    let move_vec = action_state.axis_pair(&Action::Move);
    let is_moving = move_vec.length_squared() > 0.;

    // If we were walking, and we've stopped moving, change back to idle state
    if !is_moving && *player_state == PlayerState::Walking {
        *player_state = PlayerState::Idle;
    } else if is_moving {
        // I'll eventually need additional checks here to avoid interrupting other states
        // But for now if there's a movement input we assume that's what we want to do.
        if *player_state != PlayerState::Walking {
            *player_state = PlayerState::Walking;
        }

        // Normalize input and use MOVE_SPEED and delta time to determine change to position
        let normalized_move_vec = move_vec.normalize();
        // X and Y movement are done separately to allow for sliding movement along obstacles
        let move_x = normalized_move_vec.x * MOVE_SPEED * time.delta_secs();
        transform.translation.x += move_x;
        let player_aabb = Aabb2d::new(transform.translation.xy(), sprite.custom_size.unwrap() / 2.);
        for (other_sprite, other_transform) in obstacles.iter() {
            let other_aabb = Aabb2d::new(
                other_transform.translation.xy(),
                other_sprite.custom_size.unwrap() / 2.,
            );
            if player_aabb.intersects(&other_aabb) {
                transform.translation.x -= move_x;
            }
        }

        let move_y = normalized_move_vec.y * MOVE_SPEED * time.delta_secs();
        transform.translation.y += move_y;
        let player_aabb = Aabb2d::new(transform.translation.xy(), sprite.custom_size.unwrap() / 2.);
        for (other_sprite, other_transform) in obstacles.iter() {
            let other_aabb = Aabb2d::new(
                other_transform.translation.xy(),
                other_sprite.custom_size.unwrap() / 2.,
            );
            if player_aabb.intersects(&other_aabb) {
                transform.translation.y -= move_y;
            }
        }

        // Update facing based on directional input
        let new_facing = Facing::from_vec2(move_vec);
        if *facing != new_facing {
            *facing = new_facing;
        }
    }
}
