# Sequencer-rs
This is a programmable sequencer. The sequencer interfaces over MIDI, OSC or UDP. The application doesn't make sound on it's own, and it is not a synthesizer. The intended usage is live coding environments and modular synthesizers (using [CV-rs](https://github.com/MauricevanLeeuwen/cv-rs)).

## Quickstart

```
git clone https://github.com/MauricevanLeeuwen/seq-rs
cd seq-rs
cargo run
```

## Implementation
* PortMidi using [portmidi-rs](https://github.com/musitdev/portmidi-rs)
* Text-based UI using [Termion](https://gitlab.redox-os.org/redox-os/termion)

## Usage
```
q Quit
h cursor left
j cursor down
k cursor up
l cursor right
```


## Extra
Create control voltages for a modular setup: [CV-rs](https://github.com/MauricevanLeeuwen/cv-rs)
