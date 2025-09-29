//! Sistema de Transform Mode para criação interativa de geometrias

use bevy::prelude::*;
use bevy::render::mesh::Mesh2d;
use bevy::sprite::MeshMaterial2d;
use super::{
    components::{
        Transforming, NewGeometryData, GeometryShape, GeometryPreview,
        Draggable, Selectable, GeometryBounds,
    },
    events::{StartTransformEvent, ConfirmTransformEvent, CancelTransformEvent},
    input_utils::{MouseWorldPosition, calculate_distance_and_angle},
    InteractionMode,
};
// Geometrics pode ser usado em implementações futuras

/// Plugin para modo de transformação/criação de geometrias
pub struct TransformPlugin;

impl Plugin for TransformPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrentTransformData::default())
            .add_systems(
                Update,
                (
                    detect_transform_start,
                    begin_transform,
                    update_transform_preview,
                    confirm_transform,
                    cancel_transform,
                )
                    .chain(),
            );
    }
}

/// Recurso para armazenar dados da transformação atual
#[derive(Resource, Default)]
struct CurrentTransformData {
    preview_entity: Option<Entity>,
}

/// Detecta o início do modo de transformação (tecla T ou G)
fn detect_transform_start(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_pos: Res<MouseWorldPosition>,
    mut transform_events: EventWriter<StartTransformEvent>,
    interaction_mode: Res<InteractionMode>,
) {
    if *interaction_mode != InteractionMode::None {
        return;
    }
    
    // Tecla T para criar retângulo, G para criar círculo
    let mut geometry_data = None;
    
    if keyboard.just_pressed(KeyCode::KeyT) {
        geometry_data = Some(NewGeometryData {
            shape: GeometryShape::Rectangle,
            color: Color::srgb(0.3, 0.7, 0.9),
            filled: true,
        });
    } else if keyboard.just_pressed(KeyCode::KeyG) {
        geometry_data = Some(NewGeometryData {
            shape: GeometryShape::Circle,
            color: Color::srgb(0.9, 0.3, 0.7),
            filled: true,
        });
    } else if keyboard.just_pressed(KeyCode::KeyR) {
        geometry_data = Some(NewGeometryData {
            shape: GeometryShape::Triangle,
            color: Color::srgb(0.7, 0.9, 0.3),
            filled: true,
        });
    } else if keyboard.just_pressed(KeyCode::KeyH) {
        geometry_data = Some(NewGeometryData {
            shape: GeometryShape::Hexagon,
            color: Color::srgb(0.5, 0.5, 0.9),
            filled: true,
        });
    } else if keyboard.just_pressed(KeyCode::KeyY) {
        geometry_data = Some(NewGeometryData {
            shape: GeometryShape::Star,
            color: Color::srgb(1.0, 0.8, 0.2),
            filled: true,
        });
    }
    
    if let Some(data) = geometry_data {
        let shape = data.shape;
        transform_events.send(StartTransformEvent {
            geometry_data: data,
            start_position: mouse_pos.0,
        });
        
        info!("Iniciando modo de transformação para criar {:?}", shape);
    }
}

/// Inicia o modo de transformação criando uma entidade preview
fn begin_transform(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut transform_events: EventReader<StartTransformEvent>,
    mut interaction_mode: ResMut<InteractionMode>,
    mut transform_data: ResMut<CurrentTransformData>,
) {
    for event in transform_events.read() {
        *interaction_mode = InteractionMode::Transforming;
        
        // Criar uma geometria inicial pequena
        let initial_size = 10.0;
        let mut material = ColorMaterial::from(event.geometry_data.color);
        material.color.set_alpha(0.7); // Transparência para preview
        
        let mesh = match event.geometry_data.shape {
            GeometryShape::Circle => meshes.add(Circle::new(initial_size)),
            GeometryShape::Rectangle | GeometryShape::Square => {
                meshes.add(Rectangle::new(initial_size, initial_size))
            }
            GeometryShape::Triangle => {
                meshes.add(Triangle2d::new(
                    Vec2::new(0.0, initial_size),
                    Vec2::new(-initial_size, -initial_size),
                    Vec2::new(initial_size, -initial_size),
                ))
            }
            GeometryShape::Hexagon | GeometryShape::Pentagon => {
                let sides = match event.geometry_data.shape {
                    GeometryShape::Pentagon => 5,
                    _ => 6,
                };
                meshes.add(RegularPolygon::new(initial_size, sides))
            }
            _ => meshes.add(Circle::new(initial_size)), // Fallback
        };
        
        let entity = commands
            .spawn((
                Mesh2d(mesh),
                MeshMaterial2d(materials.add(material)),
                Transform::from_translation(Vec3::new(
                    event.start_position.x,
                    event.start_position.y,
                    10.0, // Z alto para ficar acima de outras geometrias
                )),
                Transforming {
                    pivot: event.start_position,
                    geometry_data: event.geometry_data.clone(),
                },
                GeometryPreview,
            ))
            .id();
        
        transform_data.preview_entity = Some(entity);
        
        debug!("Preview entity criada: {:?}", entity);
    }
}

