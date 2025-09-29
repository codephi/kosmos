pub use bevy;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
pub mod animations;
pub mod geometrics;
use animations::{AnimatableProperty, AnimationBuilder, AnimationComponent, Easing};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Shape Morphing Animation".to_string(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(animations::AnimationPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (morph_shapes, control_animation))
        .run();
}

// Componente para o morphing com interpolação de vértices
#[derive(Component)]
struct MorphingShape {
    shapes: Vec<Vec<Vec2>>,      // Todas as formas com mesmo número de vértices
    current_vertices: Vec<Vec2>, // Vértices atuais (interpolados)
    shape_index: usize,
    next_shape_index: usize,
    morph_timer: Timer,
    transition_timer: Timer,
    colors: Vec<Color>, // Cores para cada forma
    current_color: Color,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Adicionar câmera 2D
    commands.spawn(Camera2d::default());

    // === ANIMAÇÃO DE MORPHING DE FORMAS ===

    // Número de pontos para interpolação (deve ser divisível por várias formas)
    let num_points = 60;

    // Criar as formas com o mesmo número de vértices na ordem: quadrado -> triângulo -> círculo -> estrela -> nuvem
    let square_vertices = create_square_vertices(num_points, 90.0);
    let triangle_vertices = create_triangle_vertices(num_points, 100.0);
    let circle_vertices = create_circle_vertices(num_points, 80.0);
    let star_vertices = create_star_vertices(num_points, 80.0, 35.0);
    let flower_vertices = create_flower_vertices(num_points, 80.0);

    let shapes = vec![
        square_vertices.clone(), // Começa com quadrado
        flower_vertices,
        triangle_vertices,
        circle_vertices,
        star_vertices,
    ];

    // Cores para cada forma
    let colors = vec![
        Color::srgb(0.8, 0.3, 0.3), // Vermelho para quadrado
        Color::srgb(0.2, 1.0, 0.3), // Verde para triângulo
        Color::srgb(0.2, 0.5, 1.0), // Azul para círculo
        Color::srgb(1.0, 1.0, 0.0), // Amarelo para estrela
        Color::srgb(1.0, 0.4, 0.7), // Rosa para flor
    ];

    // Começa com a mesh do quadrado
    let initial_mesh = create_morphing_mesh(square_vertices.clone());
    let mesh_handle = meshes.add(initial_mesh);
    let initial_material = materials.add(ColorMaterial::from(colors[0]));

