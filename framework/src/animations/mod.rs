//! Módulo de Animações para Kosmos
//! 
//! Fornece um sistema completo de animação baseado em timeline para geometrias 2D no Bevy.
//! 
//! # Exemplo de uso:
//! ```rust
//! use animations::{AnimationTimeline, AnimationBuilder, Easing};
//! 
//! // Criar uma animação com timeline
//! let animation = AnimationBuilder::new()
//!     .move_to(Vec2::new(100.0, 0.0), 1.0, Easing::EaseInOut)
//!     .rotate_to(90.0_f32.to_radians(), 0.5, Easing::Linear)
//!     .scale_to(Vec2::splat(2.0), 1.5, Easing::EaseOut)
//!     .color_to(Color::srgb(1.0, 0.0, 0.0), 1.0, Easing::EaseIn)
//!     .build();
//! ```

mod timeline;
mod keyframe;
mod animation_system;
mod builder;
mod easing;

// Re-exportar tipos públicos
pub use timeline::{AnimationTimeline, AnimationState, AnimationMode};
pub use keyframe::{Keyframe, AnimatableProperty};
pub use animation_system::{AnimationPlugin, AnimationComponent};
pub use builder::{AnimationBuilder, AnimationPresets};
pub use easing::Easing;

// Re-exportar um prelude para facilitar imports
pub mod prelude {
    pub use super::{
        AnimationTimeline,
        AnimationMode,
        AnimationComponent,
        AnimationBuilder,
        AnimationPresets,
        Easing,
        AnimatableProperty,
    };
}
