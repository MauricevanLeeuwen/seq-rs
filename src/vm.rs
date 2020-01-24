//#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
use portmidi::{self, MidiMessage};

struct Cell {}

pub struct Instance {
    cells: Vec<Cell>,
    midi_out: portmidi::OutputPort,
    pub tick_rate: u8,
    i: u8,
}
struct Op {}

fn bang() {}
fn step(instance: Instance) -> Instance {
    instance
}
impl Instance {
    pub fn tick(&mut self) -> portmidi::Result<()> {
        static CHANNEL: u8 = 0;
        let note: u8 = 60;
        let event = MidiMessage {
            status: 0x90 + CHANNEL,
            data1: note,
            data2: 100,
        };
        // log that a note is on
        self.midi_out.write_message(event);

        use std::thread;
        use std::time::Duration;
        let t = Duration::from_millis(100);
        thread::sleep(t);

        let event = MidiMessage {
            status: 0x80 + CHANNEL,
            data1: note,
            data2: 100,
        };
        //try!(midi_out.write_message(event));
        self.i += 1;
        self.midi_out.write_message(event)
    }
}

pub fn from_string(input: String, mut midi_out: portmidi::OutputPort) -> Instance {
    Instance {
        cells: vec![],
        midi_out: midi_out,
        tick_rate: 4,
        i: 0,
    }
}
