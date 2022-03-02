#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::gui::SimpleFlasherApplication;

mod core;
mod gui;

fn main() {
    SimpleFlasherApplication::start();
}
