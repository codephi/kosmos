//! Módulo Geometrics
//! 
//! Fornece utilitários para criar formas geométricas 2D facilmente no Bevy.
//! 
//! # Exemplo de uso:
//! ```rust
//! use geometrics::{Geometrics, GeometricsExt};
//! 
//! // Método 1: Usando a struct diretamente
//! Geometrics::circle(&mut commands, &mut meshes, &mut materials, Color::RED, 50.0, Vec2::ZERO);
//! 
//! // Método 2: Usando a extensão trait (mais conciso)
//! commands.spawn_circle(&mut meshes, &mut materials, Color::BLUE, 30.0, Vec2::new(100.0, 0.0));
//! ```

mod shapes;
mod extensions;

// Re-exportar os tipos públicos
pub use shapes::Geometrics;
pub use extensions::GeometricsExt;

// Re-exportar um prelude para facilitar imports
pub mod prelude {
    pub use super::{Geometrics, GeometricsExt};
}