    // Cria animação complexa com movimento, rotação e escala
    let morph_animation = AnimationBuilder::new("shape_morph")
        // Movimento circular com translação
        .move_to(Vec2::new(0.0, 0.0), 0.0, Easing::Linear) // Posição inicial
        .move_to(Vec2::new(150.0, 100.0), 3.0, Easing::EaseInOut) // Move para direita-cima
        .move_to(Vec2::new(0.0, 200.0), 3.0, Easing::EaseInOut) // Move para cima
        .move_to(Vec2::new(-150.0, 100.0), 3.0, Easing::EaseInOut) // Move para esquerda-cima
        .move_to(Vec2::new(-150.0, -100.0), 3.0, Easing::EaseInOut) // Move para esquerda-baixo
        .move_to(Vec2::new(0.0, -200.0), 3.0, Easing::EaseInOut) // Move para baixo
        .move_to(Vec2::new(150.0, -100.0), 3.0, Easing::EaseInOut) // Move para direita-baixo
        .move_to(Vec2::new(0.0, 0.0), 3.0, Easing::EaseInOut) // Volta ao centro
        // Rotação contínua
        .add_keyframe(
            "rotation",
            0.0,
            AnimatableProperty::Rotation(0.0),
            Easing::Linear,
        )
        .add_keyframe(
            "rotation",
            6.0,
            AnimatableProperty::Rotation(std::f32::consts::PI),
            Easing::EaseInOut,
        )
        .add_keyframe(
            "rotation",
            12.0,
            AnimatableProperty::Rotation(std::f32::consts::TAU),
            Easing::EaseInOut,
        )
        .add_keyframe(
            "rotation",
            18.0,
            AnimatableProperty::Rotation(std::f32::consts::PI * 3.0),
            Easing::EaseInOut,
        )
        .add_keyframe(
            "rotation",
            24.0,
            AnimatableProperty::Rotation(std::f32::consts::TAU * 2.0),
            Easing::Linear,
        )
        // Escala pulsa suavemente
        .add_keyframe(
            "scale",
            0.0,
            AnimatableProperty::Scale(Vec2::splat(1.0)),
            Easing::Linear,
        )
        .add_keyframe(
            "scale",
            3.0,
            AnimatableProperty::Scale(Vec2::splat(1.2)),
            Easing::EaseOut,
        )
        .add_keyframe(
            "scale",
            6.0,
            AnimatableProperty::Scale(Vec2::splat(0.9)),
            Easing::EaseIn,
        )
        .add_keyframe(
            "scale",
            9.0,
            AnimatableProperty::Scale(Vec2::splat(1.3)),
            Easing::EaseOut,
        )
        .add_keyframe(
            "scale",
            12.0,
            AnimatableProperty::Scale(Vec2::splat(1.0)),
            Easing::EaseInOut,
        )
        .add_keyframe(
            "scale",
            15.0,
            AnimatableProperty::Scale(Vec2::splat(1.1)),
            Easing::EaseOut,
        )
        .add_keyframe(
            "scale",
            18.0,
            AnimatableProperty::Scale(Vec2::splat(0.95)),
            Easing::EaseIn,
        )
        .add_keyframe(
            "scale",
            21.0,
            AnimatableProperty::Scale(Vec2::splat(1.25)),
            Easing::EaseOut,
        )
        .add_keyframe(
            "scale",
            24.0,
            AnimatableProperty::Scale(Vec2::splat(1.0)),
            Easing::EaseIn,
        )
        .repeat() // Loop infinito
        .build_and_play();

    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(initial_material),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        AnimationComponent::new(morph_animation),
        MorphingShape {
            shapes,
            current_vertices: square_vertices, // Começa com quadrado
            shape_index: 0,
            next_shape_index: 1,
            morph_timer: Timer::from_seconds(5.0, TimerMode::Repeating), // Tempo até próxima transição
            transition_timer: Timer::from_seconds(1.5, TimerMode::Once), // Duração da transição
            colors,
            current_color: Color::srgb(0.8, 0.3, 0.3), // Vermelho inicial do quadrado
        },
    ));

    // Instruções na tela
    println!("\n=== ANIMAÇÃO DE TRANSFORMAÇÃO DE FORMAS ===");
    println!("A forma se transforma em:");
    println!("1. Quadrado (vermelho)");
    println!("2. Triângulo (verde)");
    println!("3. Círculo (azul)");
    println!("4. Estrela (amarelo)");
    println!("5. Flor (rosa)");
    println!("\nEstrela estática (dourada) à direita - exemplo Bevy");
    println!("\nControles:");
    println!("ESPAÇO - Pausar/Retomar");
    println!("R - Reiniciar animação");
    println!("1-5 - Alterar velocidade\n");
}

// Sistema para transformar as formas com interpolação suave
fn morph_shapes(
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&Mesh2d, &MeshMaterial2d<ColorMaterial>, &mut MorphingShape)>,
) {
    for (mesh_handle, material_handle, mut morph) in query.iter_mut() {
        let mut needs_update = false;

        // Atualizar timer principal
        morph.morph_timer.tick(time.delta());

        // Verificar se deve iniciar nova transição
        if morph.morph_timer.just_finished() {
            // Iniciar transição para próxima forma
            morph.shape_index = morph.next_shape_index;
            morph.next_shape_index = (morph.shape_index + 1) % morph.shapes.len();
            morph.transition_timer = Timer::from_seconds(1.5, TimerMode::Once);
            morph.transition_timer.reset();
        }

        // Processar transição
        if !morph.transition_timer.finished() {
            morph.transition_timer.tick(time.delta());
            let t = morph.transition_timer.fraction();
            let eased_t = ease_in_out_cubic(t);

            // Interpolar vértices
            let from_vertices = morph.shapes[morph.shape_index].clone();
            let to_vertices = morph.shapes[morph.next_shape_index].clone();

            for i in 0..morph.current_vertices.len() {
                let from = from_vertices[i];
                let to = to_vertices[i];
                morph.current_vertices[i] = from.lerp(to, eased_t);
            }

            // Interpolar cores
            let from_color = morph.colors[morph.shape_index];
            let to_color = morph.colors[morph.next_shape_index];
            morph.current_color = interpolate_color(from_color, to_color, eased_t);

            needs_update = true;
        }

        // Atualizar mesh se necessário
        if needs_update {
            if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
                update_mesh_vertices(mesh, &morph.current_vertices);
            }

            // Atualizar cor do material
            if let Some(material) = materials.get_mut(&material_handle.0) {
                material.color = morph.current_color;
            }
        }
    }
}

