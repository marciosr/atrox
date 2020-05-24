#[allow(unused)]

use std::rc::Rc;
use crate::atroxui::ValorRadios;

pub static COEFICIENTES: [[[f64; 6];3];5] = // ver o que fazer com isso
	[
		[
			[0.50,0.60,0.68,0.76,0.85,0.95],
			[0.44,0.52,0.59,0.66,0.73,0.81],
			[0.40,0.48,0.54,0.61,0.67,0.75]
		],
		[
			[0.40,0.48,0.54,0.61,0.67,0.75],
		  [0.34,0.41,0.46,0.52,0.56,0.64],
		 	[0.31,0.38,0.43,0.48,0.53,0.59]
		],
		[
			[0.31,0.38,0.43,0.48,0.53,0.59],
		 	[0.27,0.32,0.37,0.41,0.45,0.50],
		 	[0.25,0.30,0.34,0.38,0.42,0.46]
		],
		[
			[0.22,0.26,0.29,0.33,0.37,0.41],
		 	[0.19,0.23,0.25,0.28,0.32,0.35],
		 	[0.17,0.21,0.23,0.26,0.29,0.32]
		],
		[
			[0.15,0.18,0.20,0.22,0.25,0.28],
		 	[0.13,0.15,0.18,0.20,0.22,0.24],
		 	[0.12,0.14,0.16,0.18,0.20,0.22]
		]
	];

pub static VELOCIDADE: [[f64; 8];2] =
					[
	                    [0.45,0.60,0.75,0.90,0.90,1.20,1.50,1.80],
	                    [0.60,0.75,0.90,1.05,1.20,1.50,1.80,2.10]
	                ];

pub static COEF_X: [[[f64; 3];2];2] =
		[
	        [
	            [1.5,2.0,2.5],		//Gradiente
			[2.5,3.0,3.5]
	        ],
	        [
	        	[3.5,4.0,4.5],		//argiloso, médio e arenoso
			[4.5,5.0,5.5]
	        ]
	    ];

struct VolumeVazao {
	volume_enxurrada: f64,
	vazao_max: f64,
}

pub struct Calculos {
	pub tempo_concent: 			f64,
	pub coef_escorrimento:		f64,
	pub intens_max_chuva:		f64,
	pub vazao_max:				f64,
	pub volume_enxurrada:		f64,
	pub esp_horiz:				f64,
	pub esp_vert:				f64,
	pub sessao:					f64,	// variável S anteriormente
	pub b_maior:				f64,	// B
	pub b_menor:				f64,	// b
	pub per_molhado:			f64, 	// PM
	pub talude_y:				f64, //
	pub vel_canal:				f64
	//tempo_concent_final: f64,
	//velocidade:				f64,	// V
	//altura:					f64,	// Y
}

impl Calculos {
	pub fn calcula_terraco (valores: ValorRadios, area_bacia: f64, largura_bacia: f64, declividade_talhao: f64, declividade_canal: f64,
							coeficiente_atrito: f64, profundidade: f64, tempo_retorno: f64) -> Calculos {

		// Não exponho o tempo de concentração não corrigido
		let tempo_concent = calcula_tempo_conc(area_bacia, largura_bacia, declividade_talhao);
		let (sessao, velocidade): (f64, f64);

		let coef_escorrimento = COEFICIENTES[valores.cob][valores.sol][valores.dec] ;

		let intens_max_chuva = calcula_intensidade_chuva(tempo_concent, tempo_retorno);

		let volume_vazao = calcula_vazao (tempo_concent, area_bacia, coef_escorrimento, intens_max_chuva);

		let vazao_max = volume_vazao.vazao_max;
		let volume_enxurrada = volume_vazao.volume_enxurrada;

		let espacamentos = calcula_espacamento(valores.clone(), declividade_talhao);

		let esp_horiz = espacamentos.esp_horiz;
		let esp_vert = espacamentos.esp_vert;

		let altura = profundidade;
		let proporcao: f64 = 1.0; //valores.proporcao;
		let (b_maior, b_menor, talude_y, per_molhado): (f64, f64, f64, f64);
		let vel_canal: f64;

		let calcula_trapezoidal = |altura: f64, sessao: f64, proporcao: f64, mut canal: DimensoesCanal| {
			canal.talude_y = altura * proporcao;
			canal.b_menor = (sessao * 2.0 * altura * canal.talude_y) / altura;
			canal.b_maior = canal.b_menor + 2.0 * canal.talude_y;
			canal.per_molhado = canal.b_menor + 2.0 * (canal.talude_y.powf(2.0) + altura.powf(2.0)).powf(0.5);
			canal.raio_hidraulico = sessao / canal.per_molhado;
			canal
		};

		let calcula_triangular = |altura, sessao, proporcao: f64, mut canal: DimensoesCanal| {
			canal.b_maior = (2.0 * sessao) / altura;
			//Z = ((b_maior/2.0).powf(2.0) + altura.powf(2.0)).powf(0.5);
			canal.per_molhado = 2.0 * altura * (1.0 + (1.0 / proporcao).powf(2.0)).powf(0.5);
			canal.raio_hidraulico = sessao / canal.per_molhado;
			canal.b_menor = 0.0;
			canal.talude_y = 0.0;
			canal
		};

		let mut canal = DimensoesCanal {
			b_menor: 0.0,
			b_maior: 0.0,
			talude_y: 0.0,
			raio_hidraulico: 0.0,
			per_molhado: 0.0,
		};

// Alterado para uso da estrutura
		if valores.tte == 1 {
			sessao = volume_enxurrada / (area_bacia * 10000.0 / esp_horiz); // Sessão em nível
			match valores.fmt {
				0 => { // Trapezoidal
						canal = calcula_trapezoidal(altura, sessao, proporcao, canal);
				},
				1 => { // Triangular
						canal = calcula_triangular(altura, sessao, proporcao, canal);
				},
				x => {
						panic! ("Unexpected invalid token {:?}", x)
				}
			}
			vel_canal = 0.0;
			b_maior = canal.b_maior;
			b_menor = canal.b_menor;
			talude_y = canal.talude_y;
			per_molhado = canal.per_molhado;
		} else {
			velocidade = calcula_velocidade_maxima(valores.clone());
			sessao = volume_vazao.vazao_max / velocidade; 							// Sessão em gradiente
			match valores.fmt {
				0 => { // Trapezoidal
						canal = calcula_trapezoidal(altura, sessao, proporcao, canal);
						//vel_canal = (raio_hidraulico.powf(2.0/3.0) * valores.declividade_canal.powf(0.5)) / valores.coeficiente_atrito;
				},
				1 => { // Triangular
						canal = calcula_triangular(altura, sessao, proporcao, canal);
				},
				x => {
						panic! ("Unexpected invalid token {:?}", x)
					}
			}
			vel_canal = (canal.raio_hidraulico.powf(2.0/3.0) * declividade_canal.powf(0.5)) / coeficiente_atrito;
			b_maior = canal.b_maior;
			b_menor = canal.b_menor;
			talude_y = canal.talude_y;
			per_molhado = canal.per_molhado;
			// Saida.vel_canal = ver_velocidade_canal (Dimensoes.R, V, declividade_canal, coeficiente_atrito);
		}
		Calculos { tempo_concent, coef_escorrimento, intens_max_chuva, vazao_max,
					volume_enxurrada, esp_horiz, esp_vert, sessao, b_maior, b_menor,
					per_molhado, talude_y, vel_canal }
	}
}

