//! Módulo de Animações GPU para Kosmos
//! 
//! Executa animações completamente na GPU para performance máxima.
//! Capaz de animar milhares de objetos simultaneamente sem impacto na CPU.

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, Mesh2d, MeshMaterial2d},
};

/// Plugin para animações GPU
pub struct GpuAnimationPlugin;

impl Plugin for GpuAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<AnimatedMaterial>::default())
            .add_systems(Update, update_material_time);
    }
}

/// Flags para controlar quais animações estão ativas
#[derive(Debug, Clone, Copy)]
pub struct AnimationFlags(u32);

impl AnimationFlags {
    pub const NONE: Self = Self(0);
    pub const ROTATE: Self = Self(1);
    pub const ORBIT: Self = Self(2);
    pub const PULSE: Self = Self(4);
    pub const WAVE: Self = Self(8);
    pub const COLOR_CYCLE: Self = Self(16);
    pub const RAINBOW: Self = Self(32);
    pub const ALL: Self = Self(63);
    
    pub fn with_rotation(mut self) -> Self {
        self.0 |= Self::ROTATE.0;
        self
    }
    
    pub fn with_orbit(mut self) -> Self {
        self.0 |= Self::ORBIT.0;
        self
    }
    
    pub fn with_pulse(mut self) -> Self {
        self.0 |= Self::PULSE.0;
        self
    }
    
    pub fn with_wave(mut self) -> Self {
        self.0 |= Self::WAVE.0;
        self
    }
    
    pub fn with_color_cycle(mut self) -> Self {
        self.0 |= Self::COLOR_CYCLE.0;
        self
    }
    
    pub fn with_rainbow(mut self) -> Self {
        self.0 |= Self::RAINBOW.0;
        self
    }
}

impl Default for AnimationFlags {
    fn default() -> Self {
        Self::NONE
    }
}

/// Material animado que executa na GPU
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct AnimatedMaterial {
    /// Tempo da animação (atualizado automaticamente)
    #[uniform(0)]
    pub time: f32,
    
    /// Velocidade de rotação (radianos por segundo)
    #[uniform(0)]
    pub rotation_speed: f32,
    
    /// Velocidade orbital
    #[uniform(0)]
    pub orbit_speed: f32,
    
    /// Velocidade de pulsação
    #[uniform(0)]
    pub pulse_speed: f32,
    
    /// Raio da órbita
    #[uniform(0)]
    pub orbit_radius: f32,
    
    /// Amplitude da pulsação (0.0 a 1.0)
    #[uniform(0)]
    pub pulse_amplitude: f32,
    
    /// Amplitude da onda
    #[uniform(0)]
    pub wave_amplitude: f32,
    
    /// Frequência da onda
    #[uniform(0)]
    pub wave_frequency: f32,
    
    /// Cor inicial
    #[uniform(0)]
    pub color_start: LinearRgba,
    
    /// Cor final (para transição)
    #[uniform(0)]
    pub color_end: LinearRgba,
    
    /// Velocidade de mudança de cor
    #[uniform(0)]
    pub color_speed: f32,
    
    /// Flags de animação (bitflags)
    #[uniform(0)]
    pub animation_flags: u32,
    
    /// Offset de tempo para variar animações
    #[uniform(0)]
    pub time_offset: f32,
}

impl Material2d for AnimatedMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/animated_material.wgsl".into()
    }
    
    fn vertex_shader() -> ShaderRef {
        "shaders/animated_material.wgsl".into()
    }
}

impl Default for AnimatedMaterial {
    fn default() -> Self {
        Self {
            time: 0.0,
            rotation_speed: 1.0,
            orbit_speed: 1.0,
            pulse_speed: 2.0,
            orbit_radius: 50.0,
            pulse_amplitude: 0.3,
            wave_amplitude: 10.0,
            wave_frequency: 0.1,
            color_start: LinearRgba::rgb(1.0, 1.0, 1.0),
            color_end: LinearRgba::rgb(0.5, 0.5, 1.0),
            color_speed: 1.0,
            animation_flags: AnimationFlags::ALL.0,
            time_offset: 0.0,
        }
    }
}

/// Builder para criar materiais animados facilmente
pub struct AnimatedMaterialBuilder {
    material: AnimatedMaterial,
}

