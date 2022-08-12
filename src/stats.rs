use crossbeam::channel::Receiver;
use std::io::Result;
use std::time::{Duration, Instant};

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;

    let start = Instant::now();
    let mut timer = Timer::new();

    loop {
        let buffer_size = stats_rx.recv().unwrap();

        timer.update();
        total_bytes += buffer_size;

        let rate = buffer_size as f64 / timer.delta.as_secs_f64();

        if !silent && timer.ready {
            timer.ready = false;
            eprint!(
                "\r{}, {}, {:.0}b/s",
                total_bytes,
                start.elapsed().as_secs().as_time(),
                rate
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

struct Timer {
    last_instant: Instant,
    delta: Duration,
    period: Duration,
    countdown: Duration,
    ready: bool,
}

impl Timer {
    fn new() -> Self {
        let now = Instant::now();

        Self {
            last_instant: now,
            delta: Duration::default(),
            period: Duration::from_millis(1000),
            countdown: Duration::default(),
            ready: true,
        }
    }

    fn update(&mut self) {
        let now = Instant::now();

        self.delta = now - self.last_instant;
        self.last_instant = now;

        self.countdown = self.countdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        });
    }
}
