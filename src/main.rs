#![windows_subsystem = "windows"]
#[allow(unused)]

extern crate gtk;

use gtk::prelude::*;
use gtk::{ Application, glib };
use glib::ExitCode;


#[macro_use]
mod utils;
mod atroxui;
mod calculos;

use crate::atroxui::AtroxUi;

fn main() -> ExitCode {

	let application = Application::new(Some("com.github.marciosr.atrox"),
		Default::default());

	application.connect_activate(move|app|{
		let atrox = AtroxUi::new();
		let window = atrox.window.clone();
		app.add_window(&window);
		atrox.run(atrox.clone());
		window.show();
	});

    application.run()
}
