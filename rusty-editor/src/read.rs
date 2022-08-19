use crossterm::cursor::position;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::Result;
use std::time::Duration;

pub fn read_event() -> Result<()> {
    loop {
        let event = read()?;

        match_event(&event);

        if event == Event::Key(KeyCode::Char('c').into()) {
            println!("Cursor position: {:?}\r", position());
        }

        if let Event::Resize(x, y) = event {
            let (original_size, new_size) = flush_resize_events((x, y));
            println!("Resize from: {:?}, to: {:?}\r", original_size, new_size);
        }

        if event == Event::Key(KeyCode::Esc.into()) {
            break;
        }
    }

    Ok(())
}

// Resize events can occur in batches.
// With a simple loop they can be flushed.
// This function will keep the first and last resize event.
fn flush_resize_events(first_resize: (u16, u16)) -> ((u16, u16), (u16, u16)) {
    let mut last_resize = first_resize;
    while let Ok(true) = poll(Duration::from_millis(50)) {
        if let Ok(Event::Resize(x, y)) = read() {
            last_resize = (x, y);
        }
    }

    return (first_resize, last_resize);
}

fn match_event(read_event: &Event) {
    match read_event {
        // Match one one modifier:
        Event::Key(KeyEvent {
            modifiers: KeyModifiers::CONTROL,
            code,
            ..
        }) => {
            println!("Control + {:?}\r", code);
        }
        Event::Key(KeyEvent {
            modifiers: KeyModifiers::SHIFT,
            code,
            ..
        }) => {
            println!("Shift + {:?}\r", code);
        }
        Event::Key(KeyEvent {
            modifiers: KeyModifiers::ALT,
            code,
            ..
        }) => {
            println!("Alt + {:?}\r", code);
        }

        // Match on multiple modifiers:
        Event::Key(KeyEvent {
            code, modifiers, ..
        }) => {
            if *modifiers == (KeyModifiers::ALT | KeyModifiers::SHIFT) {
                println!("Alt + Shift {:?}\r", code);
            } else {
                println!("({:?}) with key: {:?}\r", modifiers, code);
            }
        }

        _ => {}
    }
}
