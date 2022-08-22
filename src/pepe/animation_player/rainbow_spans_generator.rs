use tui::{
    style::{Color, Style},
    text::{Span, Spans},
};

const BRAILE_FULL_CHAR: char = '⣿';
const BRAILE_EMPTY_CHAR: char = '⠀';
const SPACE_CHAR: char = ' ';

const RGB_MAX: u8 = 255;
const RGB_MIN: u8 = 0;

pub enum UsedChar {
    BraileEmptyChar,
    BraileFullChar,
    SpaceChar,
}

fn calculate_rising(current: u16, width: u16) -> u8 {
    return (RGB_MAX as f32 * (current as f32 / width as f32)) as u8;
}

fn calculate_falling(current: u16, width: u16) -> u8 {
    return (RGB_MAX as f32 * (1.0 - (current as f32 / width as f32))) as u8;
}

fn make_span(content: char, color: (u8, u8, u8)) -> Span<'static> {
    return Span::styled(
        content.to_string(),
        Style::default().fg(Color::Rgb(color.0, color.1, color.2)),
    );
}

// pub fn make_rainbow_spans(width: u16) -> Spans<'static> {
pub fn make_rainbow_spans(width: u16, charUsed: UsedChar) -> Vec<Span<'static>> {
    let char_used = match charUsed {
        UsedChar::BraileEmptyChar => BRAILE_EMPTY_CHAR,
        UsedChar::BraileFullChar => BRAILE_FULL_CHAR,
        UsedChar::SpaceChar => SPACE_CHAR,
    };

    let segment_width = width / 6;

    let mut rainbow_spans: Vec<Span> = Vec::new();

    // red to yellow | #f00 to #ff0
    for x in 1..=segment_width {
        rainbow_spans.push(make_span(
            char_used,
            (RGB_MAX, calculate_rising(x, segment_width), RGB_MIN),
        ));
    }
    // yellow to green | #ff0 to #0f0
    for x in 1..=segment_width {
        rainbow_spans.push(make_span(
            char_used,
            (calculate_falling(x, segment_width), RGB_MAX, RGB_MIN),
        ));
    }
    // green to cyan | #0f0 to #0ff
    for x in 1..=segment_width {
        rainbow_spans.push(make_span(
            char_used,
            (RGB_MIN, RGB_MAX, calculate_rising(x, segment_width)),
        ));
    }
    // cyan to blue | #0ff to #00f
    for x in 1..=segment_width {
        rainbow_spans.push(make_span(
            char_used,
            (RGB_MIN, calculate_falling(x, segment_width), RGB_MAX),
        ));
    }
    // blue to violet | #00f to #f0f
    for x in 1..=segment_width {
        rainbow_spans.push(make_span(
            char_used,
            (calculate_rising(x, segment_width), RGB_MIN, RGB_MAX),
        ));
    }
    // violet to red | #f0f to #f00
    for x in 1..=segment_width {
        rainbow_spans.push(make_span(
            char_used,
            (RGB_MAX, RGB_MIN, calculate_falling(x, segment_width)),
        ));
    }

    // add remaining lengh (in red)
    for _ in 0..width - (6 * segment_width) {
        rainbow_spans.push(make_span(char_used, (RGB_MAX, RGB_MIN, RGB_MIN)));
    }

    return rainbow_spans;
}
