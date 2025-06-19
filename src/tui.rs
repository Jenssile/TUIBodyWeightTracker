use crate::data::{read_entries, write_entry};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    symbols,
    widgets::{Axis, Block, Borders, Chart, Dataset},
    Terminal,
};
use std::io::{self, stdout};

pub fn start_tui() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    res
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let entries = read_entries()?;
        let data: Vec<(f64, f64)> = entries
            .iter()
            .enumerate()
            .map(|(i, e)| (i as f64, e.weight as f64))
            .collect();

        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            let chart = Chart::new(vec![Dataset::default()
                .name("Weight")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Cyan))
                .data(&data)])
            .block(Block::default().title("Body Weight").borders(Borders::ALL))
            .x_axis(Axis::default().title("Days").bounds([0.0, data.len() as f64]))
            .y_axis(Axis::default().title("kg").bounds([50.0, 120.0]));

            f.render_widget(chart, chunks[0]);
        })?;

        if event::poll(std::time::Duration::from_millis(300))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('a') => {
                        disable_raw_mode()?;
                        println!("\nEnter weight: ");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        if let Ok(weight) = input.trim().parse::<f32>() {
                            write_entry(weight)?;
                        } else {
                            println!("Invalid weight input.");
                        }
                        enable_raw_mode()?;
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
