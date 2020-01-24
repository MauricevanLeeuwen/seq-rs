extern crate termion;
use portmidi;
use termion::event::Key;
use termion::event::Key::Char;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

use std::thread;
use std::time::Duration;

use std::io::{self, Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;

mod vm;

#[derive(StructOpt, Debug)]
#[structopt(name = "Seq")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    config: Option<PathBuf>, /* config-rs */
    #[structopt(short, long)]
    debug: bool,
    #[structopt(short = "n", long, default_value = "1")]
    steps: u8,

    #[structopt(short, long, default_value = "60")]
    tempo: u8,

    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
}

struct Tui<R, W: Write> {
    stdout: W,
    stdin: R,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

fn tui_init<W: Write, R: Read>(mut stdout: W, stdin: R) {
    write!(stdout, "{}", clear::All).unwrap();
    write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

    let mut tui = Tui {
        stdin: stdin.keys(),
        stdout: stdout,
        x: 0,
        y: 0,
        width: 64,
        height: 16,
    };
    tui.start()
}
//
impl<R: Iterator<Item = Result<Key, std::io::Error>>, W: Write> Tui<R, W> {
    fn reset(&mut self) {}

    fn start(&mut self) {
        loop {
            let key = self.stdin.next().unwrap().unwrap();

            match key {
                Char('h') => self.x = self.left(self.x),
                Char('j') => self.y = self.down(self.y),
                Char('k') => self.y = self.up(self.y),
                Char('l') => self.x = self.right(self.x),
                Char('q') => return,
                Char(c) => {
                    write!(
                        self.stdout,
                        "{}{}{}",
                        color::Bg(color::LightBlue),
                        c,
                        color::Bg(color::Reset)
                    );
                }
                _ => {
                    write!(self.stdout, "{:?}", key);
                }
            }

            self.print_status();
            // Ensure cursor position is on current position
            let stride = 1;
            write!(self.stdout, "{}", cursor::Goto((self.x * stride) + 1, self.y + 1)).unwrap();
            self.stdout.flush().unwrap();
        }
    }

    fn left(&self, x: u16) -> u16 {
        if x == 0 {
            self.width - 1
        } else {
            x - 1
        }
    }
    fn right(&self, x: u16) -> u16 {
        if x + 1 < self.width {
            x + 1
        } else {
            0
        }
    }
    fn up(&self, y: u16) -> u16 {
        if y == 0 {
            self.height - 1
        } else {
            y - 1
        }
    }
    fn down(&self, y: u16) -> u16 {
        if y + 1 < self.height {
            y + 1
        } else {
            0
        }
    }

    fn print_status(&mut self) {
        write!(self.stdout, "{}", cursor::Goto(1, self.height + 1)).unwrap();
        write!(self.stdout, "{}{}x{}\r\n", clear::CurrentLine, self.width, self.height).unwrap();
        write!(self.stdout, "{}{},{}\r\n", clear::CurrentLine, self.x, self.y).unwrap();
    }
}

fn main() {
    let opt = Opt::from_args();

    // lock stdio
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stderr = io::stderr();
    let mut stderr = stderr.lock();

    let mut input = String::new();
    // Initialize portmidi
    let pm_context = portmidi::PortMidi::new().unwrap();
    for res in pm_context.devices().unwrap() {
        println!("{:#?}", res)
    }
    let midi_out = pm_context.default_output_port(64).unwrap();

    let mut stdout = stdout.into_raw_mode().unwrap();
    //let termsize = termion::terminal_size().ok();

    let mut instance = vm::from_string(input, midi_out);
    let h = thread::spawn(move || loop {
        instance.tick();
        let next = Duration::from_millis(1000 / (instance.tick_rate as u64));
        thread::park_timeout(next);
    });

    tui_init(stdout, stdin);

    //Ok(())
}