struct DimensoesCanal {
	b_menor: f64,
	b_maior: f64,
	talude_y: f64,
	raio_hidraulico: f64,
	per_molhado: f64,
}

fn calcula_tempo_conc (area_bacia: f64, largura_bacia: f64, declividade_talhao: f64) -> f64 {
	let tempo_concent: f64;
	let tempo_concent_corr: f64;
	let tempo_concent_final: f64;


	if area_bacia > 500.0 {
		println!("Válido apenas para bacias hidrográficas inferiores a 500ha!");
		panic! ("Válido apenas para bacias hidrográficas inferiores a 500ha! {:?}");
	} else {
		if area_bacia <= 3.0 {
			tempo_concent = -0.1375 * area_bacia.powf(2.0) + 1.15 * area_bacia + 1.6875;
		} else if area_bacia > 3.0 && area_bacia <= 8.0 {
			tempo_concent = 0.03666666666667 * area_bacia.powf(2.0) - 0.243333333334 * area_bacia +4.3;
		} else if area_bacia > 8.0 && area_bacia <= 30.0 {
			tempo_concent = -0.007813081689875 * area_bacia.powf(2.0) + 0.738213873377158 * area_bacia - 0.091909945571494;
		} else {
			tempo_concent = 0.00008513311819764940 * area_bacia.powf(2.0) + 0.123118340144573 * area_bacia + 12.2730007237057;
		}

		tempo_concent_corr = ((0.230 * ((area_bacia * 1000.0 / largura_bacia) / largura_bacia)) + 0.5050) * tempo_concent;
	}

	if declividade_talhao != 5.0 {
		tempo_concent_final = tempo_concent_corr / (declividade_talhao / 0.22);
	}
	else {
		tempo_concent_final = tempo_concent_corr;
	}
	tempo_concent_final
}

fn calcula_intensidade_chuva (tempo_concent: f64, tempo_retorno: f64) -> f64 {
	let i_max_chuva: f64;

	i_max_chuva =  60.0 * 39.3015 * (tempo_concent + 20.0).powf(-0.9228)+10.1767 * (tempo_concent + 20.0).powf(-0.8764) * (-0.4653-0.804 * ((tempo_retorno/(tempo_retorno - 1.0).log10())).log10());
	i_max_chuva
}

fn calcula_vazao (tempo_concent: f64, area_bacia: f64, coef_escorrimento: f64, intens_max_chuva: f64) -> VolumeVazao {
	let mut volumevazao= VolumeVazao {
		volume_enxurrada: 0.0,
		vazao_max: 0.0,
	};
	let (vazao_max, volume_enxurrada): (f64, f64);
	vazao_max = (coef_escorrimento * intens_max_chuva * area_bacia) / 360.0;
	volume_enxurrada = (tempo_concent * intens_max_chuva * area_bacia * 10.0 * coef_escorrimento) / 60.0;
	volumevazao.volume_enxurrada = volume_enxurrada;
	volumevazao.vazao_max = vazao_max;
	volumevazao
}

struct Espacamentos {
	esp_vert: f64,
	esp_horiz: f64,
}

fn calcula_espacamento (valores: ValorRadios, declividade_talhao: f64) -> Espacamentos {
	let x = COEF_X[valores.tte][valores.tct][valores.tex] ;
	let esp_vert = (2.0 + (declividade_talhao / x)) * 0.305;
	let esp_horiz = (esp_vert * 100.0) / declividade_talhao;
	Espacamentos { esp_vert, esp_horiz }
}

fn calcula_velocidade_maxima (valores: ValorRadios) -> f64 {
	let vel_maxima = VELOCIDADE[valores.ero][valores.can];
	vel_maxima
}


