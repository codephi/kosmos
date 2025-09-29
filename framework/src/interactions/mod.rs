//! Módulo de Interações de Usuário
//! 
//! Fornece funcionalidades para interagir com geometrias:
//! - Drag and Drop: Arraste qualquer geometria
//! - Transform: Crie geometrias interativamente com o mouse
//! - Select: Selecione geometrias com feedback visual
//! 
//! # Exemplo de uso:
//! ```rust
//! use kosmos_framework::interactions::InteractionsPlugin;
//! 
//! App::new()
//!     .add_plugins(DefaultPlugins)
//!     .add_plugin(InteractionsPlugin)
//!     .run();
//! ```

use bevy::prelude::*;

mod components;
mod events;
mod input_utils;
mod drag;
mod transform;
mod select;

// Re-exportar os tipos públicos
pub use components::*;
pub use events::*;
pub use drag::DragPlugin;
pub use transform::TransformPlugin;
pub use select::SelectPlugin;

/// Plugin principal que adiciona todas as funcionalidades de interação
pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Adicionar recursos compartilhados
            .insert_resource(input_utils::MouseWorldPosition(Vec2::ZERO))
            .insert_resource(InteractionMode::None)
            
            // Registrar eventos
            .add_event::<events::StartDragEvent>()
            .add_event::<events::StopDragEvent>()
            .add_event::<events::StartTransformEvent>()
            .add_event::<events::ConfirmTransformEvent>()
            .add_event::<events::CancelTransformEvent>()
            .add_event::<events::SelectEvent>()
            .add_event::<events::DeselectEvent>()
            
            // Adicionar sistemas de atualização de input
            .add_systems(PreUpdate, input_utils::update_mouse_world_position)
            
            // Adicionar sub-plugins
            .add_plugins((
                DragPlugin,
                TransformPlugin,
                SelectPlugin,
            ));
    }
}

/// Modo de interação atual do sistema
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionMode {
    None,
    Dragging,
    Transforming,
    Selecting,
}

/// Prelude para facilitar imports
pub mod prelude {
    pub use super::{
        InteractionsPlugin,
        InteractionMode,
        components::*,
        events::*,
    };
}