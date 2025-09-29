//! Eventos do sistema de interações

use bevy::prelude::*;
use super::components::NewGeometryData;

/// Evento disparado quando uma entidade começa a ser arrastada
#[derive(Event, Debug, Clone)]
pub struct StartDragEvent {
    pub entity: Entity,
    pub start_position: Vec2,
}

/// Evento disparado quando uma entidade para de ser arrastada
#[derive(Event, Debug, Clone)]
pub struct StopDragEvent {
    pub entity: Entity,
    pub final_position: Vec2,
}

/// Evento disparado para iniciar o modo de transformação (criação de geometria)
#[derive(Event, Debug, Clone)]
pub struct StartTransformEvent {
    pub geometry_data: NewGeometryData,
    pub start_position: Vec2,
}

/// Evento disparado para confirmar a criação de uma geometria
#[derive(Event, Debug, Clone)]
pub struct ConfirmTransformEvent {
    pub entity: Entity,
}

/// Evento disparado para cancelar a criação de uma geometria
#[derive(Event, Debug, Clone)]
pub struct CancelTransformEvent;

/// Evento disparado quando uma entidade é selecionada
#[derive(Event, Debug, Clone)]
pub struct SelectEvent {
    pub entity: Entity,
}

/// Evento disparado quando uma entidade é desselecionada
#[derive(Event, Debug, Clone)]
pub struct DeselectEvent {
    pub entity: Entity,
}

/// Evento para mudança de modo de interação
#[derive(Event, Debug, Clone)]
pub struct ChangeModeEvent {
    pub new_mode: super::InteractionMode,
}