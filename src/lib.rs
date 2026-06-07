//! # emotional-colorist
//!
//! Valence-arousal based color mapping for emotional states.
//! Maps emotions to RGB colors, tracks mood trajectories, and generates
//! ambient color fields for TUI displays.
//!
//! ## Core Types
//! - [`Emotion`] — An emotional state defined by valence and arousal
//! - [`Colorist`] — Maps emotions to RGB colors using a valence model
//! - [`ColorWheel`] — Circular color space for emotional states
//! - [`MoodTracker`] — Tracks emotional trajectory over time
//! - [`AmbientDisplay`] — Generates peripheral color fields for TUI

/// An RGB color (0–255 per channel).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Convert to hex string.
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    /// Luminance (0.0–1.0).
    pub fn luminance(&self) -> f64 {
        (0.299 * self.r as f64 + 0.587 * self.g as f64 + 0.114 * self.b as f64) / 255.0
    }
}

/// An emotional state in 2D valence-arousal space.
#[derive(Debug, Clone, Copy)]
pub struct Emotion {
    /// Valence: -1.0 (negative) to +1.0 (positive).
    pub valence: f64,
    /// Arousal: 0.0 (calm) to 1.0 (excited).
    pub arousal: f64,
}

impl Emotion {
    /// Create a new emotion, clamping to valid ranges.
    pub fn new(valence: f64, arousal: f64) -> Self {
        Self {
            valence: valence.clamp(-1.0, 1.0),
            arousal: arousal.clamp(0.0, 1.0),
        }
    }

    /// Joy: high valence, high arousal.
    pub fn joy() -> Self {
        Self::new(0.8, 0.8)
    }

    /// Sadness: low valence, low arousal.
    pub fn sadness() -> Self {
        Self::new(-0.7, 0.2)
    }

    /// Anger: low valence, high arousal.
    pub fn anger() -> Self {
        Self::new(-0.6, 0.9)
    }

    /// Calm: moderate valence, low arousal.
    pub fn calm() -> Self {
        Self::new(0.5, 0.1)
    }

    /// Neutral emotion.
    pub fn neutral() -> Self {
        Self::new(0.0, 0.5)
    }

    /// Blend two emotions with a weight (0.0 = self, 1.0 = other).
    pub fn blend(&self, other: &Emotion, t: f64) -> Emotion {
        let t = t.clamp(0.0, 1.0);
        Emotion::new(
            self.valence * (1.0 - t) + other.valence * t,
            self.arousal * (1.0 - t) + other.arousal * t,
        )
    }

    /// Distance in valence-arousal space.
    pub fn distance_to(&self, other: &Emotion) -> f64 {
        let dv = self.valence - other.valence;
        let da = self.arousal - other.arousal;
        (dv * dv + da * da).sqrt()
    }
}

/// Maps emotions to RGB colors using a valence model.
#[derive(Debug, Clone)]
pub struct Colorist {
    /// Negative color (low valence).
    pub negative: Rgb,
    /// Positive color (high valence).
    pub positive: Rgb,
    /// Neutral color (zero valence).
    pub neutral: Rgb,
}

impl Default for Colorist {
    fn default() -> Self {
        Self {
            negative: Rgb::new(180, 40, 60),   // Deep red
            positive: Rgb::new(40, 180, 100),   // Green
            neutral: Rgb::new(140, 140, 160),   // Gray-blue
        }
    }
}

impl Colorist {
    /// Create a new colorist with custom color mapping.
    pub fn new(negative: Rgb, positive: Rgb, neutral: Rgb) -> Self {
        Self { negative, positive, neutral }
    }

    /// Map an emotion to an RGB color.
    pub fn map(&self, emotion: &Emotion) -> Rgb {
        let base = if emotion.valence >= 0.0 {
            lerp_color(&self.neutral, &self.positive, emotion.valence)
        } else {
            lerp_color(&self.neutral, &self.negative, -emotion.valence)
        };

        // Modulate brightness by arousal
        let factor = 0.5 + 0.5 * emotion.arousal;
        Rgb::new(
            (base.r as f64 * factor) as u8,
            (base.g as f64 * factor) as u8,
            (base.b as f64 * factor) as u8,
        )
    }
}

