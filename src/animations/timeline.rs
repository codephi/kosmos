use bevy::prelude::*;
use super::keyframe::{Keyframe, KeyframeTrack, AnimatableProperty};
use std::collections::HashMap;

/// Estado de uma animação
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationState {
    /// Animação não iniciada
    Idle,
    /// Animação em execução
    Playing,
    /// Animação pausada
    Paused,
    /// Animação finalizada
    Finished,
}

/// Modo de execução da animação
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationMode {
    /// Executar uma vez e parar
    Once,
    /// Loop infinito
    Loop,
    /// Vai e volta (ping-pong)
    PingPong,
    /// Loop com número específico de repetições
    Repeat(u32),
}

/// Timeline de animação que gerencia múltiplas tracks de propriedades
pub struct AnimationTimeline {
    /// Nome da animação
    pub name: String,
    /// Tracks de propriedades (cada track anima uma propriedade diferente)
    pub tracks: HashMap<String, KeyframeTrack>,
    /// Duração total da animação em segundos
    pub duration: f32,
    /// Modo de execução
    pub mode: AnimationMode,
    /// Estado atual
    pub state: AnimationState,
    /// Tempo decorrido desde o início
    pub elapsed_time: f32,
    /// Velocidade de reprodução (1.0 = normal, 2.0 = 2x mais rápido, 0.5 = metade da velocidade)
    pub speed: f32,
    /// Contador de repetições (para modo Repeat)
    pub repeat_count: u32,
    /// Direção da animação (para modo PingPong)
    pub reverse: bool,
    /// Delay inicial antes de começar a animação
    pub delay: f32,
    /// Tempo de delay decorrido
    pub delay_elapsed: f32,
    /// Callbacks opcionais
    pub on_complete: Option<Box<dyn Fn() + Send + Sync>>,
    pub on_start: Option<Box<dyn Fn() + Send + Sync>>,
}

impl AnimationTimeline {
    /// Cria uma nova timeline vazia
    pub fn new(name: String) -> Self {
        Self {
            name,
            tracks: HashMap::new(),
            duration: 0.0,
            mode: AnimationMode::Once,
            state: AnimationState::Idle,
            elapsed_time: 0.0,
            speed: 1.0,
            repeat_count: 0,
            reverse: false,
            delay: 0.0,
            delay_elapsed: 0.0,
            on_complete: None,
            on_start: None,
        }
    }
    
    /// Adiciona um keyframe à timeline
    pub fn add_keyframe(&mut self, track_name: &str, keyframe: Keyframe) {
        let track = self.tracks
            .entry(track_name.to_string())
            .or_insert_with(|| KeyframeTrack::new(track_name.to_string()));
        
        track.add_keyframe(keyframe);
        
        // Atualizar duração total
        self.update_duration();
    }
    
    /// Adiciona múltiplos keyframes de uma vez
    pub fn add_keyframes(&mut self, track_name: &str, keyframes: Vec<Keyframe>) {
        for keyframe in keyframes {
            self.add_keyframe(track_name, keyframe);
        }
    }
    
    /// Atualiza a duração total baseada nas tracks
    fn update_duration(&mut self) {
        self.duration = self.tracks
            .values()
            .map(|track| track.duration())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
    }
    
    /// Define o modo de execução
    pub fn set_mode(&mut self, mode: AnimationMode) {
        self.mode = mode;
    }
    
