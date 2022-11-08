use std::{io::Write, time::Duration};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnableLineWrap,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand, QueueableCommand, Result,
};
use game::Grid;

mod game;

fn main() -> Result<()> {
    let mut grid = Grid::<20, 20>::new();

    grid.set_alive(0, 1);
    grid.set_alive(1, 2);
    grid.set_alive(2, 0);
    grid.set_alive(2, 1);
    grid.set_alive(2, 2);

    enable_raw_mode()?;

    let mut stdout = std::io::stdout();

    stdout
        .queue(EnterAlternateScreen)?
        .queue(SetForegroundColor(Color::Green))?
        .queue(DisableLineWrap)?
        .execute(Hide)?;

    loop {
        stdout
            .queue(Clear(ClearType::All))?
            .queue(MoveTo(0, 0))?
            .queue(Print(grid.to_string()))?
            .flush()?;

        if poll(Duration::from_millis(150))? {
            if let Event::Key(KeyEvent { code, .. }) = read()? {
                if code == KeyCode::Esc {
                    break;
                }
            }
        }

        grid.iterate();
    }

    execute!(
        stdout,
        ResetColor,
        EnableLineWrap,
        Show,
        LeaveAlternateScreen
    )?;

    disable_raw_mode()?;

    Ok(())
}
