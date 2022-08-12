use std::io::Result;
use std::sync::mpsc;

pub fn stats_loop(
    silent: bool,
    stats_rx: mpsc::Receiver<Vec<u8>>,
    write_tx: mpsc::Sender<Vec<u8>>,
) -> Result<()> {
    let mut total_bytes = 0;

    loop {
        let buffer = stats_rx.recv().unwrap();

        total_bytes += buffer.len();

        if !silent {
            eprint!("\r{}", total_bytes);
        }

        if buffer.is_empty() {
            break;
        }

        if write_tx.send(buffer).is_err() {
            break;
        }
    }

    if !silent {
        eprintln!();
    }
    Ok(())
}
