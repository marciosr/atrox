#[allow(unused)]
//use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Builder, Button, Label, HeaderBar, ApplicationWindow, Entry, SpinButton, CheckButton, Adjustment, glib};
use std::rc::Rc;
//use glib::{clone};

use crate::calculos::Calculos;

#[derive(Copy)]
pub struct ValorRadios {
	pub dec: usize,
	pub cob: usize,
	pub sol: usize,
	pub ppt: usize,
	pub tct: usize,
	pub tte: usize,
	pub tex: usize,
	pub ero: usize,
	pub can: usize,
	pub fmt: usize,
}

impl Clone for ValorRadios {
	fn clone(&self) -> ValorRadios { *self }
}

pub struct AtroxUi {
	pub builder: Builder,
	pub window: ApplicationWindow,
	pub header_bar: HeaderBar,
	pub bt_calcular: Button,
	pub bt_calcula2: Button,
	pub spin_area_bacia: 	SpinButton,
	pub spin_largura_bacia: SpinButton,
	pub spin_dec_talhao: 	SpinButton,
	pub spin_dec_canal: 	SpinButton,
	pub spin_coef_atrito: 	SpinButton,
	pub spin_profundidade: 	SpinButton,
	pub spin_retorno: 		SpinButton,
	pub entry_tempo_conc:			Entry,
	//pub entry_tempo_conc_corrigido:	Entry,
	pub entry_coef_escorrimento:	Entry,
	pub entry_intensidade_max:		Entry,
	pub entry_vazao_bacia:			Entry,
	pub entry_volume_enxurrada:		Entry,
	pub entry_esp_h:	Entry,
	pub entry_esp_v:	Entry,
	pub entry_sessao:	Entry,
	pub entry_vel_max:	Entry,
	//pub entry_h:		Entry,
	pub entry_bb:		Entry,
	pub entry_b:		Entry,
	pub entry_y:		Entry,
	pub entry_pm:		Entry,
	pub label_critica: 	Label,
	pub adjustment_area: Adjustment,
	pub check_dec1: 		CheckButton,
	pub check_dec2: 		CheckButton,
	pub check_dec3: 		CheckButton,
	pub check_dec4: 		CheckButton,
	pub check_dec5: 		CheckButton,
	pub check_dec6: 		CheckButton,
	pub check_cob1: 		CheckButton,
	pub check_cob2: 		CheckButton,
	pub check_cob3: 		CheckButton,
	pub check_cob4: 		CheckButton,
	pub check_cob5: 		CheckButton,
	pub check_sol1: 		CheckButton,
	pub check_sol2: 		CheckButton,
	pub check_sol3: 		CheckButton,
	pub check_ppt1: 		CheckButton,
	pub check_ppt2: 		CheckButton,
	pub check_cultivo1: 	CheckButton,
	pub check_cultivo2: 	CheckButton,
	pub check_terraco1: 	CheckButton,
	pub check_terraco2: 	CheckButton,
	pub check_textura1: 	CheckButton,
	pub check_textura2: 	CheckButton,
	pub check_textura3: 	CheckButton,
	pub check_erod1: 	CheckButton,
	pub check_erod2: 	CheckButton,
	pub check_cob_canal1: CheckButton,
	pub check_cob_canal2: CheckButton,
	pub check_cob_canal3: CheckButton,
	pub check_cob_canal4: CheckButton,
	pub check_cob_canal5: CheckButton,
	pub check_cob_canal6: CheckButton,
	pub check_cob_canal7: CheckButton,
	pub check_cob_canal8: CheckButton,
	pub check_formato1: 	CheckButton,
	pub check_formato2: 	CheckButton,
}

