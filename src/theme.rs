use ratatui::style::{Color};

pub struct Theme{
    pub bg: Color, //main background
    pub fg: Color, //main foreground
    pub surface: Color, //secondary areas (sidebar, headers)

    pub border_inactive: Color, //inactive/rest color
    pub border_active: Color, //active border color (focused)

    pub primary: Color, //Main titles, buttons, etc
    pub secondary: Color, // secondary areas
    pub text: Color, //regular text body
    pub dim: Color, //secondary text, footers, hits, comments

    pub highlight: Color, //highlighter to text and other items
    pub selection: Color, //list selection background
    pub brand: Color, // one specific brand color

    pub error: Color, //error
    pub success: Color //success
}

//FIXME: integrate wallwatch to change this file theme palette
pub const PALETTE: Theme = Theme{
    bg: Color::Reset,
    fg: Color::Rgb(220, 220, 220),
    surface: Color::Indexed(234),

    border_inactive: Color::Indexed(240),
    border_active: Color::Rgb(100, 149, 237),

    primary: Color::Rgb(255, 165, 0),
    secondary: Color::Cyan,
    text: Color::Rgb(200, 200, 200),
    dim: Color::Indexed(243),

    highlight: Color::Rgb(156, 213, 255),
    selection: Color::Indexed(237),
    brand: Color::Rgb(138, 43, 226),

    error: Color::Rgb(255, 95, 95),
    success: Color::Rgb(95, 255, 95)
};

