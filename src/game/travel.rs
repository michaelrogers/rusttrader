// Travel and navigation system

use crate::types::GameState;

pub fn warp_to_system(game_state: &mut GameState, target_system_id: usize) -> Result<(), String> {
    let current = game_state.current_system().clone();
    let target = &game_state.solar_systems[target_system_id];
    
    // Calculate distance
    let distance = current.distance_to(target);
    let fuel_needed = distance.ceil() as i32;
    
    // Check if we have enough fuel
    if game_state.ship.fuel < fuel_needed {
        return Err(format!("Not enough fuel. Need {} units, have {}", fuel_needed, game_state.ship.fuel));
    }
    
    // Execute warp
    game_state.ship.fuel -= fuel_needed;
    game_state.current_system_id = target_system_id;
    game_state.days += 1;
    
    // Mark system as visited
    game_state.solar_systems[target_system_id].visited = true;
    
    Ok(())
}

pub fn systems_in_range(game_state: &GameState) -> Vec<(usize, f32)> {
    let current = game_state.current_system();
    let max_range = game_state.ship.fuel as f32;
    
    game_state.solar_systems
        .iter()
        .enumerate()
        .filter_map(|(id, system)| {
            if id == game_state.current_system_id {
                return None;
            }
            let distance = current.distance_to(system);
            if distance <= max_range {
                Some((id, distance))
            } else {
                None
            }
        })
        .collect()
}
