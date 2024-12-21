use crate::input_map::Action;
use crate::plugins::chest::components::chest::Chest;
use crate::plugins::chest::components::chest_state::ChestState;
use crate::plugins::player::components::interaction_target::InteractionTarget;
use crate::plugins::player::components::player::{Facing, Player, PlayerState};
use crate::plugins::player::components::timers::InteractTimer;
use bevy::ecs::query::QueryData;
use bevy::prelude::{Entity, Query, Res, Time, Timer, TimerMode, Transform, With};
use leafwing_input_manager::prelude::ActionState;
use std::time::Duration;

const MAX_INTERACT_DISTANCE: f32 = 60.;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct PlayerQuery {
    action_state: &'static ActionState<Action>,
    transform: &'static Transform,
    state: &'static mut PlayerState,
    facing: &'static mut Facing,
    interact_timer: &'static mut InteractTimer,
    interact_target: &'static mut InteractionTarget,
}

type ChestsQuery<'a> = (Entity, &'a mut ChestState, &'a Transform);

/// This system uses the [ActionState] component from the player entity to update its state,
/// and facing. The key bindings are configured in
/// [get_input_map](crate::plugins::player::components::player::get_input_map).
pub fn player_interaction_system(
    mut query: Query<PlayerQuery, With<Player>>,
    mut chests: Query<ChestsQuery, With<Chest>>,
    time: Res<Time>,
) {
    let mut player_data = query.single_mut();

    if *player_data.state == PlayerState::Interacting {
        player_data.interact_timer.0.tick(time.delta());
        if player_data.interact_timer.0.just_finished() {
            // This is set below, when starting the timer, so this should always be true
            if let Some(target) = player_data.interact_target.0 {
                let Ok((_, mut chest_state, _)) = chests.get_mut(target) else {
                    return;
                };
                *chest_state = ChestState::Opening;
            }
        }
        return;
    }

    if !player_data.action_state.just_pressed(&Action::Interact) {
        return;
    }

    if *player_data.state != PlayerState::Idle && *player_data.state != PlayerState::Walking {
        return; // Interact can only interrupt the idle and walking states
    }

    let mut min_distance = f32::MAX;
    let mut nearest_chest: Option<Entity> = None;
    for (chest, chest_state, chest_transform) in chests.iter() {
        if *chest_state != ChestState::Closed {
            continue;
        }
        let distance = chest_transform
            .translation
            .distance(player_data.transform.translation);
        if distance < min_distance {
            min_distance = distance;
            nearest_chest = Some(chest);
        }
    }

    // This should only return if there are no chests on the map
    let Some(nearest_chest) = nearest_chest else {
        return;
    };

    // We'll need the offset to determine the facing to apply to the player, and to determine
    // if the chest is even within reach.
    let Ok((_, _, chest_transform)) = chests.get(nearest_chest) else {
        return; // This shouldn't be possible
    };
    let offset = chest_transform.translation - player_data.transform.translation;
    if offset.length() > MAX_INTERACT_DISTANCE {
        return;
    }

    // Update components on player to initiate interaction
    player_data.interact_target.0 = Some(nearest_chest);
    player_data.interact_timer.0 = Timer::new(Duration::from_millis(500), TimerMode::Once);
    *player_data.state = PlayerState::Interacting;
    // Update facing based on direction to nearest chest
    let new_facing = Facing::from_vec3(offset);
    if *player_data.facing != new_facing {
        *player_data.facing = new_facing;
    }
}
