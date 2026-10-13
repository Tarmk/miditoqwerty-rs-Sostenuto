# miditoqwerty-rs (Sostenuto fork)

[![Check](https://github.com/Tarmk/miditoqwerty-rs-Sostenuto/actions/workflows/check.yml/badge.svg)](https://github.com/Tarmk/miditoqwerty-rs-Sostenuto/actions/workflows/check.yml)

A fork of [ArijanJ/miditoqwerty-rs](https://github.com/ArijanJ/miditoqwerty-rs), the simple MIDI-input-to-QWERTY-output tool for macOS, Linux and Windows, with **sostenuto pedal support** added so the middle pedal on a digital piano does something useful in games that only read keyboard input.

## What this fork adds

- **Sostenuto pedal (MIDI CC 66).** While the middle pedal is down the app holds **`]`** (RightBracket) and releases it when the pedal comes up, the same way the sustain pedal holds Space. Works with the *Generic* and *Piano Visualizations* output methods; *Piano Rooms* ignores it because its protocol has no sostenuto.
- Unit tests for the pedal logic, and CI that builds and tests on Windows, macOS and Linux.

### Setting it up in a game

In the game's own key settings, bind:

| Game action | Key |
|---|---|
| Sostenuto Pedal | `]` RightBracket |
| Toggle Sostenuto | `[` LeftBracket |

The app only sends `]`, held for as long as the pedal is down. Putting the game's toggle on `[` keeps it next to the pedal key and away from the note keys.

The Windows build keeps its console window open, which logs every MIDI message and key it sends; handy when a pedal does not seem to work.

## Download

Windows build: https://github.com/Tarmk/miditoqwerty-rs-Sostenuto/releases (v1.0.0, `miditoqwerty-rs.exe`).

For macOS/Linux build from source (below) or use the upstream releases at https://github.com/ArijanJ/miditoqwerty-rs/releases if you do not need the pedal.

## Build from source

```bash
# Linux needs the ALSA headers: sudo apt install libasound2-dev
cargo build --release
cargo test            # pedal logic tests
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
