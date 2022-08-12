use crossbeam::channel::Receiver;
use std::io::Result;

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;

    loop {
        let buffer_size = stats_rx.recv().unwrap();

        total_bytes += buffer_size;

        if !silent {
            eprint!("\r{}", total_bytes);
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
