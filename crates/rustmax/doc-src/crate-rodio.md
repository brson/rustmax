High-level audio playback library.

- Crate [`::rodio`].
- [docs.rs](https://docs.rs/rodio)
- [crates.io](https://crates.io/crates/rodio)
- [GitHub](https://github.com/RustAudio/rodio)

---

Rodio is a high-level audio playback library built on top of [`cpal`](crate::cpal).
It provides an easy-to-use API for playing audio files and generating audio.
Rodio handles format decoding, audio mixing, and provides convenient abstractions
for common audio operations.

Key features:
- Play audio files (WAV, MP3, FLAC, Vorbis, etc.)
- Audio synthesis (sine waves, noise, etc.)
- Volume control and audio effects
- Audio mixing and streaming
- Spatial audio positioning
- Built on [`cpal`](crate::cpal) for cross-platform support

Format support is provided by Symphonia by default, with optional
format-specific decoders available as features.

Rodio uses CPAL as its audio backend.


## Examples

Play an audio file:

```rust
// fixme
```