impl AtroxUi {
	pub fn new() -> Rc<Self> {

		let glade_src = include_str!("atrox.ui");
		let builder = gtk::Builder::from_string(glade_src);

		get_widget!(builder, ApplicationWindow, window);
		get_widget!(builder, HeaderBar, header_bar);
		get_widget!(builder, Button, bt_calcular);
		get_widget!(builder, Button, bt_calcula2);
		get_widget!(builder, SpinButton,spin_area_bacia);
		get_widget!(builder, SpinButton,spin_largura_bacia);
		get_widget!(builder, SpinButton,spin_dec_talhao);
		get_widget!(builder, SpinButton,spin_dec_canal);
		get_widget!(builder, SpinButton,spin_coef_atrito);
		get_widget!(builder, SpinButton,spin_profundidade);
		get_widget!(builder, SpinButton,spin_retorno);
		get_widget!(builder, Entry,entry_tempo_conc);
		get_widget!(builder, Entry,entry_coef_escorrimento);
		get_widget!(builder, Entry,entry_intensidade_max);
		get_widget!(builder, Entry,entry_vazao_bacia);
		get_widget!(builder, Entry,entry_volume_enxurrada);
		get_widget!(builder, Entry,entry_esp_h);
		get_widget!(builder, Entry,entry_esp_v);
		get_widget!(builder, Entry,entry_sessao);
		get_widget!(builder, Entry,entry_vel_max);
		get_widget!(builder, Entry,entry_bb);
		get_widget!(builder, Entry,entry_b);
		get_widget!(builder, Entry,entry_y);
		get_widget!(builder, Entry,entry_pm);
		get_widget!(builder, Label,label_critica);
		get_widget!(builder,Adjustment,adjustment_area);

		get_widget!(builder,CheckButton,check_dec1);
		get_widget!(builder,CheckButton,check_dec2);
		get_widget!(builder,CheckButton,check_dec3);
		get_widget!(builder,CheckButton,check_dec4);
		get_widget!(builder,CheckButton,check_dec5);
		get_widget!(builder,CheckButton,check_dec6);
		get_widget!(builder,CheckButton,check_cob1);
		get_widget!(builder,CheckButton,check_cob2);
		get_widget!(builder,CheckButton,check_cob3);
		get_widget!(builder,CheckButton,check_cob4);
		get_widget!(builder,CheckButton,check_cob5);
		get_widget!(builder,CheckButton,check_sol1);
		get_widget!(builder,CheckButton,check_sol2);
		get_widget!(builder,CheckButton,check_sol3);
		get_widget!(builder,CheckButton,check_ppt1);
		get_widget!(builder,CheckButton,check_ppt2);
		get_widget!(builder,CheckButton,check_cultivo1);
		get_widget!(builder,CheckButton,check_cultivo2);
		get_widget!(builder,CheckButton,check_terraco1);
		get_widget!(builder,CheckButton,check_terraco2);
		get_widget!(builder,CheckButton,check_textura1);
		get_widget!(builder,CheckButton,check_textura2);
		get_widget!(builder,CheckButton,check_textura3);
		get_widget!(builder,CheckButton,check_erod1);
		get_widget!(builder,CheckButton,check_erod2);
		get_widget!(builder,CheckButton,check_cob_canal1);
		get_widget!(builder,CheckButton,check_cob_canal2);
		get_widget!(builder,CheckButton,check_cob_canal3);
		get_widget!(builder,CheckButton,check_cob_canal4);
		get_widget!(builder,CheckButton,check_cob_canal5);
		get_widget!(builder,CheckButton,check_cob_canal6);
		get_widget!(builder,CheckButton,check_cob_canal7);
		get_widget!(builder,CheckButton,check_cob_canal8);
		get_widget!(builder,CheckButton,check_formato1);
		get_widget!(builder,CheckButton,check_formato2);

		//let window_clone = window.clone();
		// window.connect_delete_event(clone!(@strong window => move |_,_| {
			//window.destroy();
		// 	gtk::main_quit();
  //   	Inhibit(true)
		// }));

		let atroxui = Rc::new(Self { builder, window, header_bar, bt_calcular, bt_calcula2, spin_area_bacia,
				  spin_largura_bacia, spin_dec_talhao, spin_dec_canal, spin_coef_atrito,
				  spin_profundidade, spin_retorno, entry_tempo_conc,
				  entry_coef_escorrimento, entry_intensidade_max, entry_vazao_bacia,
				  entry_volume_enxurrada, entry_esp_h, entry_esp_v, entry_sessao,
				  entry_vel_max, entry_bb, entry_b, entry_y, entry_pm, label_critica,
				  adjustment_area, check_dec1, check_dec2, check_dec3, check_dec4, check_dec5, check_dec6,
				  check_cob1, check_cob2, check_cob3, check_cob4, check_cob5, check_sol1, check_sol2, check_sol3,
				  check_ppt1, check_ppt2, check_cultivo1, check_cultivo2, check_terraco1, check_terraco2,
				  check_textura1, check_textura2, check_textura3, check_erod1, check_erod2, check_cob_canal1,
				  check_cob_canal2, check_cob_canal3, check_cob_canal4, check_cob_canal5, check_cob_canal6,
				  check_cob_canal7, check_cob_canal8, check_formato1, check_formato2
		});
		atroxui
	}

