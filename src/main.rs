use bevy::prelude::*;

mod geometrics;
use geometrics::{Geometrics, GeometricsExt};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Kosmo 2D - Geometrics Demo".to_string(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Adicionar câmera 2D
    commands.spawn(Camera2d::default());

    // Usando o módulo Geometrics para criar formas

    // Círculo azul no centro
    Geometrics::circle(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::srgb(0.3, 0.5, 0.8),
        50.0,
        Vec2::new(0.0, 0.0),
    );

    // Quadrado vermelho à esquerda
    Geometrics::square(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::srgb(0.8, 0.3, 0.3),
        60.0,
        Vec2::new(-200.0, 0.0),
    );

    // Triângulo verde à direita
    Geometrics::triangle(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::srgb(0.3, 0.8, 0.3),
        70.0,
        Vec2::new(200.0, 0.0),
    );

    // Pentágono amarelo acima
    Geometrics::polygon(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::srgb(0.8, 0.8, 0.3),
        5,
        40.0,
        Vec2::new(0.0, 150.0),
    );

    // Elipse roxa abaixo
    Geometrics::ellipse(
        &mut commands,
        &mut meshes,
        &mut materials,
        Color::srgb(0.6, 0.3, 0.8),
        100.0,
        50.0,
        Vec2::new(0.0, -150.0),
    );

    // Exemplo alternativo usando a extensão trait (mais conciso)
    commands.spawn_circle(
        &mut meshes,
        &mut materials,
        Color::srgb(1.0, 0.5, 0.0), // Laranja
        30.0,
        Vec2::new(-150.0, 150.0),
    );

    commands.spawn_square(
        &mut meshes,
        &mut materials,
        Color::srgb(0.0, 0.5, 1.0), // Azul claro
        30.0,
        Vec2::new(150.0, 150.0),
    );

    commands.spawn_triangle(
        &mut meshes,
        &mut materials,
        Color::srgb(1.0, 0.0, 0.5), // Rosa
        40.0,
        Vec2::new(-150.0, -150.0),
    );

    // Exemplos usando o novo método draw

    // Desenhar uma estrela amarela preenchida
    Geometrics::draw_star(
        &mut commands,
        &mut meshes,
        &mut materials,
        5,                          // 5 pontas
        50.0,                       // raio externo
        20.0,                       // raio interno
        Color::srgb(1.0, 1.0, 0.0), // Amarelo
        Vec2::new(250.0, 100.0),
        true, // preenchida
    );

    // Desenhar um coração vermelho
    Geometrics::draw_heart(
        &mut commands,
        &mut meshes,
        &mut materials,
        40.0,
        Color::srgb(1.0, 0.2, 0.3),
        Vec2::new(-250.0, 100.0),
        true,
    );

    // Desenhar um hexágono customizado usando coordenadas
    let hexagon_coords = vec![
        Vec2::new(30.0, 0.0),
        Vec2::new(15.0, 26.0),
        Vec2::new(-15.0, 26.0),
        Vec2::new(-30.0, 0.0),
        Vec2::new(-15.0, -26.0),
        Vec2::new(15.0, -26.0),
    ];

    Geometrics::draw(
        &mut commands,
        &mut meshes,
        &mut materials,
        hexagon_coords,
        Color::srgb(0.5, 1.0, 0.5), // Verde claro
        Vec2::new(250.0, -100.0),
        1.5,   // escala 1.5x
        false, // apenas contorno
    );

    // Desenhar uma seta customizada
    let arrow_coords = vec![
        Vec2::new(0.0, 40.0),    // ponta
        Vec2::new(20.0, 10.0),   // lado direito superior
        Vec2::new(10.0, 10.0),   // entrada direita
        Vec2::new(10.0, -40.0),  // base direita
        Vec2::new(-10.0, -40.0), // base esquerda
        Vec2::new(-10.0, 10.0),  // entrada esquerda
        Vec2::new(-20.0, 10.0),  // lado esquerdo superior
    ];

    commands.draw(
        &mut meshes,
        &mut materials,
        arrow_coords,
        Color::srgb(0.8, 0.4, 1.0), // Roxo
        Vec2::new(-250.0, -100.0),
        1.0,
        true, // preenchida
    );
}
