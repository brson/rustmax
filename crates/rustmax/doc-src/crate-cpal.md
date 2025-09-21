Low-level cross-platform audio I/O library.

- Crate [`::cpal`].
- [docs.rs](https://docs.rs/cpal)
- [crates.io](https://crates.io/crates/cpal)
- [GitHub](https://github.com/RustAudio/cpal)

---

CPAL (Cross-Platform Audio Library) is a low-level library for audio I/O.
It allows you to enumerate audio devices, get information about their
supported formats, and create input and output streams for real-time audio processing.

CPAL provides a cross-platform API that works on Windows, macOS, Linux,
iOS, Android, and WebAssembly, abstracting over the different audio APIs
on each platform (WASAPI, CoreAudio, ALSA, etc.).

Key features:
- Enumerate audio hosts and devices
- Query device capabilities and supported formats
- Create input streams for audio recording
- Create output streams for audio playback
- Real-time audio processing with low latency

## Examples

Enumerate available audio devices:

```rust
use cpal::traits::{DeviceTrait, HostTrait};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();

    println!("Available audio devices:");
    for device in host.devices()? {
        println!("  {}", device.name()?);
    }

    Ok(())
}
```

Play a simple sine wave:

```rust,ignore
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host.default_output_device()
        .ok_or("No output device available")?;

    let config = device.default_output_config()?;

    let mut sample_clock = 0f32;
    let sample_rate = config.sample_rate().0 as f32;
    let frequency = 440.0; // A4 note

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                *sample = (sample_clock * frequency * 2.0 * PI / sample_rate).sin();
                sample_clock = (sample_clock + 1.0) % sample_rate;
            }
        },
        |err| eprintln!("Audio error: {}", err),
        None,
    )?;

    stream.play()?;

    // Keep the stream alive
    std::thread::sleep(std::time::Duration::from_secs(3));

    Ok(())
}
```

## Platform Requirements

On Linux, CPAL requires ALSA development libraries:
- Debian/Ubuntu: `libasound2-dev`
- Fedora: `alsa-lib-devel`
- Arch Linux: `alsa-lib`

## Integration with Higher-Level Libraries

CPAL is often used as the audio backend for higher-level audio libraries like [`rodio`](crate::rodio).
For most audio playback use cases, consider using `rodio` which provides a more convenient API
built on top of CPAL.