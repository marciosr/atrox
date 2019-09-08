#![windows_subsystem = "windows"]
#[allow(unused)]

extern crate gtk;
use gtk::*;

mod atroxui;
mod calculos;

use crate::atroxui::AtroxUi;

fn main() {
	if gtk::init().is_err() {
    	println!("A inicialização do gtk falhou.");
    	return;
	}

	let ui = AtroxUi::new();

	ui.run(ui.clone());
	ui.window.show_all();


	gtk::main();
}
