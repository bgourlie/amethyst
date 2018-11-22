extern crate amethyst_assets;
extern crate amethyst_core;
extern crate glyph_brush;
extern crate font_kit;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde;

mod format;
mod font;

pub use self::{
    font::{
        default::get_default_font,
        systemfont::{default_system_font, get_all_font_handles, list_system_font_families},
    },
    format::{FontAsset, FontFormat, FontHandle, OtfFormat, TtfFormat}};