/// Atualiza o preview da geometria baseado na posição do mouse
fn update_transform_preview(
    mouse_pos: Res<MouseWorldPosition>,
    mut transform_query: Query<(&mut Transform, &Transforming), With<GeometryPreview>>,
) {
    for (mut transform, transforming) in transform_query.iter_mut() {
        let (distance, angle) = calculate_distance_and_angle(transforming.pivot, mouse_pos.0);
        
        // Escalar baseado na distância
        let scale_factor = (distance / 50.0).max(0.1);
        
        match transforming.geometry_data.shape {
            GeometryShape::Circle => {
                // Círculo escala uniformemente
                transform.scale = Vec3::splat(scale_factor);
            }
            GeometryShape::Rectangle => {
                // Retângulo escala diferentemente em X e Y
                let delta = mouse_pos.0 - transforming.pivot;
                transform.scale.x = (delta.x.abs() / 50.0).max(0.1);
                transform.scale.y = (delta.y.abs() / 50.0).max(0.1);
            }
            _ => {
                // Outras formas escalam uniformemente
                transform.scale = Vec3::splat(scale_factor);
            }
        }
        
        // Rotacionar em direção ao mouse
        transform.rotation = Quat::from_rotation_z(angle);
        
        // Manter posição no pivô
        transform.translation.x = transforming.pivot.x;
        transform.translation.y = transforming.pivot.y;
    }
}

/// Confirma a criação da geometria ao clicar
fn confirm_transform(
    mouse_button: Res<ButtonInput<MouseButton>>,
    transform_query: Query<(Entity, &Transform, &Transforming), With<GeometryPreview>>,
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
    mut confirm_events: EventWriter<ConfirmTransformEvent>,
    mut interaction_mode: ResMut<InteractionMode>,
    mut transform_data: ResMut<CurrentTransformData>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }
    
    if *interaction_mode != InteractionMode::Transforming {
        return;
    }
    
    for (entity, transform, transforming) in transform_query.iter() {
        // Remover componentes de preview
        commands.entity(entity).remove::<GeometryPreview>();
        commands.entity(entity).remove::<Transforming>();
        
        // Adicionar componentes de interação
        let bounds_size = match transforming.geometry_data.shape {
            GeometryShape::Circle => {
                Vec2::splat(100.0 * transform.scale.x) // Diâmetro
            }
            GeometryShape::Rectangle => {
                Vec2::new(100.0 * transform.scale.x, 100.0 * transform.scale.y)
            }
            _ => Vec2::splat(100.0 * transform.scale.x),
        };
        
        commands.entity(entity).insert((
            Draggable,
            Selectable,
            GeometryBounds { size: bounds_size },
        ));
        
        // Material já será opaco na criação final
        
        confirm_events.send(ConfirmTransformEvent { entity });
        
        info!("Geometria confirmada: {:?}", entity);
    }
    
    transform_data.preview_entity = None;
    *interaction_mode = InteractionMode::None;
}

/// Cancela a criação da geometria ao pressionar ESC
fn cancel_transform(
    keyboard: Res<ButtonInput<KeyCode>>,
    transform_query: Query<Entity, With<GeometryPreview>>,
    mut commands: Commands,
    mut cancel_events: EventWriter<CancelTransformEvent>,
    mut interaction_mode: ResMut<InteractionMode>,
    mut transform_data: ResMut<CurrentTransformData>,
) {
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }
    
    if *interaction_mode != InteractionMode::Transforming {
        return;
    }
    
    for entity in transform_query.iter() {
        commands.entity(entity).despawn_recursive();
        cancel_events.send(CancelTransformEvent);
        
        info!("Criação de geometria cancelada");
    }
    
    transform_data.preview_entity = None;
    *interaction_mode = InteractionMode::None;
}