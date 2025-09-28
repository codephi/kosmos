use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::ui::{PositionType, Val, Node};

mod gpu_animations;
use gpu_animations::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Kosmos - GPU Animations Demo (1000+ objetos!)".to_string(),
                resolution: (1200.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        // Plugins de diagnóstico para ver FPS
        .add_plugins((
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
        ))
        // Plugin de animações GPU
        .add_plugins(GpuAnimationPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, controls_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gpu_materials: ResMut<Assets<AnimatedMaterial>>,
) {
    // Câmera 2D
    commands.spawn(Camera2d::default());
    
    // Informações na tela
    commands.spawn((
        Text::new(
            "🚀 GPU Animation Demo - 1000+ Objetos!\n\n\
            Controles:\n\
            [1] 100 objetos\n\
            [2] 500 objetos\n\
            [3] 1000 objetos\n\
            [4] 2000 objetos\n\
            [5] 5000 objetos (!)\n\
            [C] Limpar tela\n\
            [SPACE] Demo especial\n\n\
            FPS: Check console\n\n\
            🎮 TODAS as animações rodam na GPU!\n\
            Rotação, Pulsação, Órbita, Ondas, Cores..."
        ),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        UiText,
    ));
    
    // === EXEMPLO 1: Objeto único com todas as animações ===
    let showcase_material = AnimatedMaterialBuilder::new()
        .with_rotation_speed(1.0)
        .with_orbit(0.5, 100.0)
        .with_pulse(2.0, 0.3)
        .with_wave(20.0, 0.05)
        .with_colors(
            Color::srgb(0.2, 0.5, 1.0),
            Color::srgb(1.0, 0.2, 0.5),
            1.5
        )
        .with_flags(AnimationFlags::ALL)
        .build();
    
    spawn_gpu_animated_shape(
        &mut commands,
        &mut meshes,
        &mut gpu_materials,
        Mesh::from(RegularPolygon::new(50.0, 6)),
        showcase_material,
        Vec3::ZERO,
    );
    
    // === EXEMPLO 2: Grid de objetos com variações ===
    let grid_size = 10;
    let spacing = 60.0;
    let start_x = -(grid_size as f32 * spacing) / 2.0;
    let start_y = -(grid_size as f32 * spacing) / 2.0;
    
    for i in 0..grid_size {
        for j in 0..grid_size {
            let x = start_x + i as f32 * spacing;
            let y = start_y + j as f32 * spacing;
            
            // Criar material com variações baseadas na posição
            let material = AnimatedMaterialBuilder::new()
                .with_rotation_speed((i as f32 + 1.0) * 0.2)
                .with_pulse((j as f32 + 1.0) * 0.3, 0.2)
                .with_colors(
                    Color::hsl(i as f32 * 36.0, 0.8, 0.5),
                    Color::hsl(j as f32 * 36.0, 0.8, 0.5),
                    1.0
                )
                .with_time_offset(i as f32 * 0.1 + j as f32 * 0.1)
                .with_flags(
                    AnimationFlags::NONE
                        .with_rotation()
                        .with_pulse()
                        .with_color_cycle()
                )
                .build();
            
            // Alternar entre diferentes formas
            let mesh = match (i + j) % 4 {
                0 => Mesh::from(Circle::new(15.0)),
                1 => Mesh::from(Rectangle::new(25.0, 25.0)),
                2 => Mesh::from(RegularPolygon::new(18.0, 5)),
                _ => Mesh::from(Ellipse::new(20.0, 12.0)),
            };
            
            spawn_gpu_animated_shape(
                &mut commands,
                &mut meshes,
                &mut gpu_materials,
                mesh,
                material,
                Vec3::new(x, y, 0.0),
            );
        }
    }
}

/// Sistema de controles para adicionar/remover objetos
fn controls_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gpu_materials: ResMut<Assets<AnimatedMaterial>>,
    query: Query<Entity, With<GpuAnimated>>,
) {
    // Limpar tela
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
        info!("Tela limpa!");
    }
    
    // Adicionar diferentes quantidades de objetos
    let count = if keyboard_input.just_pressed(KeyCode::Digit1) {
        Some(100)
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        Some(500)
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        Some(1000)
    } else if keyboard_input.just_pressed(KeyCode::Digit4) {
        Some(2000)
    } else if keyboard_input.just_pressed(KeyCode::Digit5) {
        Some(5000)
    } else {
        None
    };
    
    if let Some(count) = count {
        // Limpar antes de adicionar novos
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
        
        info!("Criando {} objetos animados na GPU...", count);
        
        // Material base com todas as animações
        let base_material = AnimatedMaterialBuilder::new()
            .with_rotation_speed(1.0)
            .with_orbit(1.0, 30.0)
            .with_pulse(2.0, 0.3)
            .with_wave(10.0, 0.1)
            .with_colors(
                Color::srgb(0.3, 0.5, 1.0),
                Color::srgb(1.0, 0.5, 0.3),
                1.0
            )
            .with_flags(AnimationFlags::ALL)
            .build();
        
        // Criar campo de objetos
        create_animated_field(
            &mut commands,
            &mut meshes,
            &mut gpu_materials,
            count,
            base_material,
            Vec2::new(1100.0, 700.0),
        );
        
        info!("✅ {} objetos criados! Observe o FPS se manter estável!", count);
    }
    
    // Demo especial
    if keyboard_input.just_pressed(KeyCode::Space) {
        // Limpar
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
        
        info!("🌟 Demo especial: Criando vórtex de partículas!");
        
        // Criar vórtex circular
        let count = 500;
        for i in 0..count {
            let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
            let radius = 200.0 + (i as f32 / count as f32) * 200.0;
            
            let material = AnimatedMaterialBuilder::new()
                .with_rotation_speed(2.0 + (i as f32 * 0.01))
                .with_orbit(1.0 - (i as f32 / count as f32), 50.0)
                .with_pulse(3.0, 0.5)
                .with_colors(
                    Color::hsl((i as f32 / count as f32) * 360.0, 1.0, 0.5),
                    Color::hsl(((i as f32 / count as f32) * 360.0 + 180.0) % 360.0, 1.0, 0.5),
                    2.0
                )
                .with_time_offset(i as f32 * 0.01)
                .with_flags(AnimationFlags::ALL)
                .build();
            
            let size = 5.0 + (i as f32 / count as f32) * 10.0;
            let mesh = if i % 2 == 0 {
                Mesh::from(Circle::new(size))
            } else {
                Mesh::from(RegularPolygon::new(size, (3 + i % 5) as u32))
            };
            
            spawn_gpu_animated_shape(
                &mut commands,
                &mut meshes,
                &mut gpu_materials,
                mesh,
                material,
                Vec3::new(angle.cos() * radius, angle.sin() * radius, 0.0),
            );
        }
    }
}

#[derive(Component)]
struct UiText;