	pub fn run(&self, ui: Rc<AtroxUi>) {

		let ui2 = ui.clone();

		let spin_area_bacia = ui.spin_area_bacia.clone();
		let spin_largura_bacia = ui.spin_largura_bacia.clone();
		let spin_dec_talhao = ui.spin_dec_talhao.clone();
		let spin_dec_canal = ui.spin_dec_canal.clone();
		let spin_coef_atrito = ui.spin_coef_atrito.clone();
		let spin_profundidade = ui.spin_profundidade.clone();
		let spin_retorno = ui.spin_retorno.clone();

		ui2.bt_calcular.connect_clicked(move |_| {

			let valor = radio_valores(ui.clone());

			let calculos = Calculos::calcula_terraco(valor,
													spin_area_bacia.value(),
													spin_largura_bacia.value(),
													spin_dec_talhao.value(),
													spin_dec_canal.value(),
													spin_coef_atrito.value(),
													spin_profundidade.value(),
													spin_retorno.value());
			let temp_concent = calculos.tempo_concent;
			let coef_escorrimento = calculos.coef_escorrimento;
			let intens_max_chuva = calculos.intens_max_chuva;
			let vazao_max = calculos.vazao_max;
			let volume_enxurrada = calculos.volume_enxurrada;
			let esp_horiz = calculos.esp_horiz;
			let esp_vert = calculos.esp_vert;
			let sessao = calculos.sessao;
			//let vel_maxima = calculos.vel_maxima;
			let b_maior = calculos.b_maior;
			let b_menor = calculos.b_menor;
			let talude_y = calculos.talude_y;
			let per_molhado = calculos.per_molhado;
			let vel_canal = calculos.vel_canal;
			println!("Velocidade do canal {} m/min", vel_canal);


			ui.entry_tempo_conc.set_text(&temp_concent.to_string());
			ui.entry_coef_escorrimento.set_text(&coef_escorrimento.to_string());
			ui.entry_intensidade_max.set_text(&intens_max_chuva.to_string());
			ui.entry_vazao_bacia.set_text(&vazao_max.to_string());
			ui.entry_volume_enxurrada.set_text(&volume_enxurrada.to_string());
			ui.entry_esp_h.set_text(&esp_horiz.to_string());
			ui.entry_esp_v.set_text(&esp_vert.to_string());
			ui.entry_sessao.set_text(&sessao.to_string());
			ui.entry_bb.set_text(&b_maior.to_string());
			ui.entry_b.set_text(&b_menor.to_string());
			ui.entry_y.set_text(&talude_y.to_string());
			ui.entry_pm.set_text(&per_molhado.to_string());
		});

		self.window.connect_close_request(move |win| {
			win.destroy();
			glib::signal::Inhibit(false)
		});

	}
}

pub fn radio_valores (ui: Rc<AtroxUi>) -> ValorRadios {

	let mut valor = ValorRadios { dec :0, cob: 0, sol:0, ppt:0, tct:0, tte:0, tex:0, ero:0, can:0, fmt:0 };

	macro_rules! radio_ativo {
		($radio:ident, $cons:ident, $valor:expr) => {
			if ui.$radio.is_active() {
				valor.$cons = $valor;
			}
		}
	}

	radio_ativo!(check_dec1, dec, 0);
	radio_ativo!(check_dec2, dec, 1);
	radio_ativo!(check_dec3, dec, 2);
	radio_ativo!(check_dec4, dec, 3);
	radio_ativo!(check_dec5, dec, 4);
	radio_ativo!(check_dec6, dec, 5);

	radio_ativo!(check_cob1, cob, 0);
	radio_ativo!(check_cob2, cob, 1);
	radio_ativo!(check_cob3, cob, 2);
	radio_ativo!(check_cob4, cob, 3);
	radio_ativo!(check_cob5, cob, 4);

	radio_ativo!(check_sol1, sol, 0);
	radio_ativo!(check_sol2, sol, 1);
	radio_ativo!(check_sol3, sol, 2);


	radio_ativo!(check_ppt1, ppt, 0);
	radio_ativo!(check_ppt2, ppt, 1);

	radio_ativo!(check_cultivo1, tct, 0);
	radio_ativo!(check_cultivo2, tct, 1);
	radio_ativo!(check_terraco1, tte, 0);
	radio_ativo!(check_terraco2, tte, 1);
	radio_ativo!(check_textura1, tex, 0);
	radio_ativo!(check_textura2, tex, 1);
	radio_ativo!(check_textura3, tex, 2);
	radio_ativo!(check_erod1, ero, 0);
	radio_ativo!(check_erod2, ero, 1);
	radio_ativo!(check_cob_canal1, can, 0);
	radio_ativo!(check_cob_canal2, can, 1);
	radio_ativo!(check_cob_canal3, can, 2);
	radio_ativo!(check_cob_canal4, can, 3);
	radio_ativo!(check_cob_canal5, can, 4);

	radio_ativo!(check_cob_canal6, can, 5);
	radio_ativo!(check_cob_canal7, can, 6);
	radio_ativo!(check_cob_canal8, can, 7);
	radio_ativo!(check_formato1, fmt, 0);
	radio_ativo!(check_formato2, fmt, 0);
	valor
}


