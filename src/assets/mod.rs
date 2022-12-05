//! Binary assets for use with `nih_plug_vizia`. These fonts first need to be registered using their
//! associated registration function.

pub mod fonts;

use nih_plug_vizia::vizia::prelude::*;

pub const ARCHIVO_BLACK: &str = "Archivo Black";
pub const ARCHIVO_THIN: &str = "Archivo Thin";
pub const ARCHIVO_NARROW: &str = "Archivo Condensed";
pub const IBM_PLEX_MONO: &str = "IBM Plex Mono";
pub const ATKINSON_HYPERLEGIBLE_REGULAR: &str = "Atkinson Hyperlegible Regular";

pub fn register_archivo_black(cx: &mut Context) {
    cx.add_font_mem(ARCHIVO_BLACK, fonts::ARCHIVO_BLACK);
}
pub fn register_archivo_thin(cx: &mut Context) {
    cx.add_font_mem(ARCHIVO_THIN, fonts::ARCHIVO_THIN);
}
pub fn register_archivo_narrow(cx: &mut Context) {
    cx.add_font_mem(ARCHIVO_NARROW, fonts::ARCHIVO_NARROW);
}
pub fn register_ibm_plex_mono(cx: &mut Context) {
    cx.add_font_mem(IBM_PLEX_MONO, fonts::IBM_PLEX_MONO);
}
pub fn register_atkinson_hyperlegible_regular(cx: &mut Context) {
    cx.add_font_mem(ATKINSON_HYPERLEGIBLE_REGULAR, fonts::ATKINSON_HYPERLEGIBLE_REGULAR);
}