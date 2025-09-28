use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

mod geometrics;
use geometrics::{Geometrics, GeometricsExt};

mod animations;
use animations::{
    AnimatableProperty, AnimationBuilder, AnimationComponent, AnimationPresets, Easing,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Kosmo 2D - Geometrics & Animations Demo".to_string(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(animations::AnimationPlugin) // Adicionar o plugin de animações
        .add_systems(Startup, setup)
        .add_systems(Update, start_animations)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Adicionar câmera 2D
    commands.spawn(Camera2d::default());

    // === EXEMPLOS DE ANIMAÇÕES ===

    // Círculo azul com animação de bounce (salto)
    let bounce_animation = AnimationBuilder::new("circle_bounce")
        .move_to(Vec2::new(0.0, 100.0), 0.5, Easing::EaseOutQuad)
        .move_to(Vec2::new(0.0, 0.0), 0.5, Easing::EaseInQuad)
        .repeat()
        .build_and_play();

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.3, 0.5, 0.8)))),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        AnimationComponent::new(bounce_animation),
        CircleTag, // Tag para identificar o círculo
    ));

    // Quadrado vermelho com animação de rotação
    let spin_animation = AnimationBuilder::new("square_spin")
        .rotate_to(std::f32::consts::TAU, 2.0, Easing::Linear)
        .repeat()
        .build_and_play();

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(60.0, 60.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.8, 0.3, 0.3)))),
        Transform::from_translation(Vec3::new(-200.0, 0.0, 0.0)),
        AnimationComponent::new(spin_animation),
        SquareTag,
    ));

    // Triângulo verde com animação de escala (pulsação)
    let pulse_animation = AnimationBuilder::new("triangle_pulse")
        .scale_to(1.5, 1.0, Easing::EaseInOut)
        .scale_to(1.0, 1.0, Easing::EaseInOut)
        .repeat()
        .build_and_play();

    let height = 70.0 * (3.0_f32).sqrt() / 2.0;
    let half_base = 70.0 / 2.0;

    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::new(0.0, height / 2.0),
            Vec2::new(-half_base, -height / 2.0),
            Vec2::new(half_base, -height / 2.0),
        ))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.3, 0.8, 0.3)))),
        Transform::from_translation(Vec3::new(200.0, 0.0, 0.0)),
        AnimationComponent::new(pulse_animation),
        TriangleTag,
    ));

    // Pentágono amarelo com animação de cor
    let color_animation = AnimationBuilder::new("pentagon_color")
        .color_to(Color::srgb(0.8, 0.8, 0.3), 0.0, Easing::Linear) // Cor inicial
        .color_to(Color::srgb(1.0, 0.5, 0.0), 1.0, Easing::EaseInOut) // Laranja
        .color_to(Color::srgb(1.0, 0.2, 0.5), 1.0, Easing::EaseInOut) // Rosa
        .color_to(Color::srgb(0.5, 0.2, 1.0), 1.0, Easing::EaseInOut) // Roxo
        .color_to(Color::srgb(0.8, 0.8, 0.3), 1.0, Easing::EaseInOut) // Volta ao amarelo
        .repeat()
        .build_and_play();

    let material_handle = materials.add(ColorMaterial::from(Color::srgb(0.8, 0.8, 0.3)));

    commands.spawn((
        Mesh2d(meshes.add(RegularPolygon::new(40.0, 5))),
        MeshMaterial2d(material_handle),
        Transform::from_translation(Vec3::new(0.0, 150.0, 0.0)),
        AnimationComponent::new(color_animation),
        PentagonTag,
    ));

    // Elipse roxa com animação complexa (movimento circular + escala)
    let complex_animation = AnimationBuilder::new("ellipse_complex")
        // Movimento circular
        .move_to(Vec2::new(100.0, -150.0), 1.0, Easing::EaseInOut)
        .move_to(Vec2::new(0.0, -50.0), 1.0, Easing::EaseInOut)
        .move_to(Vec2::new(-100.0, -150.0), 1.0, Easing::EaseInOut)
        .move_to(Vec2::new(0.0, -150.0), 1.0, Easing::EaseInOut)
        // Ao mesmo tempo, adicionar escala
        .add_keyframe(
            "scale",
            0.0,
            AnimatableProperty::Scale(Vec2::ONE),
            Easing::Linear,
        )
        .add_keyframe(
            "scale",
            2.0,
            AnimatableProperty::Scale(Vec2::new(1.5, 0.8)),
            Easing::EaseInOut,
        )
        .add_keyframe(
            "scale",
            4.0,
            AnimatableProperty::Scale(Vec2::ONE),
            Easing::EaseInOut,
        )
        .repeat()
        .build_and_play();

    commands.spawn((
        Mesh2d(meshes.add(Ellipse::new(50.0, 25.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.6, 0.3, 0.8)))),
        Transform::from_translation(Vec3::new(0.0, -150.0, 0.0)),
        AnimationComponent::new(complex_animation),
        EllipseTag,
    ));

    // Círculo laranja com fade in/out
    let fade_animation = AnimationBuilder::new("circle_fade")
        .fade_to(1.0, 0.0, Easing::Linear) // Começa visível
        .fade_to(0.1, 1.5, Easing::EaseInOut)
        .fade_to(1.0, 1.5, Easing::EaseInOut)
        .repeat()
        .build_and_play();

    let material = materials.add(ColorMaterial::from(Color::srgba(1.0, 0.5, 0.0, 1.0)));

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(30.0))),
        MeshMaterial2d(material),
        Transform::from_translation(Vec3::new(-150.0, 150.0, 0.0)),
        AnimationComponent::new(fade_animation),
    ));

    // Quadrado azul claro com animação ping-pong
    let pingpong_animation = AnimationBuilder::new("square_pingpong")
        .move_to(Vec2::new(200.0, 150.0), 2.0, Easing::EaseInOut)
        .rotate_to(std::f32::consts::FRAC_PI_2, 2.0, Easing::EaseInOut)
        .ping_pong()
        .build_and_play();

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(30.0, 30.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.0, 0.5, 1.0)))),
        Transform::from_translation(Vec3::new(150.0, 150.0, 0.0)),
        AnimationComponent::new(pingpong_animation),
    ));

    // Triângulo rosa com sequência de animações
    let sequence_animation = AnimationBuilder::new("triangle_sequence")
        .wait(0.5) // Aguarda meio segundo
        .scale_to(1.5, 0.5, Easing::EaseOut) // Cresce
        .wait(0.2) // Pequena pausa
        .scale_to(0.5, 0.5, Easing::EaseIn) // Encolhe
        .scale_to(1.0, 0.5, Easing::EaseOut) // Volta ao normal
        .rotate_to(std::f32::consts::TAU, 1.0, Easing::EaseInOut) // Gira 360°
        .repeat_times(3) // Repete 3 vezes
        .build_and_play();

    let height = 40.0 * (3.0_f32).sqrt() / 2.0;
    let half_base = 40.0 / 2.0;

    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::new(0.0, height / 2.0),
            Vec2::new(-half_base, -height / 2.0),
            Vec2::new(half_base, -height / 2.0),
        ))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(1.0, 0.0, 0.5)))),
        Transform::from_translation(Vec3::new(-150.0, -150.0, 0.0)),
        AnimationComponent::new(sequence_animation),
    ));

    // Estrela amarela com animação usando preset
    let star_entity = Geometrics::draw_star(
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

    // Adicionar animação de rotação contínua à estrela
    commands
        .entity(star_entity)
        .insert(AnimationComponent::new(AnimationPresets::spin(0.5)));

    // Coração vermelho com animação de pulsação
    let heart_entity = Geometrics::draw_heart(
        &mut commands,
        &mut meshes,
        &mut materials,
        40.0,
        Color::srgb(1.0, 0.2, 0.3),
        Vec2::new(-250.0, 100.0),
        true,
    );

    // Adicionar animação de pulsação ao coração
    commands.entity(heart_entity).insert(
        AnimationComponent::new(AnimationPresets::pulse(1.3, 1.0))
    );
    
    // === EXEMPLO ESPECIAL: ANIMAÇÃO MÚLTIPLA SIMULTÂNEA ===
    // Anima forma (escala), cor, rotação e posição ao mesmo tempo
    
    info!("\n=== ANIMAÇÃO MÚLTIPLA SIMULTÂNEA ===");
    info!("Observe o hexágono no centro inferior:");
    info!("- Muda de cor: Azul → Verde → Vermelho → Amarelo → Roxo → Azul");
    info!("- Gira continuamente");
    info!("- Pulsa (muda de tamanho)");
    info!("- Move em padrão de 8");
    info!("Tudo acontecendo ao mesmo tempo!\n");
    
    // Criar uma animação complexa com múltiplas propriedades simultâneas
    let multi_animation = AnimationBuilder::new("multi_animation")
        // === MOVIMENTO EM FORMA DE 8 ===
        // Primeira metade do 8
        .move_to(Vec2::new(0.0, -250.0), 0.0, Easing::Linear)  // Posição inicial
        .move_to(Vec2::new(50.0, -200.0), 0.5, Easing::EaseInOut)
        .move_to(Vec2::new(100.0, -250.0), 0.5, Easing::EaseInOut)
        .move_to(Vec2::new(50.0, -300.0), 0.5, Easing::EaseInOut)
        .move_to(Vec2::new(0.0, -250.0), 0.5, Easing::EaseInOut)
        // Segunda metade do 8
        .move_to(Vec2::new(-50.0, -200.0), 0.5, Easing::EaseInOut)
        .move_to(Vec2::new(-100.0, -250.0), 0.5, Easing::EaseInOut)
        .move_to(Vec2::new(-50.0, -300.0), 0.5, Easing::EaseInOut)
        .move_to(Vec2::new(0.0, -250.0), 0.5, Easing::EaseInOut)
        
        // === ROTAÇÃO CONTÍNUA ===
        // Adiciona rotação que acontece durante todo o movimento
        .add_keyframe("rotation", 0.0, AnimatableProperty::Rotation(0.0), Easing::Linear)
        .add_keyframe("rotation", 1.0, AnimatableProperty::Rotation(std::f32::consts::PI), Easing::Linear)
        .add_keyframe("rotation", 2.0, AnimatableProperty::Rotation(std::f32::consts::TAU), Easing::Linear)
        .add_keyframe("rotation", 3.0, AnimatableProperty::Rotation(std::f32::consts::PI * 3.0), Easing::Linear)
        .add_keyframe("rotation", 4.0, AnimatableProperty::Rotation(std::f32::consts::TAU * 2.0), Easing::Linear)
        
        // === MUDANÇA DE COR GRADUAL ===
        // Transições de cor suaves ao longo do tempo
        .add_keyframe("color", 0.0, AnimatableProperty::Color(Color::srgb(0.2, 0.5, 1.0)), Easing::Linear)  // Azul
        .add_keyframe("color", 0.8, AnimatableProperty::Color(Color::srgb(0.2, 1.0, 0.3)), Easing::EaseInOut)  // Verde
        .add_keyframe("color", 1.6, AnimatableProperty::Color(Color::srgb(1.0, 0.2, 0.2)), Easing::EaseInOut)  // Vermelho
        .add_keyframe("color", 2.4, AnimatableProperty::Color(Color::srgb(1.0, 1.0, 0.2)), Easing::EaseInOut)  // Amarelo
        .add_keyframe("color", 3.2, AnimatableProperty::Color(Color::srgb(0.8, 0.2, 1.0)), Easing::EaseInOut)  // Roxo
        .add_keyframe("color", 4.0, AnimatableProperty::Color(Color::srgb(0.2, 0.5, 1.0)), Easing::EaseInOut)  // Volta ao azul
        
        // === PULSAÇÃO (ESCALA) ===
        // Mudança de tamanho rítmica
        .add_keyframe("scale", 0.0, AnimatableProperty::Scale(Vec2::splat(1.0)), Easing::Linear)
        .add_keyframe("scale", 0.4, AnimatableProperty::Scale(Vec2::splat(1.3)), Easing::EaseOut)
        .add_keyframe("scale", 0.8, AnimatableProperty::Scale(Vec2::splat(0.8)), Easing::EaseIn)
        .add_keyframe("scale", 1.2, AnimatableProperty::Scale(Vec2::splat(1.5)), Easing::EaseOut)
        .add_keyframe("scale", 1.6, AnimatableProperty::Scale(Vec2::splat(1.0)), Easing::EaseInOut)
        .add_keyframe("scale", 2.0, AnimatableProperty::Scale(Vec2::new(1.8, 0.6)), Easing::EaseOut)  // Estica horizontalmente
        .add_keyframe("scale", 2.4, AnimatableProperty::Scale(Vec2::new(0.6, 1.8)), Easing::EaseInOut)  // Estica verticalmente
        .add_keyframe("scale", 2.8, AnimatableProperty::Scale(Vec2::splat(1.2)), Easing::EaseIn)
        .add_keyframe("scale", 3.2, AnimatableProperty::Scale(Vec2::splat(0.9)), Easing::EaseOut)
        .add_keyframe("scale", 3.6, AnimatableProperty::Scale(Vec2::splat(1.4)), Easing::EaseInOut)
        .add_keyframe("scale", 4.0, AnimatableProperty::Scale(Vec2::splat(1.0)), Easing::EaseIn)
        
        .repeat()  // Loop infinito
        .build_and_play();
    
    // Criar um hexágono especial para demonstrar a animação múltipla
    let multi_material = materials.add(ColorMaterial::from(Color::srgb(0.2, 0.5, 1.0)));
    
    commands.spawn((
        Mesh2d(meshes.add(RegularPolygon::new(35.0, 6))),  // Hexágono
        MeshMaterial2d(multi_material),
        Transform::from_translation(Vec3::new(0.0, -250.0, 0.0)),
        AnimationComponent::new(multi_animation),
        MultiAnimationTag,  // Tag especial para este exemplo
    ));
    
    // === SEGUNDO EXEMPLO: ESTRELA COM ANIMAÇÃO COMPLEXA ===
    // Uma estrela que faz uma dança elaborada
    
    let star_dance = AnimationBuilder::new("star_dance")
        // Movimento em espiral
        .move_to(Vec2::new(300.0, 0.0), 0.0, Easing::Linear)
        .move_to(Vec2::new(280.0, 50.0), 0.25, Easing::EaseIn)
        .move_to(Vec2::new(240.0, 80.0), 0.25, Easing::Linear)
        .move_to(Vec2::new(200.0, 90.0), 0.25, Easing::Linear)
        .move_to(Vec2::new(160.0, 80.0), 0.25, Easing::Linear)
        .move_to(Vec2::new(140.0, 50.0), 0.25, Easing::Linear)
        .move_to(Vec2::new(140.0, 0.0), 0.25, Easing::Linear)
        .move_to(Vec2::new(160.0, -50.0), 0.25, Easing::Linear)
        .move_to(Vec2::new(200.0, -80.0), 0.25, Easing::Linear)
        .move_to(Vec2::new(240.0, -90.0), 0.25, Easing::Linear)
        .move_to(Vec2::new(280.0, -80.0), 0.25, Easing::Linear)
        .move_to(Vec2::new(300.0, -50.0), 0.25, Easing::Linear)
        .move_to(Vec2::new(300.0, 0.0), 0.25, Easing::EaseOut)
        
        // Rotação acelerando e desacelerando
        .add_keyframe("rotation", 0.0, AnimatableProperty::Rotation(0.0), Easing::Linear)
        .add_keyframe("rotation", 0.5, AnimatableProperty::Rotation(std::f32::consts::FRAC_PI_2), Easing::EaseIn)
        .add_keyframe("rotation", 1.0, AnimatableProperty::Rotation(std::f32::consts::PI * 2.0), Easing::EaseOut)
        .add_keyframe("rotation", 1.5, AnimatableProperty::Rotation(std::f32::consts::PI * 3.0), Easing::EaseIn)
        .add_keyframe("rotation", 2.0, AnimatableProperty::Rotation(std::f32::consts::PI * 5.0), Easing::EaseOut)
        .add_keyframe("rotation", 2.5, AnimatableProperty::Rotation(std::f32::consts::PI * 6.0), Easing::EaseInOut)
        .add_keyframe("rotation", 3.0, AnimatableProperty::Rotation(std::f32::consts::TAU * 4.0), Easing::Linear)
        
        // Mudança de cor tipo arco-íris
        .add_keyframe("color", 0.0, AnimatableProperty::Color(Color::srgb(1.0, 0.0, 0.0)), Easing::Linear)  // Vermelho
        .add_keyframe("color", 0.5, AnimatableProperty::Color(Color::srgb(1.0, 0.5, 0.0)), Easing::Linear)  // Laranja
        .add_keyframe("color", 1.0, AnimatableProperty::Color(Color::srgb(1.0, 1.0, 0.0)), Easing::Linear)  // Amarelo
        .add_keyframe("color", 1.5, AnimatableProperty::Color(Color::srgb(0.0, 1.0, 0.0)), Easing::Linear)  // Verde
        .add_keyframe("color", 2.0, AnimatableProperty::Color(Color::srgb(0.0, 1.0, 1.0)), Easing::Linear)  // Ciano
        .add_keyframe("color", 2.5, AnimatableProperty::Color(Color::srgb(0.0, 0.0, 1.0)), Easing::Linear)  // Azul
        .add_keyframe("color", 3.0, AnimatableProperty::Color(Color::srgb(1.0, 0.0, 1.0)), Easing::Linear)  // Magenta
        
        // Escala pulsando com ritmo diferente
        .add_keyframe("scale", 0.0, AnimatableProperty::Scale(Vec2::splat(0.5)), Easing::Linear)
        .add_keyframe("scale", 0.3, AnimatableProperty::Scale(Vec2::splat(1.0)), Easing::EaseOut)
        .add_keyframe("scale", 0.6, AnimatableProperty::Scale(Vec2::splat(0.7)), Easing::EaseIn)
        .add_keyframe("scale", 0.9, AnimatableProperty::Scale(Vec2::splat(1.2)), Easing::EaseOut)
        .add_keyframe("scale", 1.2, AnimatableProperty::Scale(Vec2::splat(0.8)), Easing::Linear)
        .add_keyframe("scale", 1.5, AnimatableProperty::Scale(Vec2::splat(1.5)), Easing::EaseOut)
        .add_keyframe("scale", 1.8, AnimatableProperty::Scale(Vec2::splat(1.0)), Easing::EaseIn)
        .add_keyframe("scale", 2.1, AnimatableProperty::Scale(Vec2::splat(1.3)), Easing::EaseInOut)
        .add_keyframe("scale", 2.4, AnimatableProperty::Scale(Vec2::splat(0.6)), Easing::EaseIn)
        .add_keyframe("scale", 2.7, AnimatableProperty::Scale(Vec2::splat(1.1)), Easing::EaseOut)
        .add_keyframe("scale", 3.0, AnimatableProperty::Scale(Vec2::splat(0.5)), Easing::EaseInOut)
        
        .repeat()  // Loop infinito
        .build_and_play();
    
    // Criar estrela com múltiplas animações
    let star_coords = create_star_points(8, 25.0, 12.0);  // Estrela de 8 pontas
    let star_material = materials.add(ColorMaterial::from(Color::srgb(1.0, 0.0, 0.0)));
    
    let star_mesh = create_custom_mesh(star_coords);
    
    commands.spawn((
        Mesh2d(meshes.add(star_mesh)),
        MeshMaterial2d(star_material),
        Transform::from_translation(Vec3::new(300.0, 0.0, 0.0)),
        AnimationComponent::new(star_dance),
        StarDanceTag,
    ));

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

// Tags para identificar diferentes entidades
#[derive(Component)]
struct CircleTag;

#[derive(Component)]
struct SquareTag;

#[derive(Component)]
struct TriangleTag;

#[derive(Component)]
struct PentagonTag;

#[derive(Component)]
struct EllipseTag;

#[derive(Component)]
struct MultiAnimationTag;

#[derive(Component)]
struct StarDanceTag;

// Sistema para iniciar animações com delay via teclado (exemplo)
fn start_animations(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut AnimationComponent>,
) {
    // Pressionar SPACE pausa/retoma todas as animações
    if keyboard_input.just_pressed(KeyCode::Space) {
        for mut animation in query.iter_mut() {
            if animation.timeline.is_playing() {
                animation.timeline.pause();
            } else {
                animation.timeline.play();
            }
        }
    }

    // Pressionar R reinicia todas as animações
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        for mut animation in query.iter_mut() {
            animation.timeline.restart();
        }
    }

    // Pressionar S para todas as animações
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        for mut animation in query.iter_mut() {
            animation.timeline.stop();
        }
    }

    // Pressionar 1-9 para alterar velocidade
    for (key, speed) in [
        (KeyCode::Digit1, 0.5),
        (KeyCode::Digit2, 1.0),
        (KeyCode::Digit3, 1.5),
        (KeyCode::Digit4, 2.0),
        (KeyCode::Digit5, 3.0),
    ] {
        if keyboard_input.just_pressed(key) {
            for mut animation in query.iter_mut() {
                animation.timeline.set_speed(speed);
            }
            println!("Velocidade das animações: {}x", speed);
        }
    }
}

