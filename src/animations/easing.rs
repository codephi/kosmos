use bevy::prelude::*;

/// Tipos de funções de easing para interpolação suave
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,
}

impl Easing {
    /// Aplica a função de easing ao valor t (0.0 a 1.0)
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        
        match self {
            Easing::Linear => t,
            
            // Quadratic
            Easing::EaseIn | Easing::EaseInQuad => t * t,
            Easing::EaseOut | Easing::EaseOutQuad => t * (2.0 - t),
            Easing::EaseInOut | Easing::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            
            // Cubic
            Easing::EaseInCubic => t * t * t,
            Easing::EaseOutCubic => {
                let t = t - 1.0;
                t * t * t + 1.0
            }
            Easing::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let t = 2.0 * t - 2.0;
                    1.0 + t * t * t / 2.0
                }
            }
            
            // Quartic
            Easing::EaseInQuart => t * t * t * t,
            Easing::EaseOutQuart => {
                let t = t - 1.0;
                1.0 - t * t * t * t
            }
            Easing::EaseInOutQuart => {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    let t = t - 1.0;
                    1.0 - 8.0 * t * t * t * t
                }
            }
            
            // Elastic
            Easing::EaseInElastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let p = 0.3;
                    let s = p / 4.0;
                    let t = t - 1.0;
                    -(2.0_f32.powf(10.0 * t) * ((t - s) * (2.0 * std::f32::consts::PI) / p).sin())
                }
            }
            Easing::EaseOutElastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let p = 0.3;
                    let s = p / 4.0;
                    2.0_f32.powf(-10.0 * t) * ((t - s) * (2.0 * std::f32::consts::PI) / p).sin() + 1.0
                }
            }
            Easing::EaseInOutElastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let p = 0.3 * 1.5;
                    let s = p / 4.0;
                    let t = 2.0 * t;
                    if t < 1.0 {
                        let t = t - 1.0;
                        -0.5 * (2.0_f32.powf(10.0 * t) * ((t - s) * (2.0 * std::f32::consts::PI) / p).sin())
                    } else {
                        let t = t - 1.0;
                        2.0_f32.powf(-10.0 * t) * ((t - s) * (2.0 * std::f32::consts::PI) / p).sin() * 0.5 + 1.0
                    }
                }
            }
            
            // Bounce
            Easing::EaseInBounce => 1.0 - Easing::EaseOutBounce.apply(1.0 - t),
            Easing::EaseOutBounce => {
                if t < 1.0 / 2.75 {
                    7.5625 * t * t
                } else if t < 2.0 / 2.75 {
                    let t = t - 1.5 / 2.75;
                    7.5625 * t * t + 0.75
                } else if t < 2.5 / 2.75 {
                    let t = t - 2.25 / 2.75;
                    7.5625 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / 2.75;
                    7.5625 * t * t + 0.984375
                }
            }
            Easing::EaseInOutBounce => {
                if t < 0.5 {
                    Easing::EaseInBounce.apply(t * 2.0) * 0.5
                } else {
                    Easing::EaseOutBounce.apply(t * 2.0 - 1.0) * 0.5 + 0.5
                }
            }
            
            // Back
            Easing::EaseInBack => {
                let s = 1.70158;
                t * t * ((s + 1.0) * t - s)
            }
            Easing::EaseOutBack => {
                let s = 1.70158;
                let t = t - 1.0;
                t * t * ((s + 1.0) * t + s) + 1.0
            }
            Easing::EaseInOutBack => {
                let s = 1.70158 * 1.525;
                let t = t * 2.0;
                if t < 1.0 {
                    0.5 * (t * t * ((s + 1.0) * t - s))
                } else {
                    let t = t - 2.0;
                    0.5 * (t * t * ((s + 1.0) * t + s) + 2.0)
                }
            }
            
            // Circular
            Easing::EaseInCirc => 1.0 - (1.0 - t * t).sqrt(),
            Easing::EaseOutCirc => {
                let t = t - 1.0;
                (1.0 - t * t).sqrt()
            }
            Easing::EaseInOutCirc => {
                let t = t * 2.0;
                if t < 1.0 {
                    -0.5 * ((1.0 - t * t).sqrt() - 1.0)
                } else {
                    let t = t - 2.0;
                    0.5 * ((1.0 - t * t).sqrt() + 1.0)
                }
            }
        }
    }
}

/// Interpola entre dois valores usando a função de easing
pub fn lerp_with_easing<T>(start: T, end: T, t: f32, easing: Easing) -> T 
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<f32, Output = T> + Copy,
{
    let t = easing.apply(t);
    start + (end - start) * t
}