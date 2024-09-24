use crate::camera::CameraController;
use crate::input::keyboard::*;
use crate::player::{Player, ViewMode};
use crate::world::{load_chunk_around_player, WORLD_MAP};
use bevy::prelude::*;

fn is_block_at_position(x: i32, y: i32, z: i32) -> bool {
    let map = WORLD_MAP.lock().unwrap();
    match map.get_block(x, y, z) {
        Some(_) => true,
        None => false,
    }
}
// System to move the player based on keyboard input
pub fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player, &mut Handle<StandardMaterial>)>,
    camera_query: Query<&Transform, (With<Camera>, With<CameraController>, Without<Player>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if is_action_just_pressed(GameAction::ToggleViewMode, &keyboard_input) {
        for (_, mut player, _) in player_query.iter_mut() {
            player.toggle_view_mode();
        }
    }

    if is_action_just_pressed(GameAction::ToggleChunkDebugMode, &keyboard_input) {
        for (_, mut player, _) in player_query.iter_mut() {
            player.toggle_chunk_debug_mode();
        }
    }

    let (mut player_transform, mut player, material_handle_mut_ref) = player_query.single_mut();
    let camera_transform = camera_query.single();

    load_chunk_around_player(
        player_transform.translation,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    let material_handle = &*material_handle_mut_ref;

    match player.view_mode {
        ViewMode::FirstPerson => {
            // make player transparent
            if let Some(material) = materials.get_mut(material_handle) {
                material.base_color = Color::srgba(0.0, 0.0, 0.0, 0.0);
            }
        }
        ViewMode::ThirdPerson => {
            if let Some(material) = materials.get_mut(material_handle) {
                material.base_color = Color::srgba(1.0, 0.0, 0.0, 1.0);
            }
        }
    }

    let speed = 5.0;
    let gravity = (-9.8) * 4.0;
    let jump_velocity = 6.0 * 2.0;

    // Calculate movement directions relative to the camera
    let mut forward = camera_transform.forward().xyz();
    forward.y = 0.0;

    let mut right = camera_transform.right().xyz();
    right.y = 0.0;

    let mut direction = Vec3::ZERO;

    // Adjust direction based on key presses
    if is_action_pressed(GameAction::MoveBackward, &keyboard_input) {
        direction -= forward;
    }
    if is_action_pressed(GameAction::MoveForward, &keyboard_input) {
        direction += forward;
    }
    if is_action_pressed(GameAction::MoveLeft, &keyboard_input) {
        direction -= right;
    }
    if is_action_pressed(GameAction::MoveRight, &keyboard_input) {
        direction += right;
    }

    // Move the player (xy plane only), only if there is no blocks
    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
        let new_pos = player_transform.translation + direction * speed * time.delta_seconds();

        // Vérification des collisions devant le joueur (sur l'axe X et Z)
        if !is_block_at_position(
            new_pos.x as i32,
            player_transform.translation.y as i32,
            new_pos.z as i32,
        ) {
            player_transform.translation = new_pos;
        }
    }

    // Handle jumping (if on the ground) and gravity
    // /*player.on_ground &&*/     (temporarily disabled for testing)
    if is_action_pressed(GameAction::Jump, &keyboard_input) {
        // Player can jump only when grounded
        player.vertical_velocity = jump_velocity;
        player.on_ground = false;
    } else if !player.on_ground {
        // Apply gravity when the player is in the air
        player.vertical_velocity += gravity * time.delta_seconds();
    }

    // apply gravity
    let new_y = player_transform.translation.y + player.vertical_velocity * time.delta_seconds();

    // check if there is a bloc underneath the player
    if is_block_at_position(
        player_transform.translation.x as i32,
        (player_transform.translation.y - 1.0) as i32,
        player_transform.translation.z as i32,
    ) {
        // si un bloc est détecté sous le joueur, il reste sur le bloc
        player_transform.translation.y = new_y;
        player.on_ground = true;
        player.vertical_velocity = 0.0; // Réinitialiser la vélocité verticale si le joueur est au sol
    } else {
        // Si aucun bloc n'est détecté sous le joueur, il continue de tomber
        player_transform.translation.y = new_y;
        player.on_ground = false;
    }

    // If the player is below the world, reset their position
    if (player_transform.translation.y - 1.0) < -50.0 {
        player_transform.translation = Vec3::new(0.0, 100.0, 0.0);
        player.vertical_velocity = 0.0;
    }

    /*
    // print coordinates
    println!(
        "Player coordinates: x = {:.2}, y = {:.2}, z = {:.2}",
        player_transform.translation.x,
        player_transform.translation.y,
        player_transform.translation.z
    );
     */
}
