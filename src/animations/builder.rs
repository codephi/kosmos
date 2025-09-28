use bevy::prelude::*;
use super::timeline::{AnimationTimeline, AnimationMode};
use super::keyframe::{Keyframe, AnimatableProperty};
use super::easing::Easing;

/// Builder para criar animações de forma fluente
pub struct AnimationBuilder {
    timeline: AnimationTimeline,
    current_time: f32,
}

impl AnimationBuilder {
    /// Cria um novo builder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            timeline: AnimationTimeline::new(name.into()),
            current_time: 0.0,
        }
    }
    
    /// Cria um builder com nome padrão
    pub fn default() -> Self {
        Self::new("default_animation")
    }
    
    // === Métodos de movimento ===
    
    /// Adiciona uma animação de movimento para uma posição
    pub fn move_to(mut self, position: Vec2, duration: f32, easing: Easing) -> Self {
        let start_time = self.current_time;
        self.current_time += duration;
        
        self.timeline.add_keyframe(
            "position",
            Keyframe::position(start_time + duration, position, easing)
        );
        
        self
    }
    
    /// Adiciona uma animação de movimento relativo
    pub fn move_by(mut self, delta: Vec2, duration: f32, easing: Easing) -> Self {
        // Nota: movimento relativo requer conhecer a posição inicial
        // Por simplicidade, vamos assumir que a posição inicial é Vec2::ZERO
        // Em uma implementação mais completa, isso deveria ser tratado diferentemente
        let start_time = self.current_time;
        self.current_time += duration;
        
        // Adicionar keyframe inicial se for o primeiro
        if start_time == 0.0 {
            self.timeline.add_keyframe(
                "position",
                Keyframe::position(0.0, Vec2::ZERO, Easing::Linear)
            );
        }
        
        // Posição é relativa ao último keyframe
        self.timeline.add_keyframe(
            "position",
            Keyframe::position(start_time + duration, delta, easing)
        );
        
        self
    }
    
    // === Métodos de rotação ===
    
    /// Adiciona uma animação de rotação para um ângulo específico
    pub fn rotate_to(mut self, angle: f32, duration: f32, easing: Easing) -> Self {
        let start_time = self.current_time;
        self.current_time += duration;
        
        self.timeline.add_keyframe(
            "rotation",
            Keyframe::rotation(start_time + duration, angle, easing)
        );
        
        self
    }
    
    /// Adiciona uma animação de rotação relativa
    pub fn rotate_by(mut self, delta_angle: f32, duration: f32, easing: Easing) -> Self {
        let start_time = self.current_time;
        self.current_time += duration;
        
        // Adicionar keyframe inicial se for o primeiro
        if start_time == 0.0 {
            self.timeline.add_keyframe(
                "rotation",
                Keyframe::rotation(0.0, 0.0, Easing::Linear)
            );
        }
        
        self.timeline.add_keyframe(
            "rotation",
            Keyframe::rotation(start_time + duration, delta_angle, easing)
        );
        
        self
    }
    
    // === Métodos de escala ===
    
    /// Adiciona uma animação de escala uniforme
    pub fn scale_to(self, scale: f32, duration: f32, easing: Easing) -> Self {
        self.scale_to_xy(Vec2::splat(scale), duration, easing)
    }
    
    /// Adiciona uma animação de escala não uniforme
    pub fn scale_to_xy(mut self, scale: Vec2, duration: f32, easing: Easing) -> Self {
        let start_time = self.current_time;
        self.current_time += duration;
        
        self.timeline.add_keyframe(
            "scale",
            Keyframe::scale(start_time + duration, scale, easing)
        );
        
        self
    }
    
    // === Métodos de cor ===
    
    /// Adiciona uma animação de mudança de cor
    pub fn color_to(mut self, color: Color, duration: f32, easing: Easing) -> Self {
        let start_time = self.current_time;
        self.current_time += duration;
        
        self.timeline.add_keyframe(
            "color",
            Keyframe::color(start_time + duration, color, easing)
        );
        
        self
    }
    
    /// Adiciona uma animação de fade (opacidade)
    pub fn fade_to(mut self, opacity: f32, duration: f32, easing: Easing) -> Self {
        let start_time = self.current_time;
        self.current_time += duration;
        
        self.timeline.add_keyframe(
            "opacity",
            Keyframe::opacity(start_time + duration, opacity.clamp(0.0, 1.0), easing)
        );
        
        self
    }
    
    /// Fade in (aparecer)
    pub fn fade_in(self, duration: f32, easing: Easing) -> Self {
        self.fade_to(1.0, duration, easing)
    }
    
    /// Fade out (desaparecer)
    pub fn fade_out(self, duration: f32, easing: Easing) -> Self {
        self.fade_to(0.0, duration, easing)
    }
    
    // === Métodos de tempo ===
    
    /// Adiciona um delay/pausa na animação
    pub fn wait(mut self, duration: f32) -> Self {
        self.current_time += duration;
        self
    }
    
    /// Define um delay inicial antes da animação começar
    pub fn with_delay(mut self, delay: f32) -> Self {
        self.timeline.set_delay(delay);
        self
    }
    
    // === Métodos de configuração ===
    
    /// Define o modo de execução da animação
    pub fn with_mode(mut self, mode: AnimationMode) -> Self {
        self.timeline.set_mode(mode);
        self
    }
    
    /// Faz a animação repetir infinitamente
    pub fn repeat(self) -> Self {
        self.with_mode(AnimationMode::Loop)
    }
    
    /// Faz a animação repetir um número específico de vezes
    pub fn repeat_times(self, times: u32) -> Self {
        self.with_mode(AnimationMode::Repeat(times))
    }
    
    /// Faz a animação ir e voltar (ping-pong)
    pub fn ping_pong(self) -> Self {
        self.with_mode(AnimationMode::PingPong)
    }
    
    /// Define a velocidade de reprodução
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.timeline.set_speed(speed);
        self
    }
    
    // === Métodos de construção personalizada ===
    
    /// Adiciona um keyframe personalizado
    pub fn add_keyframe(mut self, track_name: &str, time: f32, property: AnimatableProperty, easing: Easing) -> Self {
        self.timeline.add_keyframe(track_name, Keyframe::new(time, property, easing));
        self.current_time = self.current_time.max(time);
        self
    }
    
    /// Adiciona múltiplos keyframes de uma vez
    pub fn then(mut self, f: impl FnOnce(&mut AnimationTimeline, f32) -> f32) -> Self {
        self.current_time = f(&mut self.timeline, self.current_time);
        self
    }
    
    // === Callbacks ===
    
    /// Define callback para quando a animação começar
    pub fn on_start<F>(mut self, callback: F) -> Self 
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.timeline.on_start(callback);
        self
    }
    
    /// Define callback para quando a animação terminar
    pub fn on_complete<F>(mut self, callback: F) -> Self 
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.timeline.on_complete(callback);
        self
    }
    
    // === Construção final ===
    
    /// Constrói e retorna a timeline final
    pub fn build(mut self) -> AnimationTimeline {
        // Adicionar keyframes iniciais para propriedades que não têm
        // um keyframe no tempo 0
        for (track_name, track) in self.timeline.tracks.clone() {
            if !track.keyframes.is_empty() && track.keyframes[0].time > 0.0 {
                // Adicionar keyframe inicial com valor padrão
                let initial_property = match track.keyframes[0].property {
                    AnimatableProperty::Position(_) => AnimatableProperty::Position(Vec2::ZERO),
                    AnimatableProperty::Rotation(_) => AnimatableProperty::Rotation(0.0),
                    AnimatableProperty::Scale(_) => AnimatableProperty::Scale(Vec2::ONE),
                    AnimatableProperty::Color(_) => AnimatableProperty::Color(Color::WHITE),
                    AnimatableProperty::Opacity(_) => AnimatableProperty::Opacity(1.0),
                    AnimatableProperty::Custom(ref name, _) => AnimatableProperty::Custom(name.clone(), 0.0),
                };
                
                self.timeline.tracks.get_mut(&track_name).unwrap()
                    .keyframes.insert(0, Keyframe::new(0.0, initial_property, Easing::Linear));
            }
        }
        
        self.timeline
    }
    
    /// Constrói e inicia a animação automaticamente
    pub fn build_and_play(self) -> AnimationTimeline {
        let mut timeline = self.build();
        timeline.play();
        timeline
    }
}

