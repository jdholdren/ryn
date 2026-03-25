use crossterm::style::Color;
use serde::Deserialize;

pub struct Renderable {
    pub c: char,
    pub color: Color,
}

#[derive(Deserialize)]
pub struct RenderableData {
    #[serde(rename = "char")]
    pub c: char,
    pub color: String,
}

impl RenderableData {
    pub fn into_renderable(self) -> Renderable {
        Renderable {
            c: self.c,
            color: parse_color(&self.color),
        }
    }
}

fn parse_color(s: &str) -> Color {
    match s {
        "white" => Color::White,
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        "yellow" => Color::Yellow,
        "cyan" => Color::Cyan,
        "magenta" => Color::Magenta,
        "dark_grey" => Color::DarkGrey,
        "grey" => Color::Grey,
        "dark_red" => Color::DarkRed,
        "dark_green" => Color::DarkGreen,
        "dark_blue" => Color::DarkBlue,
        "dark_yellow" => Color::DarkYellow,
        "dark_cyan" => Color::DarkCyan,
        "dark_magenta" => Color::DarkMagenta,
        "black" => Color::Black,
        _ => Color::White,
    }
}
