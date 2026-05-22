use ratatui::style::Color;
use serde_json::Value;
use std::fs;

pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub surface: Color,
    pub border_inactive: Color,
    pub border_active: Color,
    pub primary: Color,
    pub secondary: Color,
    pub text: Color,
    pub dim: Color,
    pub highlight: Color,
    pub selection: Color,
    pub brand: Color,
    pub error: Color,
    pub success: Color,
}

impl Theme{
    pub fn new() -> Self{
        let path = format!("{}/.local/state/scheme.json", std::env::var("HOME").unwrap_or_default());
        Self::from_json(&path)
    }

    pub fn from_json(path: &str) -> Self{
        let content = fs::read_to_string(path).unwrap_or_default();
        let json: Value = serde_json::from_str(&content).unwrap_or(Value::Null);
        let colors = if json.is_array(){
            &json[0]["colors"]
        }else{
            &json["colors"]
        };

        let get = |key: &str, fallback_hex: &str| -> Color{
            colors[key].as_str().map(|h| Self::parse_hex(h)).unwrap_or_else(|| Self::parse_hex(fallback_hex))
        };

        Self{
            bg: get("background", "#0D141B"),
            fg: get("onSurface", "#DCE3EE"),
            surface: get("surfaceContainer", "#192028"),
            border_inactive: get("outlineVariant", "#3F4853"),
            border_active: get("primary", "#9BCAFF"),
            primary: get("primary", "#9BCAFF"),
            secondary: get("secondary", "#B8C6EA"),
            text: get("onSurfaceVariant", "#BEC7D5"),
            dim: get("outline", "#89919E"),
            highlight: get("primaryContainer", "#004A7A"),
            selection: get("surfaceVariant", "#3F4853"),
            brand: get("brand", "#A0CBF4"),
            error: get("error", "#FBB3BC"),
            success: get("success", "#67DBB3"),
        }
    }

    fn parse_hex(hex: &str) ->Color{
        let hex = hex.trim_start_matches('#');
        if let Ok(rgb) = u32::from_str_radix(hex, 16){
            Color::Rgb(
                ((rgb >> 16) & 0xFF) as u8,
                ((rgb >> 8) & 0xFF) as u8,
                (rgb & 0xFF) as u8
            )
        }else{
            Color::Reset
        }
    }

    pub fn theme_refresh(&mut self){
        let path = format!("{}/.local/state/scheme.json", std::env::var("HOME").unwrap_or_default());
        let updated = Self::from_json(&path);
        *self = updated;
    }
}
