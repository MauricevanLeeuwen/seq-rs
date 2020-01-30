//#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
use portmidi::{self, MidiMessage};
use rand::prelude::*;
use std::time::Duration;

struct Cell {
    channel: u8,
    note: u8,
    active: bool,
}
impl Cell {
    fn new(note: u8) -> Cell {
        Cell {
            channel: 0,
            note: note,
            active: false,
        }
    }
    fn bang(&mut self) -> bool {
        false
    }
}

pub struct Instance {
    cells: Vec<Cell>,
    notes: Vec<(u8, u8)>,
    midi_out: portmidi::OutputPort,
    ctr: u32,
    pub tick: Duration,
}
pub fn new(bpm: f32, midi_out: portmidi::OutputPort) -> Instance {
    Instance {
        cells: vec![],
        notes: vec![],
        midi_out: midi_out,
        ctr: 0,
        tick: Duration::from_millis((60_000.0 / (bpm * 24.0)) as u64), // todo:  that f32->u64 is a bit hacky, just schedule ticks as events with a deadline.
    }
}
impl Instance {
    fn note_off_sustained(&mut self) {
        // turn off all sustained notes

        // TODO: use midi message to turn all notes off.
        while let Some((ch, i)) = self.notes.pop() {
            let event = MidiMessage {
                status: 0x80 + ch,
                data1: i, //note,
                data2: 0,
            };
            self.midi_out.write_message(event);
        }
    }

    pub fn tick(&mut self) {
        // With a tick rate of 1/24th quarter note (ie. 1/96th note)
        // 1/16 note events happen every 6 ticks.

        self.ctr = (self.ctr + 1) % 24;

        self.note_off_sustained();
        // note on
        let x = rand::random::<usize>() % self.cells.len();
        let x = &self.cells[x];

        let note: u8 = 60;
        let event = MidiMessage {
            status: 0x90 + x.channel,
            data1: x.note,
            data2: 64,
        };
        self.notes.push((x.channel, x.note));
        self.midi_out.write_message(event);
    }
}

pub fn from_string(input: String, mut midi_out: portmidi::OutputPort) -> Instance {
    let y = 12 * 3;
    Instance {
        cells: vec![Cell::new(y + 0)],
        ctr: 0,
        midi_out: midi_out,
        notes: vec![],
        tick: Duration::from_millis(1000 / 24), /* midi clock syncs at 1/24 of a quarter note */
    }
}
