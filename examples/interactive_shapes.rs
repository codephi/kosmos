//! Exemplo demonstrando as funcionalidades de interação:
//! - Drag and Drop: Arraste qualquer forma com o mouse
//! - Transform Mode: Pressione T (retângulo), G (círculo), R (triângulo), H (hexágono), Y (estrela)
//! - Selection: Clique em uma forma para selecioná-la
//! 
//! Controles:
//! - Mouse Left: Arrastar/Selecionar/Confirmar criação
//! - T: Criar retângulo
//! - G: Criar círculo
//! - R: Criar triângulo
//! - H: Criar hexágono
//! - Y: Criar estrela
//! - ESC: Cancelar criação

use bevy::prelude::*;
use kosmos_framework::{
    geometrics::prelude::*,
    interactions::prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InteractionsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, show_instructions)
        .run();
}

/// Configura a cena inicial
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Câmera 2D
    commands.spawn(Camera2d::default());
    
    // Criar algumas geometrias iniciais
    
    // Círculo azul
    let circle = Geometrics::circle(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::srgb(0.3, 0.5, 0.9),
        50.0,
        Vec2::new(-200.0, 100.0),
    );
    commands.entity(circle).insert((
        Draggable,
        Selectable,
        GeometryBounds { size: Vec2::splat(100.0) },
    ));
    
    // Quadrado vermelho
    let square = Geometrics::square(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::srgb(0.9, 0.3, 0.3),
        80.0,
        Vec2::new(0.0, 100.0),
    );
    commands.entity(square).insert((
        Draggable,
        Selectable,
        GeometryBounds { size: Vec2::splat(80.0) },
    ));
    
    // Triângulo verde
    let triangle = Geometrics::triangle(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::srgb(0.3, 0.9, 0.3),
        60.0,
        Vec2::new(200.0, 100.0),
    );
    commands.entity(triangle).insert((
        Draggable,
        Selectable,
        GeometryBounds { size: Vec2::splat(60.0) },
    ));
    
    // Hexágono roxo
    let hexagon = Geometrics::polygon(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::srgb(0.7, 0.3, 0.9),
        6,
        40.0,
        Vec2::new(-200.0, -100.0),
    );
    commands.entity(hexagon).insert((
        Draggable,
        Selectable,
        GeometryBounds { size: Vec2::splat(80.0) },
    ));
    
    // Estrela amarela
    let star = Geometrics::draw_star(
        &mut commands,
        &mut meshes,
        &mut materials,
        5,
        50.0,
        25.0,
        Color::srgb(1.0, 0.9, 0.2),
        Vec2::new(0.0, -100.0),
        true,
    );
    commands.entity(star).insert((
        Draggable,
        Selectable,
        GeometryBounds { size: Vec2::splat(100.0) },
    ));
    
    // Coração rosa
    let heart = Geometrics::draw_heart(
        &mut commands,
        &mut meshes,
        &mut materials,
        30.0,
        Color::srgb(1.0, 0.4, 0.6),
        Vec2::new(200.0, -100.0),
        true,
    );
    commands.entity(heart).insert((
        Draggable,
        Selectable,
        GeometryBounds { size: Vec2::splat(60.0) },
    ));
    
    // Texto de instruções
    commands.spawn((
        Text2d::new("Interactive Shapes Demo\n\nControles:\n[Mouse] Arrastar/Selecionar\n[T] Criar Retângulo\n[G] Criar Círculo\n[R] Criar Triângulo\n[H] Criar Hexágono\n[Y] Criar Estrela\n[ESC] Cancelar"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 300.0, 100.0),
    ));
}

/// Sistema para mostrar informações sobre o modo atual
fn show_instructions(
    mode: Res<InteractionMode>,
    mut query: Query<(&mut Text2d, &mut TextColor)>,
    selected: Query<Entity, With<Selected>>,
    dragging: Query<Entity, With<Dragging>>,
) {
    for (mut text, mut color) in query.iter_mut() {
        let mode_text = match *mode {
            InteractionMode::None => {
                color.0 = Color::WHITE;
                "Modo: Livre"
            }
            InteractionMode::Dragging => {
                color.0 = Color::srgb(0.3, 0.9, 0.3);
                if let Ok(_) = dragging.get_single() {
                    "Modo: Arrastando..."
                } else {
                    "Modo: Drag & Drop"
                }
            }
            InteractionMode::Transforming => {
                color.0 = Color::srgb(0.9, 0.3, 0.9);
                "Modo: Criando Geometria (clique para confirmar)"
            }
            InteractionMode::Selecting => {
                color.0 = Color::srgb(0.9, 0.9, 0.3);
                if let Ok(_) = selected.get_single() {
                    "Modo: Geometria Selecionada"
                } else {
                    "Modo: Seleção"
                }
            }
        };
        
        text.0 = format!(
            "Interactive Shapes Demo - {}\n\nControles:\n[Mouse] Arrastar/Selecionar\n[T] Criar Retângulo\n[G] Criar Círculo\n[R] Criar Triângulo\n[H] Criar Hexágono\n[Y] Criar Estrela\n[ESC] Cancelar",
            mode_text
        );
    }
}