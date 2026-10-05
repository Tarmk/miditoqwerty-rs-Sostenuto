# miditoqwerty-rs (Sostenuto fork)

A fork of [ArijanJ/miditoqwerty-rs](https://github.com/ArijanJ/miditoqwerty-rs), the simple MIDI-input-to-QWERTY-output tool for macOS, Linux and Windows, with **sostenuto pedal support** added so the middle pedal on a digital piano does something useful in games that only read keyboard input.

## What this fork adds

- **Sostenuto pedal (MIDI CC 66)** is translated into keyboard events, next to the existing sustain handling.
- Two new binds in *Settings → Binds*:
  - `Toggle Sostenuto` – default **LeftBracket** `[`
  - `Sostenuto Pedal` – default **RightBracket** `]`
- A console window on Windows so you can see what is being sent.

Map the same two keys inside the game you are playing and the pedal just works.

## Download

Windows build: https://github.com/Tarmk/miditoqwerty-rs-Sostenuto/releases (v1.0.0, `miditoqwerty-rs.exe`).

For macOS/Linux build from source (below) or use the upstream releases at https://github.com/ArijanJ/miditoqwerty-rs/releases if you do not need the pedal.

## Build from source

```bash
# Linux needs the ALSA headers: sudo apt install libasound2-dev
cargo build --release
./target/release/miditoqwerty-rs
```

A current stable Rust toolchain (`rustup`) is the only requirement on Windows and macOS.

## Troubleshooting on macOS

If notes aren't being played but you are [sure your piano works](https://hardwaretester.com/midi), it's probably a permission issue. The app needs Accessibility permission to send keystrokes.

To reset all permissions after running a new version:
`sudo tccutil reset All dev.arijanj.miditoqwerty`

Then restart the app and switch "Output Method" to a different one so the permission prompt comes back up.

## Alternatives

- [Windows] https://github.com/ArijanJ/miditoqwerty – predecessor to this app, more customisable
- [Windows] https://github.com/Zephkek/MIDIPlusPlus – has a MIDI autoplayer

## License

**MIT**, same as upstream. Thanks to Arijan for the original project.
