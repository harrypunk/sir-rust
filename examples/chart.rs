mod util;
use crate::util::event::{Event, Events};
use sir_rust::algorithm::sir::Euler;
use sir_rust::algorithm::{Coefficients, InitialValue, Method, SimResult};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    let init_val1 = InitialValue {
        n: 1000,
        s0: 999,
        i0: 10,
        r0: 2,
    };
    let euler = Euler {
        init_val: init_val1,
        rates: Coefficients {
            infection_rate: 0.3,
            recover_rate: 0.5,
        },
    };

    let days = 30;

    let result1: SimResult = euler.calc(days);
    let arr1 = result1
        .data
        .iter()
        .enumerate()
        .map(|(i, sir)| (i as f64, sir.0))
        .collect::<Vec<(f64, f64)>>();

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(size);

            let datasets = vec![Dataset::default()
                .name("Case numbers")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Yellow))
                .graph_type(GraphType::Line)
                .data(&arr1)];
            let chart = Chart::new(datasets)
                .block(
                    Block::default()
                        .title(Span::styled(
                            "Chart 1",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .title("X Axis")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, days as f64])
                        .labels(vec![
                            Span::styled("4/1", Style::default().add_modifier(Modifier::BOLD)),
                            Span::styled("4/3", Style::default().add_modifier(Modifier::BOLD)),
                        ]),
                )
                .y_axis(
                    Axis::default()
                        .title("Y Axis")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, 1.0])
                        .labels(vec![]),
                );
            f.render_widget(chart, chunks[1]);
        })?;

        match events.next()? {
            Event::Input(input) => {
                if input == Key::Char('q') {
                    break;
                }
            }
            Event::Tick => {}
        }
    }

    Ok(())
}
