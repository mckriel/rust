use anyhow::Result;
use crossterm::{execute, terminal::{Clear, ClearType}};
use std::io::stdout;

// clear the console
pub fn clear_console() -> Result<()> {
    execute!(stdout(), Clear(ClearType::All))?;
    Ok(())
}
