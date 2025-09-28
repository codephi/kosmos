use bevy::prelude::*;
use bevy::sprite::MeshMaterial2d;
use super::timeline::AnimationTimeline;

/// Componente que marca uma entidade como animada
#[derive(Component)]
pub struct AnimationComponent {
    /// Timeline da animação
    pub timeline: AnimationTimeline,
    /// Se deve aplicar automaticamente as transformações
    pub auto_apply: bool,
}

impl AnimationComponent {
    /// Cria um novo componente de animação
    pub fn new(timeline: AnimationTimeline) -> Self {
        Self {
            timeline,
            auto_apply: true,
        }
    }
    
    /// Cria com auto_apply customizado
    pub fn with_auto_apply(timeline: AnimationTimeline, auto_apply: bool) -> Self {
        Self {
            timeline,
            auto_apply,
        }
    }
}

/// Plugin de animação que adiciona os sistemas necessários
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_animations)
            .add_systems(Update, apply_animation_properties.after(update_animations));
    }
}

/// Sistema que atualiza todas as animações ativas
pub fn update_animations(
    time: Res<Time>,
    mut query: Query<&mut AnimationComponent>,
) {
    let delta = time.delta_secs();
    
    for mut animation in query.iter_mut() {
        animation.timeline.update(delta);
    }
}

/// Sistema que aplica as propriedades animadas às entidades
pub fn apply_animation_properties(
    mut query: Query<(
        &AnimationComponent,
        &mut Transform,
        Option<&MeshMaterial2d<ColorMaterial>>,
    )>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (animation, mut transform, material_component) in query.iter_mut() {
        if !animation.auto_apply || !animation.timeline.is_playing() {
            continue;
        }
        
        let values = animation.timeline.get_current_values();
        
        // Aplicar propriedades ao Transform
        for (_, property) in values.iter() {
            property.apply_to_transform(&mut transform);
            
            // Aplicar propriedades ao material se existir
            if let Some(material_component) = material_component {
                if let Some(material) = materials.get_mut(&material_component.0) {
                    property.apply_to_material(material);
                }
            }
        }
    }
}

/// Sistema auxiliar para controlar animações via eventos
#[derive(Event)]
pub enum AnimationEvent {
    Play(Entity),
    Pause(Entity),
    Stop(Entity),
    Restart(Entity),
}

/// Sistema que processa eventos de animação
pub fn handle_animation_events(
    mut events: EventReader<AnimationEvent>,
    mut query: Query<&mut AnimationComponent>,
) {
    for event in events.read() {
        match event {
            AnimationEvent::Play(entity) => {
                if let Ok(mut animation) = query.get_mut(*entity) {
                    animation.timeline.play();
                }
            }
            AnimationEvent::Pause(entity) => {
                if let Ok(mut animation) = query.get_mut(*entity) {
                    animation.timeline.pause();
                }
            }
            AnimationEvent::Stop(entity) => {
                if let Ok(mut animation) = query.get_mut(*entity) {
                    animation.timeline.stop();
                }
            }
            AnimationEvent::Restart(entity) => {
                if let Ok(mut animation) = query.get_mut(*entity) {
                    animation.timeline.restart();
                }
            }
        }
    }
}

/// Helper trait para adicionar animações facilmente a entidades
pub trait AnimateEntity {
    fn animate(&mut self, timeline: AnimationTimeline) -> &mut Self;
}

impl AnimateEntity for EntityCommands<'_> {
    fn animate(&mut self, timeline: AnimationTimeline) -> &mut Self {
        self.insert(AnimationComponent::new(timeline));
        self
    }
}