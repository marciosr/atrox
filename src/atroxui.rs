#[allow(unused)]
use gtk::prelude::*;
use gtk::{Builder, Button, Label, HeaderBar, ApplicationWindow, Entry, SpinButton, RadioButton, Adjustment};
use std::rc::Rc;
use glib::{clone};

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
	pub radiodec1: 		RadioButton,
	pub radiodec2: 		RadioButton,
	pub radiodec3: 		RadioButton,
	pub radiodec4: 		RadioButton,
	pub radiodec5: 		RadioButton,
	pub radiodec6: 		RadioButton,
	pub radiocob1: 		RadioButton,
	pub radiocob2: 		RadioButton,
	pub radiocob3: 		RadioButton,
	pub radiocob4: 		RadioButton,
	pub radiocob5: 		RadioButton,
	pub radiosol1: 		RadioButton,
	pub radiosol2: 		RadioButton,
	pub radiosol3: 		RadioButton,
	pub radioppt1: 		RadioButton,
	pub radioppt2: 		RadioButton,
	pub radiocultivo1: 	RadioButton,
	pub radiocultivo2: 	RadioButton,
	pub radioterraco1: 	RadioButton,
	pub radioterraco2: 	RadioButton,
	pub radiotextura1: 	RadioButton,
	pub radiotextura2: 	RadioButton,
	pub radiotextura3: 	RadioButton,
	pub radioerod1: 	RadioButton,
	pub radioerod2: 	RadioButton,
	pub radiocanalcob1: RadioButton,
	pub radiocanalcob2: RadioButton,
	pub radiocanalcob3: RadioButton,
	pub radiocanalcob4: RadioButton,
	pub radiocanalcob5: RadioButton,
	pub radiocanalcob6: RadioButton,
	pub radiocanalcob7: RadioButton,
	pub radiocanalcob8: RadioButton,
	pub radioformato1: 	RadioButton,
	pub radioformato2: 	RadioButton,
}

