#[allow(unused)]

extern crate gtk;

use gtk::*;
//use std::string::String;
use std::rc::Rc;

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
	pub window: Window,
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
		let window: gtk::Window = builder.get_object("window").unwrap();

		let header_bar: HeaderBar = builder.get_object("header_bar").unwrap();
		let bt_calcular: Button = builder.get_object("bt_calcular").unwrap();
		let bt_calcula2: Button = builder.get_object("bt_calcular").unwrap();

		let spin_area_bacia: SpinButton = builder.get_object("sb_area_bacia").unwrap();
		let spin_largura_bacia: SpinButton = builder.get_object("spin_largura_bacia").unwrap();
		let spin_dec_talhao: SpinButton = builder.get_object("spin_dec_talhao").unwrap();
		let spin_dec_canal: SpinButton = builder.get_object("sb_dec_canal").unwrap();
		let spin_coef_atrito: SpinButton = builder.get_object("spin_coef_atrito").unwrap();
		let spin_profundidade: SpinButton = builder.get_object("spin_profundidade").unwrap();
		let spin_retorno: SpinButton = builder.get_object("spin_retorno").unwrap();

		let entry_tempo_conc: Entry = builder.get_object("entry_tempo_conc").unwrap();
		//let entry_tempo_conc_corrigido: Entry = builder.get_object("entry_tempo_conc_corrigido").unwrap();
		let entry_coef_escorrimento: Entry = builder.get_object("entry_coef_escorrimento").unwrap();
		let entry_intensidade_max: Entry = builder.get_object("entry_intensidade_max").unwrap();
		let entry_vazao_bacia: Entry = builder.get_object("entry_vazao_bacia").unwrap();
		let entry_volume_enxurrada: Entry = builder.get_object("entry_volume_enxurrada").unwrap();

		let entry_esp_h: Entry = builder.get_object("entry_esp_h").unwrap();
		let entry_esp_v: Entry = builder.get_object("entry_esp_v").unwrap();
		let entry_sessao: Entry = builder.get_object("entry_sessao").unwrap();
		let entry_vel_max: Entry = builder.get_object("entry_vel_max").unwrap();
		//let entry_h: Entry = builder.get_object("entry_h").unwrap();
		let entry_bb: Entry = builder.get_object("entry_bb").unwrap();
		let entry_b: Entry = builder.get_object("entry_b").unwrap();
		let entry_y: Entry = builder.get_object("entry_y").unwrap();
		let entry_pm: Entry = builder.get_object("entry_pm").unwrap();

		let label_critica: Label = builder.get_object("label_critica").unwrap();

		let adjustment_area: Adjustment = builder.get_object("adjustment_area").unwrap();

		let radiodec1: RadioButton = builder.get_object("radiodec1").unwrap();
		let radiodec2: RadioButton = builder.get_object("radiodec2").unwrap();
		let radiodec3: RadioButton = builder.get_object("radiodec3").unwrap();
		let radiodec4: RadioButton = builder.get_object("radiodec4").unwrap();
		let radiodec5: RadioButton = builder.get_object("radiodec5").unwrap();
		let radiodec6: RadioButton = builder.get_object("radiodec6").unwrap();

		let radiocob1: RadioButton = builder.get_object("radiocob1").unwrap();
		let radiocob2: RadioButton = builder.get_object("radiocob2").unwrap();
		let radiocob3: RadioButton = builder.get_object("radiocob3").unwrap();
		let radiocob4: RadioButton = builder.get_object("radiocob4").unwrap();
		let radiocob5: RadioButton = builder.get_object("radiocob5").unwrap();

		let radiosol1: RadioButton = builder.get_object("radiosol1").unwrap();
		let radiosol2: RadioButton = builder.get_object("radiosol2").unwrap();
		let radiosol3: RadioButton = builder.get_object("radiosol3").unwrap();

		let radioppt1: RadioButton = builder.get_object("radioppt1").unwrap();
		let radioppt2: RadioButton = builder.get_object("radioppt2").unwrap();

		let radiocultivo1: RadioButton = builder.get_object("radiocultivo1").unwrap();
		let radiocultivo2: RadioButton = builder.get_object("radiocultivo2").unwrap();

		let radioterraco1: RadioButton = builder.get_object("radioterraco1").unwrap();
		let radioterraco2: RadioButton = builder.get_object("radioterraco2").unwrap();

		let radiotextura1: RadioButton = builder.get_object("radiotextura1").unwrap();
		let radiotextura2: RadioButton = builder.get_object("radiotextura2").unwrap();
		let radiotextura3: RadioButton = builder.get_object("radiotextura3").unwrap();

		let radioerod1: RadioButton = builder.get_object("radioerod1").unwrap();
		let radioerod2: RadioButton = builder.get_object("radioerod2").unwrap();

		let radiocanalcob1: RadioButton = builder.get_object("radiocanalcob1").unwrap();
		let radiocanalcob2: RadioButton = builder.get_object("radiocanalcob2").unwrap();
		let radiocanalcob3: RadioButton = builder.get_object("radiocanalcob3").unwrap();
		let radiocanalcob4: RadioButton = builder.get_object("radiocanalcob4").unwrap();
		let radiocanalcob5: RadioButton = builder.get_object("radiocanalcob5").unwrap();
		let radiocanalcob6: RadioButton = builder.get_object("radiocanalcob6").unwrap();
		let radiocanalcob7: RadioButton = builder.get_object("radiocanalcob7").unwrap();
		let radiocanalcob8: RadioButton = builder.get_object("radiocanalcob8").unwrap();

		let radioformato1: RadioButton = builder.get_object("radioformato1").unwrap();
		let radioformato2: RadioButton = builder.get_object("radioformato2").unwrap();

		window.connect_delete_event(move |_,_| {
			main_quit();
    	 	       Inhibit(false)
		});

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


