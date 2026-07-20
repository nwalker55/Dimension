mod gui;

use crate::gui::main_menu::menu;

fn main() {
    if let Err(why) = menu() {
        eprintln!("Could not start main menu: {why}");
    }
}