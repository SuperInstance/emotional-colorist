# emotional-colorist

> **Valence-based color mapping for agent emotional states**

[![crates.io](https://img.shields.io/crates/v/emotional-colorist.svg)](https://crates.io/crates/emotional-colorist)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Maps emotional states to RGB colors using a valence-arousal model. Enables ambient awareness of agent states through peripheral color displays in the TUI.

## The Valence-Arousal Model

Every emotion can be plotted in 2D:
- **Valence**: -1.0 (negative) to +1.0 (positive) → maps to hue
- **Arousal**: 0.0 (calm) to 1.0 (excited) → maps to saturation/brightness

This gives a continuous color wheel for emotional states:
- Red = negative + high arousal (angry, alert)
- Blue = negative + low arousal (sad, calm)
- Green = positive + low arousal (content, peaceful)
- Yellow = positive + high arousal (excited, happy)

## Installation

```toml
[dependencies]
emotional-colorist = "0.1.0"
```

## License

MIT © [SuperInstance](https://github.com/SuperInstance)

---

*Part of the [Exocortex](https://github.com/SuperInstance/exocortex) project — persistent cognitive substrate for multi-agent systems.*
