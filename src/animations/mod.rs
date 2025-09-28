//! Módulo de Animações
//! 
//! Sistema de animações inspirado no Framer Motion para animar geometrias no Bevy.
//! 
//! # Exemplo de uso:
//! ```rust
//! use animations::prelude::*;
//! 
//! Motion::new()
//!     .from(MotionState::position(0.0, 0.0))
//!     .to(MotionState::position(100.0, 100.0))
//!     .duration(2.0)
//!     .easing(Easing::EaseInOut)
//!     .spawn(&mut commands, entity);
//! ```

mod components;
mod systems;
mod easing;
mod builder;

pub use components::*;
pub use systems::*;
pub use easing::*;
pub use builder::*;

use bevy::prelude::*;

/// Plugin principal de animações
pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_animations,
                update_transform_animations,
                update_color_animations,
                update_scale_animations,
                update_rotation_animations,
                cleanup_finished_animations,
            ).chain());
    }
}

pub mod prelude {
    pub use super::{
        AnimationsPlugin,
        Motion,
        MotionState,
        AnimationComponent,
        Easing,
        AnimationStatus,
        TransformAnimation,
        ColorAnimation,
        ScaleAnimation,
        RotationAnimation,
    };
}