impl AnimatedMaterialBuilder {
    pub fn new() -> Self {
        Self {
            material: AnimatedMaterial::default(),
        }
    }
    
    pub fn with_rotation_speed(mut self, speed: f32) -> Self {
        self.material.rotation_speed = speed;
        self
    }
    
    pub fn with_orbit(mut self, speed: f32, radius: f32) -> Self {
        self.material.orbit_speed = speed;
        self.material.orbit_radius = radius;
        self
    }
    
    pub fn with_pulse(mut self, speed: f32, amplitude: f32) -> Self {
        self.material.pulse_speed = speed;
        self.material.pulse_amplitude = amplitude;
        self
    }
    
    pub fn with_wave(mut self, amplitude: f32, frequency: f32) -> Self {
        self.material.wave_amplitude = amplitude;
        self.material.wave_frequency = frequency;
        self
    }
    
    pub fn with_colors(mut self, start: Color, end: Color, speed: f32) -> Self {
        self.material.color_start = start.into();
        self.material.color_end = end.into();
        self.material.color_speed = speed;
        self
    }
    
    pub fn with_flags(mut self, flags: AnimationFlags) -> Self {
        self.material.animation_flags = flags.0;
        self
    }
    
    pub fn with_time_offset(mut self, offset: f32) -> Self {
        self.material.time_offset = offset;
        self
    }
    
    pub fn build(self) -> AnimatedMaterial {
        self.material
    }
}

/// Sistema que atualiza o tempo dos materiais
fn update_material_time(
    time: Res<Time>,
    mut materials: ResMut<Assets<AnimatedMaterial>>,
) {
    let elapsed = time.elapsed_secs();
    for (_, material) in materials.iter_mut() {
        material.time = elapsed;
    }
}

/// Helper para criar geometrias animadas na GPU
pub fn spawn_gpu_animated_shape(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<AnimatedMaterial>>,
    mesh: Mesh,
    material: AnimatedMaterial,
    position: Vec3,
) -> Entity {
    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(material)),
        Transform::from_translation(position),
        GpuAnimated,  // Marker component
    )).id()
}

/// Marker component para identificar entidades animadas na GPU
#[derive(Component)]
pub struct GpuAnimated;

/// Cria um campo de partículas/formas animadas na GPU
pub fn create_animated_field(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<AnimatedMaterial>>,
    count: usize,
    base_material: AnimatedMaterial,
    area_size: Vec2,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    for i in 0..count {
        // Variar os parâmetros para cada objeto
        let mut material = base_material.clone();
        material.time_offset = i as f32 * 0.1;
        material.rotation_speed = rng.gen_range(0.5..2.0);
        material.pulse_speed = rng.gen_range(1.0..3.0);
        material.orbit_speed = rng.gen_range(0.5..1.5);
        
        // Posição aleatória
        let x = rng.gen_range(-area_size.x/2.0..area_size.x/2.0);
        let y = rng.gen_range(-area_size.y/2.0..area_size.y/2.0);
        
        // Escolher forma aleatória
        let mesh = match rng.gen_range(0..5) {
            0 => Mesh::from(Circle::new(rng.gen_range(5.0..15.0))),
            1 => Mesh::from(Rectangle::new(
                rng.gen_range(10.0..20.0),
                rng.gen_range(10.0..20.0)
            )),
            2 => Mesh::from(RegularPolygon::new(
                rng.gen_range(8.0..16.0),
                rng.gen_range(3..8)
            )),
            3 => Mesh::from(Ellipse::new(
                rng.gen_range(10.0..20.0),
                rng.gen_range(5.0..15.0)
            )),
            _ => Mesh::from(Triangle2d::new(
                Vec2::new(0.0, rng.gen_range(10.0..20.0)),
                Vec2::new(-rng.gen_range(5.0..15.0), -rng.gen_range(5.0..15.0)),
                Vec2::new(rng.gen_range(5.0..15.0), -rng.gen_range(5.0..15.0))
            )),
        };
        
        spawn_gpu_animated_shape(
            commands,
            meshes,
            materials,
            mesh,
            material,
            Vec3::new(x, y, 0.0),
        );
    }
}