/// Circular color wheel for emotional states.
#[derive(Debug, Clone)]
pub struct ColorWheel {
    /// Colors at 0°, 90°, 180°, 270°.
    pub quarters: [Rgb; 4],
}

impl Default for ColorWheel {
    fn default() -> Self {
        Self {
            quarters: [
                Rgb::new(255, 80, 80),   // 0°   - anger/red
                Rgb::new(255, 220, 60),   // 90°  - joy/yellow
                Rgb::new(60, 200, 120),   // 180° - calm/green
                Rgb::new(60, 100, 255),   // 270° - sadness/blue
            ],
        }
    }
}

impl ColorWheel {
    /// Create a wheel with custom quarter colors.
    pub fn new(quarters: [Rgb; 4]) -> Self {
        Self { quarters }
    }

    /// Get color at a given angle (0–360°), interpolating between quarters.
    pub fn at_angle(&self, degrees: f64) -> Rgb {
        let deg = ((degrees % 360.0) + 360.0) % 360.0;
        let segment = deg / 90.0;
        let idx = segment.floor() as usize % 4;
        let next = (idx + 1) % 4;
        let t = segment - segment.floor();
        lerp_color(&self.quarters[idx], &self.quarters[next], t)
    }

    /// Map an emotion to a wheel angle (valence → angle, arousal → saturation).
    pub fn emotion_to_angle(&self, emotion: &Emotion) -> f64 {
        // Map valence to 0–180°, arousal shifts toward extremes
        let base = (emotion.valence + 1.0) / 2.0 * 180.0;
        let arousal_shift = (emotion.arousal - 0.5) * 90.0;
        base + arousal_shift
    }
}

/// Tracks emotional trajectory over time.
#[derive(Debug, Clone)]
pub struct MoodTracker {
    entries: Vec<(u64, Emotion)>,
}

impl Default for MoodTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl MoodTracker {
    /// Create an empty mood tracker.
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    /// Record an emotion at a timestamp.
    pub fn record(&mut self, timestamp: u64, emotion: Emotion) {
        self.entries.push((timestamp, emotion));
    }

    /// Get the most recent emotion.
    pub fn current(&self) -> Option<&Emotion> {
        self.entries.last().map(|(_, e)| e)
    }

    /// Get the emotional trajectory as a slice.
    pub fn trajectory(&self) -> &[(u64, Emotion)] {
        &self.entries
    }

    /// Compute the average emotion over the trajectory.
    pub fn average(&self) -> Option<Emotion> {
        if self.entries.is_empty() {
            return None;
        }
        let n = self.entries.len() as f64;
        let valence: f64 = self.entries.iter().map(|(_, e)| e.valence).sum::<f64>() / n;
        let arousal: f64 = self.entries.iter().map(|(_, e)| e.arousal).sum::<f64>() / n;
        Some(Emotion::new(valence, arousal))
    }

    /// Number of recorded moods.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if tracker is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Ambient display color field generator for TUI.
#[derive(Debug, Clone)]
pub struct AmbientDisplay {
    pub width: usize,
    pub height: usize,
}

impl AmbientDisplay {
    /// Create a new display with given dimensions.
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    /// Generate a color field based on a base emotion.
    /// Returns a 2D grid of Rgb colors with subtle variation.
    pub fn generate(&self, base: &Emotion) -> Vec<Vec<Rgb>> {
        let colorist = Colorist::default();
        let _base_color = colorist.map(base);

        (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(|x| {
                        let dx = (x as f64 / self.width as f64 - 0.5) * 0.2;
                        let dy = (y as f64 / self.height as f64 - 0.5) * 0.2;
                        let variation = Emotion::new(
                            base.valence + dx,
                            (base.arousal + dy).clamp(0.0, 1.0),
                        );
                        colorist.map(&variation)
                    })
                    .collect()
            })
            .collect()
    }

    /// Render a row of the color field as ANSI escape codes.
    pub fn render_row_ansi(&self, field: &[Vec<Rgb>], row: usize) -> String {
        if row >= field.len() {
            return String::new();
        }
        field[row]
            .iter()
            .map(|c| format!("\x1b[48;2;{};{};{}m \x1b[0m", c.r, c.g, c.b))
            .collect()
    }
}

