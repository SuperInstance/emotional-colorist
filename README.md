# emotional-colorist

> **Valence-based color mapping for agent emotional states — emotions become colors, moods become trajectories**

[![crates.io](https://img.shields.io/crates/v/emotional-colorist.svg)](https://crates.io/crates/emotional-colorist)
[![docs.rs](https://docs.rs/emotional-colorist/badge.svg)](https://docs.rs/emotional-colorist)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## What is Emotional Colorist?

Emotional Colorist maps emotional states to RGB colors using a **valence-arousal model** — the dominant framework in affective computing. Every emotion is a point in 2D space:

- **Valence** (x-axis): −1.0 (negative) to +1.0 (positive) — how pleasant the emotion is
- **Arousal** (y-axis): 0.0 (calm) to 1.0 (excited) — how activated the emotion is

Joy is at (+0.8, +0.8). Sadness is at (−0.7, +0.2). Anger is at (−0.6, +0.9). Calm is at (+0.5, +0.1).

The `Colorist` maps each (valence, arousal) pair to an RGB color: valence controls the hue (negative → red, positive → green), arousal controls brightness (calm → dim, excited → bright). A `ColorWheel` provides an alternative mapping around a 360° circle, and a `MoodTracker` records emotional trajectories over time.

## Why Does This Matter?

Color is the most intuitive visualization channel humans have. Mapping emotions to colors enables:

- **Ambient awareness**: A terminal background that shifts from red (agent struggling) to green (agent thriving) without requiring explicit monitoring
- **Mood dashboards**: Track agent wellbeing over time as a color trajectory — spot burnout or stagnation patterns early
- **TUI color fields**: Generate 2D ambient color fields that create a "mood atmosphere" in terminal interfaces
- **Emotion blending**: When two agents interact, blend their emotional colors to visualize the resulting state
- **Accessibility**: Color provides an at-a-glance summary that doesn't require reading text

Real-world applications:
- **Agent monitoring**: Watch a fleet of agents' emotional states as shifting colors on a dashboard
- **Conversational AI**: Map user sentiment in real-time to ambient lighting or UI color themes
- **Game design**: Generate dynamic color palettes from NPC emotional states
- **Meditation apps**: Track emotional trajectories during sessions, visualize progress as color paths

## Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                Emotional Colorist Pipeline                     │
│                                                              │
│  Emotion Space (Valence × Arousal)                            │
│  Arousal ▲                                                   │
│  1.0 ┤  Anger        Joy                                     │
│      │  (-0.6, 0.9)  (0.8, 0.8)                             │
│  0.5 ┤       Neutral                                          │
│      │       (0.0, 0.5)                                       │
│  0.0 ┤  Sadness       Calm                                   │
│      │  (-0.7, 0.2)  (0.5, 0.1)                              │
│      └────────────────────────────▶ Valence                  │
│     -1.0            0.0             1.0                      │
│                                                              │
│           │                    │                              │
│           ▼                    ▼                              │
│  ┌──────────────┐    ┌──────────────┐                        │
│  │  Colorist    │    │  ColorWheel  │                        │
│  │  V>0 → green │    │  0°→ red     │                        │
│  │  V<0 → red   │    │  90°→ yellow │                        │
│  │  V=0 → gray  │    │  180°→ green │                        │
│  │  A→brightness│    │  270°→ blue  │                        │
│  └──────┬───────┘    └──────┬───────┘                        │
│         │                   │                                 │
│         ▼                   ▼                                 │
│  ┌─────────────────────────────────┐                         │
│  │  Rgb Color   MoodTracker       │                         │
│  │  #28b464      trajectory over  │                         │
│  │  luminance    time → average   │                         │
│  └─────────────────────────────────┘                         │
│                                                              │
│  ┌─────────────────────────────────┐                         │
│  │  AmbientDisplay (TUI)          │                         │
│  │  Generate 2D color field with  │                         │
│  │  subtle variation around base  │                         │
│  │  emotion → ANSI escape codes   │                         │
│  └─────────────────────────────────┘                         │
└──────────────────────────────────────────────────────────────┘
```

## Quick Start

```rust
use emotional_colorist::{Emotion, Colorist, Rgb};

let colorist = Colorist::default();

// Map emotions to colors
let joy_color = colorist.map(&Emotion::joy());
let sad_color = colorist.map(&Emotion::sadness());
let angry_color = colorist.map(&Emotion::anger());
let calm_color = colorist.map(&Emotion::calm());

println!("Joy: {}     → luminance: {:.2}", joy_color.to_hex(), joy_color.luminance());
println!("Sad: {}  → luminance: {:.2}", sad_color.to_hex(), sad_color.luminance());
println!("Angry: {} → luminance: {:.2}", angry_color.to_hex(), angry_color.luminance());
```

### Custom Emotions and Blending

```rust
// Create any emotion in valence-arousal space
let focused = Emotion::new(0.6, 0.4);
let curious = Emotion::new(0.7, 0.7);

// Blend two emotions (0.0 = first, 1.0 = second)
let mix = focused.blend(&curious, 0.5);
println!("Blended: valence={:.2}, arousal={:.2}", mix.valence, mix.arousal);

// Distance between emotions
let dist = Emotion::joy().distance_to(&Emotion::sadness());
println!("Joy ↔ Sadness distance: {:.2}", dist);
```

### Mood Tracking Over Time

```rust
use emotional_colorist::{MoodTracker, Emotion};

let mut tracker = MoodTracker::new();
tracker.record(100, Emotion::calm());
tracker.record(200, Emotion::joy());
tracker.record(300, Emotion::new(0.3, 0.5));

// Current mood
println!("Current: valence={:.2}", tracker.current().unwrap().valence);

// Average mood over the trajectory
let avg = tracker.average().unwrap();
println!("Average: valence={:.2}, arousal={:.2}", avg.valence, avg.arousal);

// Full trajectory
for (ts, emotion) in tracker.trajectory() {
    println!("  t={}: ({:.2}, {:.2})", ts, emotion.valence, emotion.arousal);
}
```

### Color Wheel

```rust
use emotional_colorist::{ColorWheel, Emotion};

let wheel = ColorWheel::default();

// Get color at specific angles
let red = wheel.at_angle(0.0);
let yellow = wheel.at_angle(90.0);
let green = wheel.at_angle(180.0);
let blue = wheel.at_angle(270.0);

// Map emotion to angle
let emotion = Emotion::new(0.5, 0.7);
let angle = wheel.emotion_to_angle(&emotion);
let color = wheel.at_angle(angle);
println!("Emotion → {:.1}° → {}", angle, color.to_hex());
```

### Ambient Display for TUI

```rust
use emotional_colorist::{AmbientDisplay, Emotion};

let display = AmbientDisplay::new(40, 10);
let field = display.generate(&Emotion::calm());

// Render a row as ANSI background colors
for row in 0..10 {
    let line = display.render_row_ansi(&field, row);
    print!("{}\n", line);
}
```

## API Reference

### Rgb

| Method | Returns | Description |
|--------|---------|-------------|
| `Rgb::new(r, g, b)` | `Rgb` | Create color (0–255 per channel) |
| `color.to_hex()` | `String` | Hex string like `#ff8000` |
| `color.luminance()` | `f64` | Perceived brightness (0.0–1.0) |

### Emotion

| Method | Returns | Description |
|--------|---------|-------------|
| `Emotion::new(valence, arousal)` | `Emotion` | Create (clamped to valid ranges) |
| `Emotion::joy()` / `sadness()` / `anger()` / `calm()` / `neutral()` | `Emotion` | Preset emotions |
| `e.blend(other, t)` | `Emotion` | Linear blend (t=0→self, t=1→other) |
| `e.distance_to(other)` | `f64` | Euclidean distance in V-A space |

### Colorist

| Method | Returns | Description |
|--------|---------|-------------|
| `Colorist::default()` | `Colorist` | Red-negative, green-positive, gray-neutral |
| `Colorist::new(neg, pos, neutral)` | `Colorist` | Custom color mapping |
| `c.map(emotion)` | `Rgb` | Map emotion to color |

### ColorWheel

| Method | Returns | Description |
|--------|---------|-------------|
| `ColorWheel::default()` | `ColorWheel` | Red/Yellow/Green/Blue quarters |
| `wheel.at_angle(degrees)` | `Rgb` | Interpolated color at angle |
| `wheel.emotion_to_angle(emotion)` | `f64` | Map emotion to angle |

### MoodTracker

| Method | Returns | Description |
|--------|---------|-------------|
| `MoodTracker::new()` | `MoodTracker` | Create empty tracker |
| `t.record(timestamp, emotion)` | `()` | Record a mood |
| `t.current()` | `Option<&Emotion>` | Most recent emotion |
| `t.average()` | `Option<Emotion>` | Average over trajectory |
| `t.trajectory()` | `&[(u64, Emotion)]` | Full history |

### AmbientDisplay

| Method | Returns | Description |
|--------|---------|-------------|
| `AmbientDisplay::new(w, h)` | `AmbientDisplay` | Create display of given size |
| `d.generate(emotion)` | `Vec<Vec<Rgb>>` | 2D color field with variation |
| `d.render_row_ansi(field, row)` | `String` | ANSI escape codes for one row |

## Mathematical Background

### Circumplex Model of Affect

The valence-arousal model is based on James Russell's **circumplex model of affect** (1980). Every emotional state is a point in 2D space:

```
Arousal ↑
  1.0 │  Tense/Alert    Excited/Elated
      │  (Anger,Fear)   (Joy,Enthusiasm)
  0.5 │  Distressed     Happy
      │
  0.0 │  Depressed      Calm/Content
      └────────────────────────────→ Valence
     -1.0              0.0            1.0
```

### Color Mapping Function

The `Colorist` uses a three-point interpolation:
```
base = lerp(neutral, positive, |valence|)    if valence ≥ 0
base = lerp(neutral, negative, |valence|)    if valence < 0

final = base × (0.5 + 0.5 × arousal)
```

This ensures:
- Positive emotions → green tones (or custom positive color)
- Negative emotions → red tones (or custom negative color)
- High arousal → brighter, more saturated
- Low arousal → dimmer, more muted

### Luminance Calculation

Luminance follows the ITU-R BT.601 standard:
```
L = (0.299·R + 0.587·G + 0.114·B) / 255
```

This perceptual weighting accounts for the human eye's greater sensitivity to green light.

## Installation

```bash
cargo add emotional-colorist
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
emotional-colorist = "0.1.0"
```

## Related Crates

- [`constellation-map`](https://github.com/SuperInstance/constellation-map) — Fleet visualization as star charts
- [`memory-plimpsest`](https://github.com/SuperInstance/memory-plimpsest) — Layered memory with ghost traces
- [`knowledge-compass`](https://github.com/SuperInstance/knowledge-compass) — Provenance navigation for knowledge graphs
- [`cortex-toml`](https://github.com/SuperInstance/cortex-toml) — Configuration-as-code for Exocortex

## License

MIT © [SuperInstance](https://github.com/SuperInstance)

---

*Part of the [Exocortex](https://github.com/SuperInstance/exocortex) project — persistent cognitive substrate for multi-agent systems.*