/// Helper para criar animações predefinidas comuns
pub struct AnimationPresets;

impl AnimationPresets {
    /// Cria uma animação de bounce (salto)
    pub fn bounce(height: f32, duration: f32) -> AnimationTimeline {
        AnimationBuilder::new("bounce")
            .move_to(Vec2::new(0.0, height), duration / 2.0, Easing::EaseOutQuad)
            .move_to(Vec2::ZERO, duration / 2.0, Easing::EaseInQuad)
            .repeat()
            .build()
    }
    
    /// Cria uma animação de rotação contínua
    pub fn spin(speed: f32) -> AnimationTimeline {
        AnimationBuilder::new("spin")
            .rotate_to(std::f32::consts::TAU, 1.0 / speed, Easing::Linear)
            .repeat()
            .build()
    }
    
    /// Cria uma animação de pulsação
    pub fn pulse(scale: f32, duration: f32) -> AnimationTimeline {
        AnimationBuilder::new("pulse")
            .scale_to(scale, duration / 2.0, Easing::EaseInOut)
            .scale_to(1.0, duration / 2.0, Easing::EaseInOut)
            .repeat()
            .build()
    }
    
    /// Cria uma animação de shake (tremor)
    pub fn shake(intensity: f32, duration: f32) -> AnimationTimeline {
        let steps = 10;
        let step_duration = duration / steps as f32;
        let mut builder = AnimationBuilder::new("shake");
        
        for i in 0..steps {
            let x = if i % 2 == 0 { intensity } else { -intensity };
            let y = if i % 3 == 0 { intensity * 0.5 } else { -intensity * 0.5 };
            builder = builder.move_to(Vec2::new(x, y), step_duration, Easing::Linear);
        }
        
        builder.move_to(Vec2::ZERO, step_duration, Easing::EaseOut).build()
    }
    
    /// Cria uma animação de fade in
    pub fn fade_in(duration: f32) -> AnimationTimeline {
        AnimationBuilder::new("fade_in")
            .fade_to(0.0, 0.0, Easing::Linear)
            .fade_to(1.0, duration, Easing::EaseIn)
            .build()
    }
    
    /// Cria uma animação de fade out
    pub fn fade_out(duration: f32) -> AnimationTimeline {
        AnimationBuilder::new("fade_out")
            .fade_to(1.0, 0.0, Easing::Linear)
            .fade_to(0.0, duration, Easing::EaseOut)
            .build()
    }
}