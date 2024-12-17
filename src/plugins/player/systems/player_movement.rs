use crate::input_map::Action;
use crate::plugins::player::components::player::{Facing, Player, PlayerState};
use bevy::prelude::{Query, Res, Time, Transform, With};
use leafwing_input_manager::prelude::ActionState;

const MOVE_SPEED: f32 = 100.;

type PlayerMovementQuery<'a> = (
    &'a ActionState<Action>,
    &'a mut Transform,
    &'a mut PlayerState,
    &'a mut Facing,
);

/// This system uses the [ActionState] component from the player entity to update its state,
/// facing, and position. The key bindings are configured in
/// [get_input_map](crate::plugins::player::components::player::get_input_map).
pub fn player_movement_system(
    mut query: Query<PlayerMovementQuery, With<Player>>,
    time: Res<Time>,
) {
    let (action_state, mut transform, mut player_state, mut facing) = query.single_mut();
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
        transform.translation.x += normalized_move_vec.x * MOVE_SPEED * time.delta_secs();
        transform.translation.y += normalized_move_vec.y * MOVE_SPEED * time.delta_secs();

        // Update facing based on directional input
        // If both are the same magnitude (exact diagonal) default to horizontal facings
        let new_facing = if move_vec.y.abs() > move_vec.x.abs() {
            if move_vec.y < 0. {
                Facing::South
            } else {
                Facing::North
            }
        } else if move_vec.x < 0. {
            Facing::West
        } else {
            Facing::East
        };

        if *facing != new_facing {
            *facing = new_facing;
        }
    }
}