impl AtroxUi {
	pub fn new() -> Rc<Self> {

		let glade_src = include_str!("atrox-rs.ui");
		let builder = gtk::Builder::new_from_string(glade_src);

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

		get_widget!(builder,RadioButton,radiodec1);
		get_widget!(builder,RadioButton,radiodec2);
		get_widget!(builder,RadioButton,radiodec3);
		get_widget!(builder,RadioButton,radiodec4);
		get_widget!(builder,RadioButton,radiodec5);
		get_widget!(builder,RadioButton,radiodec6);
		get_widget!(builder,RadioButton,radiocob1);
		get_widget!(builder,RadioButton,radiocob2);
		get_widget!(builder,RadioButton,radiocob3);
		get_widget!(builder,RadioButton,radiocob4);
		get_widget!(builder,RadioButton,radiocob5);
		get_widget!(builder,RadioButton,radiosol1);
		get_widget!(builder,RadioButton,radiosol2);
		get_widget!(builder,RadioButton,radiosol3);
		get_widget!(builder,RadioButton,radioppt1);
		get_widget!(builder,RadioButton,radioppt2);
		get_widget!(builder,RadioButton,radiocultivo1);
		get_widget!(builder,RadioButton,radiocultivo2);
		get_widget!(builder,RadioButton,radioterraco1);
		get_widget!(builder,RadioButton,radioterraco2);
		get_widget!(builder,RadioButton,radiotextura1);
		get_widget!(builder,RadioButton,radiotextura2);
		get_widget!(builder,RadioButton,radiotextura3);
		get_widget!(builder,RadioButton,radioerod1);
		get_widget!(builder,RadioButton,radioerod2);
		get_widget!(builder,RadioButton,radiocanalcob1);
		get_widget!(builder,RadioButton,radiocanalcob2);
		get_widget!(builder,RadioButton,radiocanalcob3);
		get_widget!(builder,RadioButton,radiocanalcob4);
		get_widget!(builder,RadioButton,radiocanalcob5);
		get_widget!(builder,RadioButton,radiocanalcob6);
		get_widget!(builder,RadioButton,radiocanalcob7);
		get_widget!(builder,RadioButton,radiocanalcob8);
		get_widget!(builder,RadioButton,radioformato1);
		get_widget!(builder,RadioButton,radioformato2);

		//let window_clone = window.clone();
		window.connect_delete_event(clone!(@strong window => move |_,_| {
			//window.destroy();
			gtk::main_quit();
    	Inhibit(true)
		}));

		let atroxui = Rc::new(Self { builder, window, header_bar, bt_calcular, bt_calcula2, spin_area_bacia,
				  spin_largura_bacia, spin_dec_talhao, spin_dec_canal, spin_coef_atrito,
				  spin_profundidade, spin_retorno, entry_tempo_conc,
				  entry_coef_escorrimento, entry_intensidade_max, entry_vazao_bacia,
				  entry_volume_enxurrada, entry_esp_h, entry_esp_v, entry_sessao,
				  entry_vel_max, entry_bb, entry_b, entry_y, entry_pm, label_critica,
				  adjustment_area, radiodec1, radiodec2, radiodec3, radiodec4, radiodec5, radiodec6,
				  radiocob1, radiocob2, radiocob3, radiocob4, radiocob5, radiosol1, radiosol2, radiosol3,
				  radioppt1, radioppt2, radiocultivo1, radiocultivo2, radioterraco1, radioterraco2,
				  radiotextura1, radiotextura2, radiotextura3, radioerod1, radioerod2, radiocanalcob1,
				  radiocanalcob2, radiocanalcob3, radiocanalcob4, radiocanalcob5, radiocanalcob6,
				  radiocanalcob7, radiocanalcob8, radioformato1, radioformato2
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
													spin_area_bacia.get_value(),
													spin_largura_bacia.get_value(),
													spin_dec_talhao.get_value(),
													spin_dec_canal.get_value(),
													spin_coef_atrito.get_value(),
													spin_profundidade.get_value(),
													spin_retorno.get_value());
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
	}
}

pub fn radio_valores (ui: Rc<AtroxUi>) -> ValorRadios {

	let mut valor = ValorRadios { dec :0, cob: 0, sol:0, ppt:0, tct:0, tte:0, tex:0, ero:0, can:0, fmt:0 };

	macro_rules! radio_ativo {
		($radio:ident, $cons:ident, $valor:expr) => {
			if ui.$radio.get_active() {
				valor.$cons = $valor;
			}
		}
	};

	radio_ativo!(radiodec1, dec, 0);
	radio_ativo!(radiodec2, dec, 1);
	radio_ativo!(radiodec3, dec, 2);
	radio_ativo!(radiodec4, dec, 3);
	radio_ativo!(radiodec5, dec, 4);
	radio_ativo!(radiodec6, dec, 5);

	radio_ativo!(radiocob1, cob, 0);
	radio_ativo!(radiocob2, cob, 1);
	radio_ativo!(radiocob3, cob, 2);
	radio_ativo!(radiocob4, cob, 3);
	radio_ativo!(radiocob5, cob, 4);

	radio_ativo!(radiosol1, sol, 0);
	radio_ativo!(radiosol2, sol, 1);
	radio_ativo!(radiosol3, sol, 2);


	radio_ativo!(radioppt1, ppt, 0);
	radio_ativo!(radioppt2, ppt, 1);

	radio_ativo!(radiocultivo1, tct, 0);
	radio_ativo!(radiocultivo2, tct, 1);
	radio_ativo!(radioterraco1, tte, 0);
	radio_ativo!(radioterraco2, tte, 1);
	radio_ativo!(radiotextura1, tex, 0);
	radio_ativo!(radiotextura2, tex, 1);
	radio_ativo!(radiotextura3, tex, 2);
	radio_ativo!(radioerod1, ero, 0);
	radio_ativo!(radioerod2, ero, 1);
	radio_ativo!(radiocanalcob1, can, 0);
	radio_ativo!(radiocanalcob2, can, 1);
	radio_ativo!(radiocanalcob3, can, 2);
	radio_ativo!(radiocanalcob4, can, 3);
	radio_ativo!(radiocanalcob5, can, 4);

	radio_ativo!(radiocanalcob6, can, 5);
	radio_ativo!(radiocanalcob7, can, 6);
	radio_ativo!(radiocanalcob8, can, 7);
	radio_ativo!(radioformato1, fmt, 0);
	radio_ativo!(radioformato2, fmt, 0);
	valor
}


