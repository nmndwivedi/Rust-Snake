use piston_window::types::Color;

pub fn make_color(color_string: &str, transparency: Option<f32>) -> Color {
    let r = u8::from_str_radix(&color_string[0..2], 16).unwrap() as f32 / 255.0;
    let g = u8::from_str_radix(&color_string[2..4], 16).unwrap() as f32 / 255.0;
    let b = u8::from_str_radix(&color_string[4..6], 16).unwrap() as f32 / 255.0;
    if let Some(t) = transparency {
        [r, g, b, t]
    } else {
        [r, g, b, 1.0]
    }
    
}