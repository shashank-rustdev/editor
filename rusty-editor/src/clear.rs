use crossterm::cursor;
use crossterm::{execute, terminal, terminal::ClearType};
use std::io::stdout;

// Clear Screen and reset the cursor position
pub fn clear_screen() -> crossterm::Result<()> {
    execute!(stdout(), terminal::Clear(ClearType::All))?;
    execute!(stdout(), cursor::MoveTo(0, 0))
}