// Sistema de controle da animação
fn control_animation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut AnimationComponent>,
) {
    // Pressionar SPACE pausa/retoma a animação
    if keyboard_input.just_pressed(KeyCode::Space) {
        for mut animation in query.iter_mut() {
            if animation.timeline.is_playing() {
                animation.timeline.pause();
                println!("Animação pausada");
            } else {
                animation.timeline.play();
                println!("Animação retomada");
            }
        }
    }

    // Pressionar R reinicia a animação
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        for mut animation in query.iter_mut() {
            animation.timeline.restart();
            println!("Animação reiniciada");
        }
    }

    // Pressionar 1-5 para alterar velocidade
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
            println!("Velocidade da animação: {}x", speed);
        }
    }
}

// Funções auxiliares para criação de formas com mesmo número de vértices

fn create_square_vertices(num_points: usize, size: f32) -> Vec<Vec2> {
    let mut vertices = Vec::new();
    let half_size = size / 2.0;

    // Quatro pontos do quadrado
    let square_points = vec![
        Vec2::new(-half_size, half_size),  // Superior esquerdo
        Vec2::new(half_size, half_size),   // Superior direito
        Vec2::new(half_size, -half_size),  // Inferior direito
        Vec2::new(-half_size, -half_size), // Inferior esquerdo
    ];

    // Distribuir pontos ao longo das arestas do quadrado
    let points_per_edge = num_points / 4;

    for edge in 0..4 {
        let start = square_points[edge];
        let end = square_points[(edge + 1) % 4];

        for i in 0..points_per_edge {
            let t = i as f32 / points_per_edge as f32;
            vertices.push(start.lerp(end, t));
        }
    }

    // Adicionar pontos restantes se necessário
    while vertices.len() < num_points {
        vertices.push(vertices[vertices.len() % 4]);
    }

    vertices
}

fn create_circle_vertices(num_points: usize, radius: f32) -> Vec<Vec2> {
    let mut vertices = Vec::new();
    for i in 0..num_points {
        let angle = (i as f32 / num_points as f32) * std::f32::consts::TAU;
        vertices.push(Vec2::new(angle.cos() * radius, angle.sin() * radius));
    }
    vertices
}

fn create_triangle_vertices(num_points: usize, size: f32) -> Vec<Vec2> {
    let mut vertices = Vec::new();
    let height = size * (3.0_f32).sqrt() / 2.0;

    // Três pontos do triângulo
    let triangle_points = vec![
        Vec2::new(0.0, height * 0.6),          // Topo
        Vec2::new(-size / 2.0, -height * 0.4), // Esquerda
        Vec2::new(size / 2.0, -height * 0.4),  // Direita
    ];

    // Distribuir pontos ao longo das arestas do triângulo
    let points_per_edge = num_points / 3;

    for edge in 0..3 {
        let start = triangle_points[edge];
        let end = triangle_points[(edge + 1) % 3];

        for i in 0..points_per_edge {
            let t = i as f32 / points_per_edge as f32;
            vertices.push(start.lerp(end, t));
        }
    }

    // Adicionar pontos restantes se necessário
    while vertices.len() < num_points {
        vertices.push(vertices[vertices.len() % 3]);
    }

    vertices
}

