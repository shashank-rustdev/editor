pub struct Reader;
use crossterm::event;
use crossterm::event::{Event, KeyEvent};
use std::time::Duration;

impl Reader {
    pub fn read_key(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}