// Função auxiliar para criar pontos de estrela
fn create_star_points(points: usize, outer_radius: f32, inner_radius: f32) -> Vec<Vec2> {
    let mut coords = Vec::new();
    let angle_step = std::f32::consts::TAU / (points * 2) as f32;
    
    for i in 0..points * 2 {
        let angle = angle_step * i as f32 - std::f32::consts::FRAC_PI_2;
        let radius = if i % 2 == 0 { outer_radius } else { inner_radius };
        coords.push(Vec2::new(angle.cos() * radius, angle.sin() * radius));
    }
    
    coords
}

// Função auxiliar para criar mesh customizada
fn create_custom_mesh(coordinates: Vec<Vec2>) -> Mesh {
    let vertices: Vec<[f32; 3]> = coordinates
        .iter()
        .map(|v| [v.x, v.y, 0.0])
        .collect();
    
    // Criar índices para triangulação (fan triangulation)
    let mut indices = Vec::new();
    for i in 1..coordinates.len() - 1 {
        indices.push(0);
        indices.push(i as u32);
        indices.push((i + 1) as u32);
    }
    
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_indices(Indices::U32(indices));
    
    // Adicionar normais e UVs
    let normals = vec![[0.0, 0.0, 1.0]; coordinates.len()];
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    
    let uvs: Vec<[f32; 2]> = coordinates
        .iter()
        .map(|v| [(v.x + 1.0) * 0.5, (v.y + 1.0) * 0.5])
        .collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    
    mesh
}
