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
    cx.add_fonts_mem(&[fonts::ARCHIVO_BLACK]);
}
pub fn register_archivo_thin(cx: &mut Context) {
    cx.add_fonts_mem(&[fonts::ARCHIVO_THIN]);
}
pub fn register_archivo_narrow(cx: &mut Context) {
    cx.add_fonts_mem(&[fonts::ARCHIVO_NARROW]);
}
pub fn register_ibm_plex_mono(cx: &mut Context) {
    cx.add_fonts_mem(&[fonts::IBM_PLEX_MONO]);
}
pub fn register_atkinson_hyperlegible_regular(cx: &mut Context) {
    cx.add_fonts_mem(&[fonts::ATKINSON_HYPERLEGIBLE_REGULAR]);
}
