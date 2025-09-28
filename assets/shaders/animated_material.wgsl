// Shader para animações executadas inteiramente na GPU

#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::view
#import bevy_sprite::mesh2d_bindings::mesh

struct AnimatedMaterial {
    // Tempo global da animação
    time: f32,
    // Velocidades de animação
    rotation_speed: f32,
    orbit_speed: f32,
    pulse_speed: f32,
    // Parâmetros de movimento
    orbit_radius: f32,
    pulse_amplitude: f32,
    wave_amplitude: f32,
    wave_frequency: f32,
    // Cores para transição
    color_start: vec4<f32>,
    color_end: vec4<f32>,
    color_speed: f32,
    // Tipo de animação (bitflags)
    animation_flags: u32,
    // Offset para variar animações entre objetos
    time_offset: f32,
}

@group(2) @binding(0)
var<uniform> material: AnimatedMaterial;

const FLAG_ROTATE: u32 = 1u;
const FLAG_ORBIT: u32 = 2u;
const FLAG_PULSE: u32 = 4u;
const FLAG_WAVE: u32 = 8u;
const FLAG_COLOR_CYCLE: u32 = 16u;
const FLAG_RAINBOW: u32 = 32u;

// Funções de easing implementadas na GPU
fn ease_in_out_quad(t: f32) -> f32 {
    if (t < 0.5) {
        return 2.0 * t * t;
    } else {
        return -1.0 + (4.0 - 2.0 * t) * t;
    }
}

fn ease_in_out_cubic(t: f32) -> f32 {
    if (t < 0.5) {
        return 4.0 * t * t * t;
    } else {
        let t2 = 2.0 * t - 2.0;
        return 1.0 + t2 * t2 * t2 / 2.0;
    }
}

fn ease_bounce(t: f32) -> f32 {
    if (t < 1.0 / 2.75) {
        return 7.5625 * t * t;
    } else if (t < 2.0 / 2.75) {
        let t2 = t - 1.5 / 2.75;
        return 7.5625 * t2 * t2 + 0.75;
    } else if (t < 2.5 / 2.75) {
        let t2 = t - 2.25 / 2.75;
        return 7.5625 * t2 * t2 + 0.9375;
    } else {
        let t2 = t - 2.625 / 2.75;
        return 7.5625 * t2 * t2 + 0.984375;
    }
}

// Função para criar cor arco-íris
fn rainbow_color(t: f32) -> vec3<f32> {
    let r = sin(t * 6.28318 + 0.0) * 0.5 + 0.5;
    let g = sin(t * 6.28318 + 2.09439) * 0.5 + 0.5;
    let b = sin(t * 6.28318 + 4.18879) * 0.5 + 0.5;
    return vec3<f32>(r, g, b);
}

@vertex
fn vertex(
    @builtin(vertex_index) vertex_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    
    // Tempo com offset para variar animações
    let t = material.time + material.time_offset;
    
    // Posição inicial
    var animated_pos = position;
    
    // === ROTAÇÃO ===
    if ((material.animation_flags & FLAG_ROTATE) != 0u) {
        let angle = t * material.rotation_speed;
        let cos_a = cos(angle);
        let sin_a = sin(angle);
        
        let rotated_x = animated_pos.x * cos_a - animated_pos.y * sin_a;
        let rotated_y = animated_pos.x * sin_a + animated_pos.y * cos_a;
        animated_pos.x = rotated_x;
        animated_pos.y = rotated_y;
    }
    
    // === PULSAÇÃO (ESCALA) ===
    if ((material.animation_flags & FLAG_PULSE) != 0u) {
        let pulse_t = sin(t * material.pulse_speed);
        let scale = 1.0 + pulse_t * material.pulse_amplitude;
        
        // Aplicar easing para pulsação mais suave
        let eased_scale = 1.0 + ease_in_out_cubic(pulse_t * 0.5 + 0.5) * material.pulse_amplitude;
        
        animated_pos.x *= eased_scale;
        animated_pos.y *= eased_scale;
    }
    
    // === MOVIMENTO ORBITAL ===
    if ((material.animation_flags & FLAG_ORBIT) != 0u) {
        let orbit_angle = t * material.orbit_speed;
        
        // Movimento em forma de 8 (lemniscata)
        let figure8_x = material.orbit_radius * sin(orbit_angle);
        let figure8_y = material.orbit_radius * sin(orbit_angle * 2.0) * 0.5;
        
        animated_pos.x += figure8_x;
        animated_pos.y += figure8_y;
    }
    
    // === EFEITO ONDA ===
    if ((material.animation_flags & FLAG_WAVE) != 0u) {
        let wave_x = sin(position.y * material.wave_frequency + t * 3.0) * material.wave_amplitude;
        let wave_y = cos(position.x * material.wave_frequency + t * 2.0) * material.wave_amplitude * 0.5;
        
        animated_pos.x += wave_x;
        animated_pos.y += wave_y;
    }
    
    // Transformar para espaço de tela
    let model_matrix = mesh.model;
    let world_position = model_matrix * vec4<f32>(animated_pos, 1.0);
    out.position = view.clip_from_world * world_position;
    
    // Passar UV e dados para fragment shader
    out.uv = uv;
    
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let t = material.time + material.time_offset;
    var color = material.color_start;
    
    // === TRANSIÇÃO DE COR ===
    if ((material.animation_flags & FLAG_COLOR_CYCLE) != 0u) {
        // Ciclo suave entre cor inicial e final
        let color_t = sin(t * material.color_speed) * 0.5 + 0.5;
        let eased_t = ease_in_out_quad(color_t);
        
        color = mix(material.color_start, material.color_end, eased_t);
    }
    
    // === ARCO-ÍRIS ===
    if ((material.animation_flags & FLAG_RAINBOW) != 0u) {
        let rainbow_t = fract(t * material.color_speed * 0.2);
        let rainbow = rainbow_color(rainbow_t + in.uv.x * 0.3);
        color = vec4<f32>(rainbow, color.a);
    }
    
    // Adicionar efeito de gradiente baseado em UV
    let gradient = sin(in.uv.x * 3.14159) * sin(in.uv.y * 3.14159);
    color.rgb *= 0.8 + gradient * 0.2;
    
    // Efeito de brilho pulsante
    let glow = sin(t * 4.0) * 0.1 + 0.9;
    color.rgb *= glow;
    
    return color;
}