/// Linear interpolation between two colors.
fn lerp_color(a: &Rgb, b: &Rgb, t: f64) -> Rgb {
    Rgb::new(
        (a.r as f64 + (b.r as f64 - a.r as f64) * t) as u8,
        (a.g as f64 + (b.g as f64 - a.g as f64) * t) as u8,
        (a.b as f64 + (b.b as f64 - a.b as f64) * t) as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_hex() {
        let c = Rgb::new(255, 128, 0);
        assert_eq!(c.to_hex(), "#ff8000");
    }

    #[test]
    fn test_rgb_luminance() {
        let white = Rgb::new(255, 255, 255);
        assert!((white.luminance() - 1.0).abs() < 0.01);
        let black = Rgb::new(0, 0, 0);
        assert!((black.luminance() - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_emotion_clamping() {
        let e = Emotion::new(5.0, -1.0);
        assert!((e.valence - 1.0).abs() < 1e-9);
        assert!((e.arousal - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_emotion_blend() {
        let a = Emotion::new(0.0, 0.0);
        let b = Emotion::new(1.0, 1.0);
        let mid = a.blend(&b, 0.5);
        assert!((mid.valence - 0.5).abs() < 1e-9);
        assert!((mid.arousal - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_emotion_distance() {
        let a = Emotion::new(0.0, 0.0);
        let b = Emotion::new(0.3, 0.4);
        let d = a.distance_to(&b);
        assert!((d - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_colorist_positive() {
        let colorist = Colorist::default();
        let color = colorist.map(&Emotion::joy());
        // Positive valence should lean green
        assert!(color.g > color.r);
    }

    #[test]
    fn test_colorist_negative() {
        let colorist = Colorist::default();
        let color = colorist.map(&Emotion::anger());
        // Negative valence should lean red
        assert!(color.r > color.g);
    }

    #[test]
    fn test_colorist_arousal_brightness() {
        let colorist = Colorist::default();
        let calm = colorist.map(&Emotion::new(0.5, 0.1));
        let excited = colorist.map(&Emotion::new(0.5, 0.9));
        let calm_lum = calm.r as f64 + calm.g as f64 + calm.b as f64;
        let excited_lum = excited.r as f64 + excited.g as f64 + excited.b as f64;
        assert!(excited_lum > calm_lum);
    }

    #[test]
    fn test_color_wheel_at_angle() {
        let wheel = ColorWheel::default();
        let at_zero = wheel.at_angle(0.0);
        assert_eq!(at_zero, wheel.quarters[0]);
    }

    #[test]
    fn test_color_wheel_wrap() {
        let wheel = ColorWheel::default();
        let at_360 = wheel.at_angle(360.0);
        let at_0 = wheel.at_angle(0.0);
        assert_eq!(at_360, at_0);
    }

    #[test]
    fn test_mood_tracker_record_and_current() {
        let mut tracker = MoodTracker::new();
        assert!(tracker.current().is_none());
        tracker.record(100, Emotion::joy());
        tracker.record(200, Emotion::sadness());
        assert_eq!(tracker.len(), 2);
        assert!((tracker.current().unwrap().valence - (-0.7)).abs() < 1e-9);
    }

    #[test]
    fn test_mood_tracker_average() {
        let mut tracker = MoodTracker::new();
        tracker.record(100, Emotion::new(0.0, 0.5));
        tracker.record(200, Emotion::new(1.0, 0.5));
        let avg = tracker.average().unwrap();
        assert!((avg.valence - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_ambient_display_generate() {
        let display = AmbientDisplay::new(4, 3);
        let field = display.generate(&Emotion::neutral());
        assert_eq!(field.len(), 3);
        assert_eq!(field[0].len(), 4);
    }

    #[test]
    fn test_ambient_display_ansi() {
        let display = AmbientDisplay::new(2, 1);
        let field = display.generate(&Emotion::calm());
        let ansi = display.render_row_ansi(&field, 0);
        assert!(ansi.contains("\x1b[48;2;"));
    }
}
