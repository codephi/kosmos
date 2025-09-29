//! Componentes para o sistema de interações

use bevy::prelude::*;

/// Marca uma entidade como arrastável
#[derive(Component, Debug, Clone, Default)]
pub struct Draggable;

/// Estado de arrastar ativo em uma entidade
#[derive(Component, Debug, Clone)]
pub struct Dragging {
    /// Offset do ponto de clique em relação ao centro da entidade
    pub offset: Vec2,
}

/// Marca uma entidade como selecionável
#[derive(Component, Debug, Clone, Default)]
pub struct Selectable;

/// Indica que uma entidade está selecionada
#[derive(Component, Debug, Clone, Default)]
pub struct Selected;

/// Entidade de borda para visualização de seleção
#[derive(Component, Debug, Clone)]
pub struct SelectionBorder {
    /// Entidade pai que está selecionada
    pub parent: Entity,
}

/// Estado de transformação ativo (criação de geometria)
#[derive(Component, Debug, Clone)]
pub struct Transforming {
    /// Ponto pivô onde o mouse foi clicado inicialmente
    pub pivot: Vec2,
    /// Dados da nova geometria sendo criada
    pub geometry_data: NewGeometryData,
}

/// Dados para criação de uma nova geometria
#[derive(Debug, Clone)]
pub struct NewGeometryData {
    /// Tipo de forma geométrica
    pub shape: GeometryShape,
    /// Cor da geometria
    pub color: Color,
    /// Se a forma deve ser preenchida
    pub filled: bool,
}

impl Default for NewGeometryData {
    fn default() -> Self {
        Self {
            shape: GeometryShape::Rectangle,
            color: Color::WHITE,
            filled: true,
        }
    }
}

/// Tipos de formas geométricas suportadas
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometryShape {
    Circle,
    Square,
    Rectangle,
    Triangle,
    Pentagon,
    Hexagon,
    Star,
    Heart,
    Custom,
}

/// Marca uma geometria em modo preview (sendo criada)
#[derive(Component, Debug, Clone, Default)]
pub struct GeometryPreview;

/// Componente para armazenar os limites de uma geometria (para hit testing)
#[derive(Component, Debug, Clone)]
pub struct GeometryBounds {
    /// Tamanho da caixa delimitadora
    pub size: Vec2,
}