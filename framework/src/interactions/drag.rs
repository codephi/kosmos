//! Sistema de Drag and Drop para geometrias

use bevy::prelude::*;
use super::{
    components::{Draggable, Dragging, GeometryBounds},
    events::{StartDragEvent, StopDragEvent},
    input_utils::{self, MouseWorldPosition},
    InteractionMode,
};

/// Plugin para funcionalidade de arrastar e soltar
pub struct DragPlugin;

impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                detect_drag_start,
                apply_drag_start,
                update_dragging,
                detect_drag_stop,
            )
                .chain()
                .run_if(not_in_transform_mode),
        );
    }
}

/// Condição para executar apenas quando não estiver em modo de transformação
fn not_in_transform_mode(mode: Res<InteractionMode>) -> bool {
    *mode != InteractionMode::Transforming
}

/// Detecta o início de um arraste quando o botão do mouse é pressionado
fn detect_drag_start(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mouse_pos: Res<MouseWorldPosition>,
    draggable_query: Query<(Entity, &Transform, Option<&GeometryBounds>), (With<Draggable>, Without<Dragging>)>,
    mut drag_start_events: EventWriter<StartDragEvent>,
    mut interaction_mode: ResMut<InteractionMode>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }
    
    if *interaction_mode != InteractionMode::None {
        return;
    }
    
    // Encontrar entidade sob o cursor
    let mut closest_entity = None;
    let mut closest_z = f32::MIN;
    
    for (entity, transform, bounds) in draggable_query.iter() {
        let is_hit = if let Some(bounds) = bounds {
            input_utils::point_in_bounds(
                mouse_pos.0,
                transform.translation,
                bounds.size,
                transform.scale,
            )
        } else {
            input_utils::point_in_bounds(
                mouse_pos.0,
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
    
    if let Some(entity) = closest_entity {
        drag_start_events.send(StartDragEvent {
            entity,
            start_position: mouse_pos.0,
        });
        *interaction_mode = InteractionMode::Dragging;
        
        info!("Iniciando arraste da entidade {:?}", entity);
    }
}

/// Aplica o componente Dragging quando um arraste é iniciado
fn apply_drag_start(
    mut commands: Commands,
    mut drag_events: EventReader<StartDragEvent>,
    transform_query: Query<&Transform>,
) {
    for event in drag_events.read() {
        if let Ok(transform) = transform_query.get(event.entity) {
            let entity_pos = Vec2::new(transform.translation.x, transform.translation.y);
            let offset = entity_pos - event.start_position;
            
            commands.entity(event.entity).insert(Dragging { offset });
            
            debug!("Aplicando componente Dragging com offset: {:?}", offset);
        }
    }
}

/// Atualiza a posição das entidades sendo arrastadas
fn update_dragging(
    mouse_pos: Res<MouseWorldPosition>,
    mut dragging_query: Query<(&mut Transform, &Dragging)>,
) {
    for (mut transform, dragging) in dragging_query.iter_mut() {
        let new_position = mouse_pos.0 + dragging.offset;
        transform.translation.x = new_position.x;
        transform.translation.y = new_position.y;
    }
}

/// Detecta o fim de um arraste quando o botão do mouse é solto
fn detect_drag_stop(
    mouse_button: Res<ButtonInput<MouseButton>>,
    _mouse_pos: Res<MouseWorldPosition>,
    dragging_query: Query<Entity, With<Dragging>>,
    transform_query: Query<&Transform>,
    mut commands: Commands,
    mut drag_stop_events: EventWriter<StopDragEvent>,
    mut interaction_mode: ResMut<InteractionMode>,
) {
    if !mouse_button.just_released(MouseButton::Left) {
        return;
    }
    
    if *interaction_mode != InteractionMode::Dragging {
        return;
    }
    
    for entity in dragging_query.iter() {
        // Remover componente Dragging
        commands.entity(entity).remove::<Dragging>();
        
        // Enviar evento de parada
        if let Ok(transform) = transform_query.get(entity) {
            let final_position = Vec2::new(transform.translation.x, transform.translation.y);
            drag_stop_events.send(StopDragEvent {
                entity,
                final_position,
            });
            
            info!("Finalizando arraste da entidade {:?} na posição {:?}", entity, final_position);
        }
    }
    
    *interaction_mode = InteractionMode::None;
}