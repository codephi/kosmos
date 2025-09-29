use bevy::prelude::*;
use super::easing::{Easing, lerp_with_easing};

/// Propriedade que pode ser animada
#[derive(Debug, Clone)]
pub enum AnimatableProperty {
    /// Posição 2D
    Position(Vec2),
    /// Rotação em radianos
    Rotation(f32),
    /// Escala uniforme ou não-uniforme
    Scale(Vec2),
    /// Cor
    Color(Color),
    /// Opacidade (0.0 a 1.0)
    Opacity(f32),
    /// Propriedade customizada com nome e valor
    Custom(String, f32),
}

impl AnimatableProperty {
    /// Interpola entre duas propriedades do mesmo tipo
    pub fn interpolate(&self, other: &Self, t: f32, easing: Easing) -> Option<AnimatableProperty> {
        match (self, other) {
            (AnimatableProperty::Position(a), AnimatableProperty::Position(b)) => {
                let x = lerp_with_easing(a.x, b.x, t, easing);
                let y = lerp_with_easing(a.y, b.y, t, easing);
                Some(AnimatableProperty::Position(Vec2::new(x, y)))
            }
            (AnimatableProperty::Rotation(a), AnimatableProperty::Rotation(b)) => {
                Some(AnimatableProperty::Rotation(lerp_with_easing(*a, *b, t, easing)))
            }
            (AnimatableProperty::Scale(a), AnimatableProperty::Scale(b)) => {
                let x = lerp_with_easing(a.x, b.x, t, easing);
                let y = lerp_with_easing(a.y, b.y, t, easing);
                Some(AnimatableProperty::Scale(Vec2::new(x, y)))
            }
            (AnimatableProperty::Color(a), AnimatableProperty::Color(b)) => {
                let rgba_a = a.to_linear();
                let rgba_b = b.to_linear();
                
                let r = lerp_with_easing(rgba_a.red, rgba_b.red, t, easing);
                let g = lerp_with_easing(rgba_a.green, rgba_b.green, t, easing);
                let b_val = lerp_with_easing(rgba_a.blue, rgba_b.blue, t, easing);
                let alpha = lerp_with_easing(rgba_a.alpha, rgba_b.alpha, t, easing);
                
                Some(AnimatableProperty::Color(Color::linear_rgba(r, g, b_val, alpha)))
            }
            (AnimatableProperty::Opacity(a), AnimatableProperty::Opacity(b)) => {
                Some(AnimatableProperty::Opacity(lerp_with_easing(*a, *b, t, easing)))
            }
            (AnimatableProperty::Custom(name_a, val_a), AnimatableProperty::Custom(name_b, val_b)) 
                if name_a == name_b => {
                Some(AnimatableProperty::Custom(
                    name_a.clone(),
                    lerp_with_easing(*val_a, *val_b, t, easing)
                ))
            }
            _ => None, // Tipos incompatíveis
        }
    }
    
    /// Aplica a propriedade a um Transform
    pub fn apply_to_transform(&self, transform: &mut Transform) {
        match self {
            AnimatableProperty::Position(pos) => {
                transform.translation.x = pos.x;
                transform.translation.y = pos.y;
            }
            AnimatableProperty::Rotation(rot) => {
                transform.rotation = Quat::from_rotation_z(*rot);
            }
            AnimatableProperty::Scale(scale) => {
                transform.scale.x = scale.x;
                transform.scale.y = scale.y;
            }
            _ => {}
        }
    }
    
    /// Aplica a propriedade a um ColorMaterial (para cor e opacidade)
    pub fn apply_to_material(&self, material: &mut ColorMaterial) {
        match self {
            AnimatableProperty::Color(color) => {
                material.color = *color;
            }
            AnimatableProperty::Opacity(opacity) => {
                material.color.set_alpha(*opacity);
            }
            _ => {}
        }
    }
}

/// Um keyframe representa um ponto específico na timeline
#[derive(Debug, Clone)]
pub struct Keyframe {
    /// Tempo em segundos desde o início da animação
    pub time: f32,
    /// Propriedade a ser animada
    pub property: AnimatableProperty,
    /// Função de easing para transição até este keyframe
    pub easing: Easing,
}

impl Keyframe {
    /// Cria um novo keyframe
    pub fn new(time: f32, property: AnimatableProperty, easing: Easing) -> Self {
        Self {
            time,
            property,
            easing,
        }
    }
    
    /// Cria um keyframe de posição
    pub fn position(time: f32, position: Vec2, easing: Easing) -> Self {
        Self::new(time, AnimatableProperty::Position(position), easing)
    }
    
    /// Cria um keyframe de rotação
    pub fn rotation(time: f32, rotation: f32, easing: Easing) -> Self {
        Self::new(time, AnimatableProperty::Rotation(rotation), easing)
    }
    
    /// Cria um keyframe de escala
    pub fn scale(time: f32, scale: Vec2, easing: Easing) -> Self {
        Self::new(time, AnimatableProperty::Scale(scale), easing)
    }
    
    /// Cria um keyframe de cor
    pub fn color(time: f32, color: Color, easing: Easing) -> Self {
        Self::new(time, AnimatableProperty::Color(color), easing)
    }
    
    /// Cria um keyframe de opacidade
    pub fn opacity(time: f32, opacity: f32, easing: Easing) -> Self {
        Self::new(time, AnimatableProperty::Opacity(opacity), easing)
    }
}

/// Grupo de keyframes que afetam a mesma propriedade
#[derive(Debug, Clone)]
pub struct KeyframeTrack {
    /// Nome da track (ex: "position", "rotation", etc)
    pub name: String,
    /// Lista de keyframes ordenados por tempo
    pub keyframes: Vec<Keyframe>,
}

impl KeyframeTrack {
    pub fn new(name: String) -> Self {
        Self {
            name,
            keyframes: Vec::new(),
        }
    }
    
    /// Adiciona um keyframe à track
    pub fn add_keyframe(&mut self, keyframe: Keyframe) {
        self.keyframes.push(keyframe);
        // Manter ordenado por tempo
        self.keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }
    
    /// Obtém o valor interpolado no tempo especificado
    pub fn get_value_at(&self, time: f32) -> Option<AnimatableProperty> {
        if self.keyframes.is_empty() {
            return None;
        }
        
        // Se o tempo está antes do primeiro keyframe
        if time <= self.keyframes[0].time {
            return Some(self.keyframes[0].property.clone());
        }
        
        // Se o tempo está depois do último keyframe
        if time >= self.keyframes.last().unwrap().time {
            return Some(self.keyframes.last().unwrap().property.clone());
        }
        
        // Encontrar os dois keyframes entre os quais interpolar
        for i in 0..self.keyframes.len() - 1 {
            let current = &self.keyframes[i];
            let next = &self.keyframes[i + 1];
            
            if time >= current.time && time <= next.time {
                // Calcular o t normalizado entre os dois keyframes
                let t = (time - current.time) / (next.time - current.time);
                return current.property.interpolate(&next.property, t, next.easing);
            }
        }
        
        None
    }
    
    /// Obtém a duração total da track
    pub fn duration(&self) -> f32 {
        self.keyframes.last().map(|kf| kf.time).unwrap_or(0.0)
    }
}