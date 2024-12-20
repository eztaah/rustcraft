use bevy::prelude::*;
use bevy_atmosphere::prelude::AtmosphereCamera;
use bevy_mod_raycast::prelude::*;

use crate::GameState;

#[derive(TypePath)]
pub struct BlockRaycastSet;

#[derive(Component)]
pub struct CameraController {
    pub distance: f32,
    pub angle_x: f32,
    pub angle_y: f32,
    pub mouse_sensitivity: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            distance: 10.0,
            angle_x: 0.0,
            angle_y: 20.0f32.to_radians(),
            mouse_sensitivity: 0.003,
        }
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 5.0, 10.0))
                .looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
            projection: Projection::Perspective(PerspectiveProjection {
                fov: f32::to_radians(60.0),
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(CameraController::default()) // Ajoute le CameraController
        .insert({
            let mut raycast_source = RaycastSource::<BlockRaycastSet>::default(); // Initialisation par défaut
            raycast_source.cast_method = RaycastMethod::Transform; // Utilise la transformation de la caméra pour lancer le rayon
            raycast_source // Retourne l'objet
        })
        .insert(AtmosphereCamera::default())
        .insert(StateScoped(GameState::Game));
}
