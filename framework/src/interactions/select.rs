//! Sistema de seleção com feedback visual

use bevy::prelude::*;
use bevy::render::mesh::Mesh2d;
use bevy::sprite::MeshMaterial2d;
use super::{
    components::{Selectable, Selected, SelectionBorder, GeometryBounds},
    events::{SelectEvent, DeselectEvent},
    input_utils::{self, MouseWorldPosition},
    InteractionMode,
};

/// Plugin para funcionalidade de seleção
pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrentSelection::default())
            .add_systems(
                Update,
                (
                    detect_selection_click,
                    apply_selection,
                    apply_deselection,
                    update_selection_border,
                )
                    .chain()
                    .run_if(can_select),
            );
    }
}

/// Recurso para rastrear a seleção atual
#[derive(Resource, Default)]
struct CurrentSelection {
    selected_entity: Option<Entity>,
    border_entity: Option<Entity>,
}

/// Condição para permitir seleção apenas quando não estiver em outro modo
fn can_select(mode: Res<InteractionMode>) -> bool {
    *mode == InteractionMode::None || *mode == InteractionMode::Selecting
}

/// Detecta cliques para seleção
fn detect_selection_click(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mouse_pos: Res<MouseWorldPosition>,
    selectable_query: Query<(Entity, &Transform, Option<&GeometryBounds>), (With<Selectable>, Without<Selected>)>,
    mut select_events: EventWriter<SelectEvent>,
    mut deselect_events: EventWriter<DeselectEvent>,
    mut interaction_mode: ResMut<InteractionMode>,
    current_selection: Res<CurrentSelection>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }
    
    if *interaction_mode == InteractionMode::Dragging || *interaction_mode == InteractionMode::Transforming {
        return;
    }
    
    // Verificar se há uma entidade selecionável sob o cursor
    let mut closest_entity = None;
    let mut closest_z = f32::MIN;
    
    for (entity, transform, bounds) in selectable_query.iter() {
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
        // Desselecionar a entidade atual se houver
        if let Some(current) = current_selection.selected_entity {
            if current != entity {
                deselect_events.send(DeselectEvent { entity: current });
            }
        }
        
        // Selecionar a nova entidade
        select_events.send(SelectEvent { entity });
        *interaction_mode = InteractionMode::Selecting;
        
        info!("Selecionando entidade: {:?}", entity);
    } else {
        // Clicar em área vazia desseleciona tudo
        if let Some(current) = current_selection.selected_entity {
            deselect_events.send(DeselectEvent { entity: current });
            *interaction_mode = InteractionMode::None;
        }
    }
}

/// Aplica o estado de seleção e cria a borda visual
fn apply_selection(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut select_events: EventReader<SelectEvent>,
    transform_query: Query<(&Transform, Option<&GeometryBounds>)>,
    mut current_selection: ResMut<CurrentSelection>,
) {
    for event in select_events.read() {
        // Adicionar componente Selected
        commands.entity(event.entity).insert(Selected);
        
        // Obter informações da entidade selecionada
        if let Ok((transform, bounds)) = transform_query.get(event.entity) {
            // Calcular tamanho da borda
            let bounds_size = bounds.map(|b| b.size).unwrap_or(Vec2::new(100.0, 100.0));
            let border_size = bounds_size * transform.scale.truncate() + Vec2::splat(10.0);
            
            // Criar mesh de borda (retângulo vazado)
            let border_mesh = create_border_mesh(border_size);
            
            // Criar material da borda (cor de destaque)
            let border_material = ColorMaterial::from(Color::srgb(1.0, 1.0, 0.0)); // Amarelo
            
            // Spawnar entidade da borda como filho
            let border_entity = commands
                .spawn((
                Mesh2d(meshes.add(border_mesh)),
                MeshMaterial2d(materials.add(border_material)),
                    Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)), // Levemente acima
                    SelectionBorder {
                        parent: event.entity,
                    },
                ))
                .id();
            
            // Fazer a borda ser filho da entidade selecionada
            commands.entity(event.entity).add_child(border_entity);
            
            // Atualizar rastreamento
            current_selection.selected_entity = Some(event.entity);
            current_selection.border_entity = Some(border_entity);
            
            debug!("Borda de seleção criada: {:?}", border_entity);
        }
    }
}

/// Remove o estado de seleção e a borda visual
fn apply_deselection(
    mut commands: Commands,
    mut deselect_events: EventReader<DeselectEvent>,
    mut current_selection: ResMut<CurrentSelection>,
    border_query: Query<Entity, With<SelectionBorder>>,
) {
    for event in deselect_events.read() {
        // Remover componente Selected
        commands.entity(event.entity).remove::<Selected>();
        
        // Despawnar a borda se existir
        if let Some(border_entity) = current_selection.border_entity {
            if border_query.contains(border_entity) {
                commands.entity(border_entity).despawn_recursive();
            }
        }
        
        // Limpar rastreamento
        current_selection.selected_entity = None;
        current_selection.border_entity = None;
        
        info!("Entidade desselecionada: {:?}", event.entity);
    }
}

/// Atualiza a posição/escala da borda conforme a entidade pai muda
fn update_selection_border(
    parent_query: Query<(&Transform, Option<&GeometryBounds>), (With<Selected>, Changed<Transform>)>,
    mut border_query: Query<(&mut Transform, &SelectionBorder), Without<Selected>>,
) {
    for (mut border_transform, selection_border) in border_query.iter_mut() {
        if let Ok((parent_transform, bounds)) = parent_query.get(selection_border.parent) {
            // Atualizar escala da borda se necessário
            let bounds_size = bounds.map(|b| b.size).unwrap_or(Vec2::new(100.0, 100.0));
            let _new_border_size = bounds_size * parent_transform.scale.truncate() + Vec2::splat(10.0);
            
            // Nota: Em produção, você poderia querer atualizar a mesh também
            // Por simplicidade, apenas ajustamos a posição relativa aqui
            border_transform.translation.z = 0.1; // Manter levemente acima
        }
    }
}

/// Cria uma mesh de borda (retângulo vazado)
fn create_border_mesh(size: Vec2) -> Mesh {
    use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};
    use bevy::render::render_asset::RenderAssetUsages;
    
    let half = size * 0.5;
    let thickness = 2.0; // Espessura da borda
    
    // Vértices externos
    let outer_vertices = vec![
        [-half.x - thickness, -half.y - thickness, 0.0],
        [half.x + thickness, -half.y - thickness, 0.0],
        [half.x + thickness, half.y + thickness, 0.0],
        [-half.x - thickness, half.y + thickness, 0.0],
    ];
    
    // Vértices internos
    let inner_vertices = vec![
        [-half.x + thickness, -half.y + thickness, 0.0],
        [half.x - thickness, -half.y + thickness, 0.0],
        [half.x - thickness, half.y - thickness, 0.0],
        [-half.x + thickness, half.y - thickness, 0.0],
    ];
    
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    vertices.extend(&outer_vertices);
    vertices.extend(&inner_vertices);
    
    // Índices para formar a borda (8 triângulos para o retângulo vazado)
    let indices = vec![
        // Bottom edge
        0, 1, 5,
        0, 5, 4,
        // Right edge
        1, 2, 6,
        1, 6, 5,
        // Top edge
        2, 3, 7,
        2, 7, 6,
        // Left edge
        3, 0, 4,
        3, 4, 7,
    ];
    
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_indices(Indices::U32(indices));
    
    // Adicionar normais e UVs
    let normals = vec![[0.0, 0.0, 1.0]; 8];
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    
    let uvs = vec![[0.0, 0.0]; 8];
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    
    mesh
}