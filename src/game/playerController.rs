use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{self as term, ClearType},
    ExecutableCommand,
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    cursor,
};
use std::io::{self, stdout, Write};
use colored::Styles::Clear;
use crate::game::*;

pub fn getInput() -> io::Result<(char, char)> {
    term::enable_raw_mode()?;
    let mut stdout = stdout();

    let (mut first, mut second, mut error) = (None, None, None);

    loop {
        queue!(
            stdout,
            term::Clear(ClearType::CurrentLine),
            cursor::MoveToColumn(0),
        )?;
        print!("\rEnter your move (1-3): ");

        if let Some(err) = &error {
            queue!(
                stdout,
                SetForegroundColor(Color::Red),
                Print(err),
                ResetColor,
            )?;
        } else {
            if let Some(f) = first {
                queue!(stdout, Print(f), Print(" x "))?;
                if let Some(s) = second {
                    queue!(stdout, Print(s))?;
                }
            }
        }

        stdout.flush()?;

        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind != KeyEventKind::Press { continue }

            if let Some(_) = error {
                if code == KeyCode::Enter {
                    error = None;
                }
                continue;
            }

            match code {
                KeyCode::Char(c @ '1') | KeyCode::Char(c @ '2') | KeyCode::Char(c @ '3') => {
                    if first.is_none() {
                        first = Some(c);
                    } else if second.is_none() {
                        second = Some(c);
                    }
                    print!("{} ", c);
                }
                KeyCode::Char(c @ 'q') => {
                    queue!(stdout, Print("Exiting...\n"))?;
                    stdout.flush()?;
                    term::disable_raw_mode()?;
                    std::process::exit(0);
                }
                KeyCode::Backspace => {
                    if second.is_some() {
                        second = None;
                    } else if first.is_some() {
                        first = None;
                    }
                }
                _ => {
                    if first.is_none() || second.is_none() {
                        error = Some("\rInvalid input. Press Enter to retry.".to_string());
                    }
                }
            }
        }

        if first.is_some() && second.is_some() { break }
    }

    term::disable_raw_mode()?;
    return Ok((first.unwrap(), second.unwrap()));
}

pub fn playerController(board: &mut Board) -> Point {
    loop {
        let (firstchar, secondchar) = getInput().expect("Error getting input");
        let first = (firstchar.to_digit(10).unwrap() as usize) - 1;
        let second = (secondchar.to_digit(10).unwrap() as usize) - 1;

        if board.get((first, second)) != ' ' {
            queue!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print("\rInvalid move. Press Enter to retry."),
                ResetColor,
            ).expect("Error setting foreground color.");

            io::stdout().flush().expect("Error flushing stdout.");

            term::enable_raw_mode().expect("Error setting terminal mode.");
            loop {
                if let Ok(true) = event::poll(Duration::from_millis(100)) {
                    if let Ok(Event::Key(event)) = event::read() {
                        if event.code == KeyCode::Enter { break }
                    }
                }
            }
            term::disable_raw_mode().expect("Error setting terminal mode.");
            continue;
        }

        return (first, second);
    }
}