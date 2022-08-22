mod rainbow_spans_generator;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{fmt, io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans, Text},
    widgets::{
        canvas::{Canvas, Label, Line, Map, MapResolution, Points, Rectangle},
        Block, BorderType, Borders, Paragraph, Widget, Wrap,
    },
    Terminal,
};

use self::rainbow_spans_generator::{make_rainbow_spans, UsedChar};

use super::Pepe;

pub fn play_animation(pepe: Pepe) -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // let mut global_width: u16 = 0;
    let mut background_line: Vec<Span> = Vec::new();

    // let mut background_line: Vec<Span> =
    //     make_rainbow_spans(pepe_paragrapg_size.width, UsedChar::BraileEmptyChar);

    for _ in 0..100 {
        terminal.draw(|f| {
            let size = f.size();

            if background_line.is_empty() {
                //     global_width = size.width;
                background_line = make_rainbow_spans(size.width, UsedChar::BraileEmptyChar);
            } else {
                background_line.rotate_right(1);
            }
            // background_line.rotate_right(1);

            let block = Block::default()
                .title(pepe.name.to_uppercase())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                );

            let pepe_paragrapg_size = Rect::new(0, 0, pepe.get_width(), pepe.get_height());
            let pepe_paragraph = Paragraph::new(pepe.pepe)
                .style(
                    Style::default().add_modifier(Modifier::BOLD), //.fg(Color::Green), // .add_modifier(Modifier::BOLD),
                )
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });

            let pepe_canvas = Canvas::default()
                .x_bounds([
                    pepe_paragrapg_size.x as f64,
                    pepe_paragrapg_size.width as f64,
                ])
                .y_bounds([
                    pepe_paragrapg_size.y as f64,
                    pepe_paragrapg_size.height as f64,
                ])
                .paint(|ctx| {
                    let mut local_rainbow_spans = background_line.clone();
                    for line in 0..=pepe_paragrapg_size.height {
                        local_rainbow_spans.rotate_right(1);
                        ctx.print(0.0, line as f64, local_rainbow_spans.clone());
                    }
                });

                let render_width = match pepe_paragrapg_size.width > size.width {
                    true => size.width,
                    false => pepe_paragrapg_size.width,
                };
                let render_height = match pepe_paragrapg_size.height > size.height {
                    true => size.height,
                    false => pepe_paragrapg_size.height,
                };
                let render_area = Rect::new((size.width/2) - (render_width/2), (size.height/2) - (render_height/2), render_width, render_height);

            // f.render_widget(pepe_canvas, Rect{x:0, y:0, width: size.width/2, height: size.height/2});
            f.render_widget(pepe_canvas, render_area);
            f.render_widget(pepe_paragraph, render_area);
            // f.render_widget(background_paragraph, size);
            f.render_widget(block, render_area);
        })?;
        thread::sleep(Duration::from_millis(20));
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        // DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
