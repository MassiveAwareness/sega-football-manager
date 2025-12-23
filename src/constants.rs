#![allow(dead_code)]
use macroquad::prelude::Color;

// Sega Genesis Paletta
pub const SEGA_BLUE: Color = Color::new(0.05, 0.05, 0.4, 1.0);      // Mélykék háttér
pub const SEGA_LIGHT_BLUE: Color = Color::new(0.2, 0.2, 0.7, 1.0);  // Kiemelés
pub const SEGA_YELLOW: Color = Color::new(0.9, 0.9, 0.2, 1.0);      // Kiemelt szöveg
pub const SEGA_WHITE: Color = Color::new(0.9, 0.9, 0.9, 1.0);       // Normál szöveg
pub const SEGA_GRAY: Color = Color::new(0.6, 0.6, 0.6, 1.0);        // Inaktív / Másodlagos szín
pub const SEGA_RED: Color = Color::new(0.9, 0.2, 0.2, 1.0);         // Hibaüzenetekhez
pub const BACKGROUND_COLOR: Color = Color::new(0.0, 0.0, 0.2, 1.0); // Háttérszín

// Virtuális felbontás (Kega Fusion style)
pub const VIRTUAL_WIDTH: f32 = 800.0;
pub const VIRTUAL_HEIGHT: f32 = 600.0;