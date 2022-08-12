mod timer;

use crossbeam::channel::Receiver;
use crossterm::{
    cursor, execute,
    style::{self, Color, PrintStyledContent},
    terminal::{Clear, ClearType},
};
use std::io::{self, Result, Stderr, Write};
use std::time::Instant;
use timer::Timer;

fn output_progress(stderr: &mut Stderr, bytes: usize, elapsed: String, rate: f64) {
    let bytes = style::style(format!("{} ", bytes)).with(Color::Red);
    let elapsed = style::style(elapsed).with(Color::Green);
    let rate = style::style(format!(" [{:.0}b/s]", rate)).with(Color::Blue);

    let _ = execute!(
        stderr,
        cursor::MoveToColumn(0),
        Clear(ClearType::CurrentLine),
        PrintStyledContent(bytes),
        PrintStyledContent(elapsed),
        PrintStyledContent(rate)
    );

    let _ = stderr.flush();
}

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;

    let start = Instant::now();
    let mut timer = Timer::new();
    let mut stderr = io::stderr();

    loop {
        let buffer_size = stats_rx.recv().unwrap();

        timer.update();
        total_bytes += buffer_size;

        let rate = buffer_size as f64 / timer.delta.as_secs_f64();

        if !silent && timer.ready {
            timer.ready = false;
            output_progress(
                &mut stderr,
                total_bytes,
                start.elapsed().as_secs().as_time(),
                rate,
            );
        }

        if buffer_size == 0 {
            break;
        }
    }

    if !silent {
        eprintln!();
    }

    Ok(())
}

trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (min, sec) = (left / 60, left % 60);
        format!("{}:{:02}:{02}", hours, min, sec)
    }
}
