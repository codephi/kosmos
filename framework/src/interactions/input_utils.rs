//! Utilitários de input e conversão de coordenadas

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Recurso que armazena a posição do mouse em coordenadas do mundo
#[derive(Resource, Debug, Clone, Default)]
pub struct MouseWorldPosition(pub Vec2);

/// Sistema que atualiza a posição do mouse em coordenadas do mundo
pub fn update_mouse_world_position(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut mouse_world_pos: ResMut<MouseWorldPosition>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    
    let Ok((camera, camera_transform)) = camera.get_single() else {
        return;
    };
    
    // Converter posição do cursor para coordenadas do mundo
    if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
        mouse_world_pos.0 = world_position;
    }
}

/// Verifica se um ponto está dentro dos limites de uma entidade
pub fn point_in_bounds(
    point: Vec2,
    entity_position: Vec3,
    entity_size: Vec2,
    entity_scale: Vec3,
) -> bool {
    let half_size = entity_size * Vec2::new(entity_scale.x, entity_scale.y) * 0.5;
    let entity_pos_2d = Vec2::new(entity_position.x, entity_position.y);
    
    point.x >= entity_pos_2d.x - half_size.x
        && point.x <= entity_pos_2d.x + half_size.x
        && point.y >= entity_pos_2d.y - half_size.y
        && point.y <= entity_pos_2d.y + half_size.y
}

/// Verifica se um ponto está dentro de um círculo
pub fn point_in_circle(
    point: Vec2,
    center: Vec2,
    radius: f32,
    scale: f32,
) -> bool {
    let distance = point.distance(center);
    distance <= radius * scale
}

/// Encontra a entidade mais próxima sob o cursor
pub fn find_entity_under_cursor(
    cursor_pos: Vec2,
    query: &Query<(Entity, &Transform, Option<&super::components::GeometryBounds>)>,
) -> Option<Entity> {
    let mut closest_entity = None;
    let mut closest_z = f32::MIN;
    
    for (entity, transform, bounds) in query.iter() {
        let is_hit = if let Some(bounds) = bounds {
            point_in_bounds(
                cursor_pos,
                transform.translation,
                bounds.size,
                transform.scale,
            )
        } else {
            // Usar um tamanho padrão se não houver bounds definidos
            point_in_bounds(
                cursor_pos,
                transform.translation,
                Vec2::new(100.0, 100.0),
                transform.scale,
            )
        };
        
        if is_hit && transform.translation.z > closest_z {
            closest_z = transform.translation.z;
            closest_entity = Some(entity);
        }
    }
    
    closest_entity
}

/// Calcula a distância e ângulo entre dois pontos
pub fn calculate_distance_and_angle(from: Vec2, to: Vec2) -> (f32, f32) {
    let delta = to - from;
    let distance = delta.length();
    let angle = delta.y.atan2(delta.x);
    (distance, angle)
}