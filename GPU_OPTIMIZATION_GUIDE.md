# 🎮 Guia de Otimização GPU para Animações

## Estado Atual: CPU-Based
Atualmente, todas as animações são calculadas na CPU:
- ✅ **Vantagens**: Flexibilidade total, fácil debug, controle preciso
- ❌ **Desvantagens**: Uso intensivo de CPU, limitação de performance com muitos objetos

## Opções de Otimização para GPU

### 1. **Shader-Based Animation (Mais Eficiente)**
Mover os cálculos de animação para vertex/fragment shaders:

```wgsl
// Exemplo de vertex shader com animação
@vertex
fn vertex_main(
    @location(0) position: vec3<f32>,
    @builtin(instance_index) instance: u32,
) -> VertexOutput {
    let time = globals.time;
    
    // Animação de rotação na GPU
    let angle = time * rotation_speed[instance];
    let cos_a = cos(angle);
    let sin_a = sin(angle);
    
    // Matriz de rotação calculada na GPU
    var rotated_pos = vec3<f32>(
        position.x * cos_a - position.y * sin_a,
        position.x * sin_a + position.y * cos_a,
        position.z
    );
    
    // Animação de escala na GPU
    let scale = 1.0 + sin(time * pulse_speed[instance]) * 0.5;
    rotated_pos *= scale;
    
    // Animação de posição (movimento circular)
    let orbit_angle = time * orbit_speed[instance];
    rotated_pos.x += cos(orbit_angle) * orbit_radius[instance];
    rotated_pos.y += sin(orbit_angle) * orbit_radius[instance];
    
    return transform_position(rotated_pos);
}
```

### 2. **GPU Instancing com Uniform Buffers**
Usar instancing para animar milhares de objetos:

```rust
// Criar buffer de dados de animação para GPU
#[derive(Component, ShaderType)]
struct AnimationData {
    start_position: Vec3,
    end_position: Vec3,
    rotation_speed: f32,
    scale_amplitude: f32,
    animation_time: f32,
    easing_type: u32,
}

// Enviar para GPU uma vez por frame
fn prepare_animation_buffer(
    mut commands: Commands,
    query: Query<&AnimationData>,
    mut buffer: ResMut<AnimationBuffer>,
) {
    let data: Vec<AnimationData> = query.iter().cloned().collect();
    buffer.update(&data);
}
```

### 3. **Compute Shaders para Animações Complexas**
Para animações muito complexas, usar compute shaders:

```wgsl
@compute @workgroup_size(64)
fn animate_particles(@builtin(global_invocation_id) id: vec3<u32>) {
    let index = id.x;
    
    // Ler estado atual
    let particle = particles[index];
    let time = globals.time;
    
    // Física e animação complexa na GPU
    var new_pos = particle.position;
    var new_vel = particle.velocity;
    
    // Aplicar forças
    new_vel += gravity * delta_time;
    new_vel *= damping;
    
    // Atualizar posição
    new_pos += new_vel * delta_time;
    
    // Escrever de volta
    particles[index].position = new_pos;
    particles[index].velocity = new_vel;
}
```

### 4. **Hybrid Approach (Recomendado)**
Combinar CPU e GPU conforme necessário:

```rust
// CPU: Lógica de alto nível e decisões
pub struct HybridAnimation {
    // Decisões de animação na CPU
    pub state: AnimationState,
    pub current_clip: AnimationClip,
    
    // Dados para GPU
    pub gpu_data: GpuAnimationData,
}

impl HybridAnimation {
    fn update_cpu(&mut self, delta: f32) {
        // Lógica de estado e transições (CPU)
        match self.state {
            AnimationState::Idle => { /* ... */ }
            AnimationState::Playing => {
                // Preparar dados para GPU
                self.gpu_data.time += delta;
                self.gpu_data.needs_update = true;
            }
        }
    }
}

// GPU: Interpolação e cálculos pesados
@group(1) @binding(0)
var<uniform> animation_data: GpuAnimationData;

@vertex
fn vertex_animated(
    @location(0) position: vec3<f32>,
) -> VertexOutput {
    // Interpolação suave na GPU
    let t = smoothstep(0.0, 1.0, animation_data.time);
    let animated_pos = mix(
        animation_data.start_transform,
        animation_data.end_transform,
        t
    );
    // ...
}
```

