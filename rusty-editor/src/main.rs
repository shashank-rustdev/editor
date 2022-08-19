mod clear;
use clear::*;
mod read;
use crossterm::cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition};
use crossterm::event::{
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::{
    event::{
        DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

use read::*;
use std::io::stdout;

fn main() -> Result<()> {
    let _ = clear_screen();
    println!("Welcome to, Rusty Editor!\n");
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(
        stdout,
        EnableBracketedPaste,
        EnableFocusChange,
        EnableMouseCapture,
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
        )
    )?;
    execute!(
        stdout,
        SavePosition,
        MoveTo(0, 0),
        EnableBlinking,
        DisableBlinking,
        RestorePosition
    )?;

    if let Err(e) = read_event() {
        println!("Error: {:?}\r", e);
    }

    execute!(
        stdout,
        DisableBracketedPaste,
        PopKeyboardEnhancementFlags,
        DisableFocusChange,
        DisableMouseCapture
    )?;

    disable_raw_mode()
}