    /// Define a velocidade de reprodução
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.max(0.0);
    }
    
    /// Define o delay inicial
    pub fn set_delay(&mut self, delay: f32) {
        self.delay = delay.max(0.0);
    }
    
    /// Inicia a animação
    pub fn play(&mut self) {
        if self.state == AnimationState::Idle {
            self.state = AnimationState::Playing;
            self.elapsed_time = 0.0;
            self.delay_elapsed = 0.0;
            self.repeat_count = 0;
            self.reverse = false;
            
            if let Some(callback) = &self.on_start {
                callback();
            }
        } else if self.state == AnimationState::Paused {
            self.state = AnimationState::Playing;
        }
    }
    
    /// Pausa a animação
    pub fn pause(&mut self) {
        if self.state == AnimationState::Playing {
            self.state = AnimationState::Paused;
        }
    }
    
    /// Para a animação e reseta
    pub fn stop(&mut self) {
        self.state = AnimationState::Idle;
        self.elapsed_time = 0.0;
        self.delay_elapsed = 0.0;
        self.repeat_count = 0;
        self.reverse = false;
    }
    
    /// Reinicia a animação do início
    pub fn restart(&mut self) {
        self.stop();
        self.play();
    }
    
    /// Atualiza a animação com o delta time
    pub fn update(&mut self, delta: f32) {
        if self.state != AnimationState::Playing {
            return;
        }
        
        // Processar delay inicial
        if self.delay_elapsed < self.delay {
            self.delay_elapsed += delta;
            if self.delay_elapsed < self.delay {
                return; // Ainda em delay
            }
            // Ajustar delta para o tempo restante após o delay
            let overflow = self.delay_elapsed - self.delay;
            self.update_animation(overflow);
        } else {
            self.update_animation(delta);
        }
    }
    
    /// Atualiza a animação propriamente dita
    fn update_animation(&mut self, delta: f32) {
        let adjusted_delta = delta * self.speed;
        
        if self.reverse {
            self.elapsed_time -= adjusted_delta;
        } else {
            self.elapsed_time += adjusted_delta;
        }
        
        // Verificar limites e processar modos
        match self.mode {
            AnimationMode::Once => {
                if self.elapsed_time >= self.duration {
                    self.elapsed_time = self.duration;
                    self.state = AnimationState::Finished;
                    if let Some(callback) = &self.on_complete {
                        callback();
                    }
                }
            }
            AnimationMode::Loop => {
                if self.elapsed_time >= self.duration {
                    self.elapsed_time = self.elapsed_time % self.duration;
                }
            }
            AnimationMode::PingPong => {
                if !self.reverse && self.elapsed_time >= self.duration {
                    self.elapsed_time = self.duration;
                    self.reverse = true;
                } else if self.reverse && self.elapsed_time <= 0.0 {
                    self.elapsed_time = 0.0;
                    self.reverse = false;
                    self.repeat_count += 1;
                }
            }
            AnimationMode::Repeat(count) => {
                if self.elapsed_time >= self.duration {
                    self.repeat_count += 1;
                    if self.repeat_count >= count {
                        self.elapsed_time = self.duration;
                        self.state = AnimationState::Finished;
                        if let Some(callback) = &self.on_complete {
                            callback();
                        }
                    } else {
                        self.elapsed_time = self.elapsed_time % self.duration;
                    }
                }
            }
        }
        
        self.elapsed_time = self.elapsed_time.clamp(0.0, self.duration);
    }
    
    /// Obtém os valores atuais de todas as propriedades animadas
    pub fn get_current_values(&self) -> HashMap<String, AnimatableProperty> {
        let mut values = HashMap::new();
        
        for (name, track) in &self.tracks {
            if let Some(value) = track.get_value_at(self.elapsed_time) {
                values.insert(name.clone(), value);
            }
        }
        
        values
    }
    
    /// Obtém o progresso da animação (0.0 a 1.0)
    pub fn progress(&self) -> f32 {
        if self.duration > 0.0 {
            (self.elapsed_time / self.duration).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
    
    /// Verifica se a animação está em execução
    pub fn is_playing(&self) -> bool {
        self.state == AnimationState::Playing
    }
    
    /// Verifica se a animação terminou
    pub fn is_finished(&self) -> bool {
        self.state == AnimationState::Finished
    }
    
    /// Define o callback de conclusão
    pub fn on_complete<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_complete = Some(Box::new(callback));
    }
    
    /// Define o callback de início
    pub fn on_start<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_start = Some(Box::new(callback));
    }
}