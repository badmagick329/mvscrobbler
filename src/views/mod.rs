pub mod fzf_selector;
pub mod menu;
pub mod mv_selector;
pub mod updater;

use crossterm::{cursor, terminal, QueueableCommand};
use std::io::{stdout, Write};

fn clear_term(header: &str) -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    stdout
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?;
    write!(stdout, "{}", header)?;
    stdout.flush()
}