fn create_star_vertices(num_points: usize, outer_radius: f32, inner_radius: f32) -> Vec<Vec2> {
    let mut vertices = Vec::new();

    // Centro da estrela
    vertices.push(Vec2::ZERO);

    // Criar 10 vértices da estrela (5 externos + 5 internos intercalados)
    for i in 0..num_points {
        // O ângulo entre cada vértice é 1/10 de uma rotação completa
        let angle = i as f32 * std::f32::consts::PI / 5.0;

        // O raio dos vértices internos (índices pares) é inner_radius.
        // Para vértices externos (índices ímpares) é outer_radius.
        let radius = if i % 2 == 0 {
            inner_radius
        } else {
            outer_radius
        };

        // Adicionar a posição do vértice (usando sin para x e cos para y para manter compatibilidade)
        vertices.push(Vec2::new(radius * angle.sin(), radius * angle.cos()));
    }

    vertices
}

fn create_flower_vertices(num_points: usize, size: f32) -> Vec<Vec2> {
    let mut vertices = Vec::new();

    // Criar flor com 6 pétalas
    let num_petals = 6;
    let petal_width = std::f32::consts::TAU / num_petals as f32;

    for i in 0..num_points {
        let t = i as f32 / num_points as f32;
        let angle = t * std::f32::consts::TAU;

        // Determinar em qual pétala estamos
        let petal_angle = angle % petal_width;
        let petal_center_offset = petal_width / 2.0;

        // Distância do centro da pétala atual
        let distance_from_petal_center = (petal_angle - petal_center_offset).abs();

        // Criar formato de pétala usando função senoidal
        let petal_factor = (1.0 - distance_from_petal_center / petal_center_offset).max(0.0);
        let petal_shape = (petal_factor * std::f32::consts::PI).sin();

        // Raio base da flor (centro menor)
        let base_radius = size * 0.25;

        // Extensão da pétala
        let petal_extension = petal_shape * size * 0.6;

        // Raio final
        let final_radius = base_radius + petal_extension;

        // Adicionar pequena variação para tornar mais orgânico
        let organic_variation = (angle * 15.0).sin() * size * 0.03;
        let total_radius = final_radius + organic_variation;

        // Calcular posição final
        let x = angle.cos() * total_radius;
        let y = angle.sin() * total_radius;

        vertices.push(Vec2::new(x, y));
    }

    vertices
} // Função para criar mesh a partir de vértices
fn create_morphing_mesh(vertices: Vec<Vec2>) -> Mesh {
    let positions: Vec<[f32; 3]> = vertices.iter().map(|v| [v.x, v.y, 0.0]).collect();

    // Criar índices para triangulação (fan triangulation)
    let mut indices = Vec::new();
    for i in 1..vertices.len() - 1 {
        indices.push(0);
        indices.push(i as u32);
        indices.push((i + 1) as u32);
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(indices));

    // Adicionar normais
    let normals = vec![[0.0, 0.0, 1.0]; vertices.len()];
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    // Adicionar UVs
    let uvs: Vec<[f32; 2]> = vertices
        .iter()
        .map(|v| [(v.x / 100.0 + 0.5), (v.y / 100.0 + 0.5)])
        .collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh
}

// Função para atualizar vértices da mesh
fn update_mesh_vertices(mesh: &mut Mesh, vertices: &[Vec2]) {
    let positions: Vec<[f32; 3]> = vertices.iter().map(|v| [v.x, v.y, 0.0]).collect();

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

    // Atualizar UVs também
    let uvs: Vec<[f32; 2]> = vertices
        .iter()
        .map(|v| [(v.x / 100.0 + 0.5), (v.y / 100.0 + 0.5)])
        .collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
}

// Função de easing cúbico
fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

// Função para interpolar cores
fn interpolate_color(from: Color, to: Color, t: f32) -> Color {
    let from_srgba = from.to_srgba();
    let to_srgba = to.to_srgba();

    Color::srgba(
        from_srgba.red + (to_srgba.red - from_srgba.red) * t,
        from_srgba.green + (to_srgba.green - from_srgba.green) * t,
        from_srgba.blue + (to_srgba.blue - from_srgba.blue) * t,
        from_srgba.alpha + (to_srgba.alpha - from_srgba.alpha) * t,
    )
}