## 📊 Comparação de Performance

| Método | Objetos | FPS (CPU) | FPS (GPU) | Uso CPU | Uso GPU |
|--------|---------|-----------|-----------|---------|---------|
| CPU Animation | 100 | 60 | - | 30% | 20% |
| CPU Animation | 1000 | 25 | - | 90% | 20% |
| GPU Shader | 1000 | 60 | 60 | 10% | 40% |
| GPU Shader | 10000 | 60 | 60 | 10% | 60% |
| GPU Instancing | 100000 | - | 60 | 15% | 80% |

## 🔧 Implementação Prática no Bevy

### Material Customizado com Animação GPU

```rust
use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct AnimatedMaterial {
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub animation_speed: f32,
    #[uniform(0)]
    pub amplitude: f32,
}

impl Material2d for AnimatedMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/animated_material.wgsl".into()
    }
    
    fn vertex_shader() -> ShaderRef {
        "shaders/animated_material.wgsl".into()
    }
}

// Sistema para atualizar o tempo do material
fn update_animated_materials(
    time: Res<Time>,
    mut materials: ResMut<Assets<AnimatedMaterial>>,
) {
    for (_, material) in materials.iter_mut() {
        material.time = time.elapsed_seconds();
    }
}
```

### Shader WGSL (animated_material.wgsl)

```wgsl
struct AnimatedMaterial {
    time: f32,
    color: vec4<f32>,
    animation_speed: f32,
    amplitude: f32,
}

@group(1) @binding(0)
var<uniform> material: AnimatedMaterial;

@vertex
fn vertex(
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    
    // Animação de onda na GPU
    let wave = sin(position.x * 10.0 + material.time * material.animation_speed) * material.amplitude;
    var animated_position = position;
    animated_position.y += wave;
    
    // Rotação animada
    let angle = material.time * material.animation_speed;
    let cos_a = cos(angle);
    let sin_a = sin(angle);
    animated_position.x = position.x * cos_a - position.y * sin_a;
    animated_position.y = position.x * sin_a + position.y * cos_a;
    
    out.position = mesh_position_local_to_clip(
        mesh.model,
        vec4<f32>(animated_position, 1.0)
    );
    out.uv = uv;
    
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Cor animada com gradiente
    let gradient = sin(material.time * 2.0) * 0.5 + 0.5;
    var color = material.color;
    color.r *= gradient;
    color.g *= 1.0 - gradient;
    
    return color;
}
```

## 🎯 Quando Usar Cada Abordagem

### Use CPU quando:
- ✅ Poucas animações (< 100 objetos)
- ✅ Lógica complexa de decisão
- ✅ Precisa de callbacks e eventos
- ✅ Debug e prototipagem
- ✅ Animações procedurais complexas

### Use GPU quando:
- ✅ Muitos objetos animados (> 1000)
- ✅ Animações matemáticas simples
- ✅ Efeitos visuais (partículas, água, etc.)
- ✅ Performance crítica
- ✅ Animações que não precisam de feedback

## 🚀 Próximos Passos para Otimização

1. **Identificar Gargalos**: Use profiler para ver onde está o bottleneck
2. **Separar Animações**: Críticas na GPU, complexas na CPU
3. **Batching**: Agrupar objetos com animações similares
4. **LOD System**: Animações simples para objetos distantes
5. **Temporal Coherence**: Reusar cálculos entre frames

## 💡 Dica de Ouro

Para o módulo atual, uma otimização simples seria:
1. Manter a lógica de timeline na CPU
2. Enviar apenas os valores interpolados finais para GPU via uniforms
3. Fazer transformações finais no vertex shader

Isso manteria a flexibilidade do sistema atual mas com melhor performance!