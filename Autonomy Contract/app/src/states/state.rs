// Add your state code
// Imports necesarios
use sails_rs::{
    prelude::*,
};
use crate::services::combat::{Atributos, Clase, Habilidad,MatchingState};
use crate::services::objets::{Inventario};
use gstd::collections::BTreeMap;

// Variable estática mutable para el estado del contrato
pub static mut STATE: Option<State> = None;


#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Animo {
    Feliz,
    Triste,
    Enojado,
    Enfermo,
    Neutral,
}



#[derive(Debug, Clone, Encode, Decode, TypeInfo, Default)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Clima {
    #[default] Soleado,  // Valor predeterminado
    Lluvioso,
    Nevado,
    Ventoso,
}


// Estructura para representar un ser autónomo
#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct SerAutonomo {
    pub owner: ActorId,
    pub name:String,
    pub energia: u32,
    pub salud:u32,
    pub mana:u32,
    pub tiempo_de_vida_restante: u64,
    pub esta_vivo: bool,
    pub ultima_comida: u64,
    pub ultima_siesta: u64,
    pub esta_dormido: bool,
    pub estado_animo: String,
    pub animo: Animo,
    pub amigos: Vec<u32>,  // IDs de otros seres con los que interactúa
    pub historial_chat: Vec<String>,
    pub nivel: u32,         // Nuevo campo de nivel
    pub experiencia: u32, 
    pub ultima_actualizacion: u64,
    pub clase: Option<Clase>, // Clase del ser (Guerrero, Mago, Arquero, Curandero)
    pub habilidades: Vec<Habilidad>, // Habilidades aprendidas por el ser
    pub atributos: Atributos, // Atributos base del ser (salud, ataque, defensa, mana)
    pub inventario:Inventario
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Comida {
    pub nombre: String,
    pub energia: u32,
    pub es_saludable: bool,
}


// Estructura para representar el estado del contrato
#[derive(Clone, Default)]
pub struct State {
    pub admins: Vec<ActorId>,
    pub players: Vec<(ActorId, i32)>,
    pub games: Vec<(ActorId, GameResult)>,
    pub seres: BTreeMap<u32, SerAutonomo>,
    pub next_id: u32,
    pub clima_actual: Clima, 
    pub matching_state: MatchingState,
}

// Estructura para representar un resultado de partida
#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct GameResult {
    pub player: ActorId,
    pub score: i32,
    pub description: String,
}

// Implementaciones relacionadas al estado
impl State {
    pub fn new() -> Self {
        Self { next_id: 0, ..Default::default() }
    }

    pub fn init_state() {
        unsafe {
            STATE = Some(Self::new());
        };
    }

    
    

    pub fn state_mut() -> &'static mut State {
        let state = unsafe { STATE.as_mut() };
        debug_assert!(state.is_some(), "El estado no está inicializado");
        unsafe { state.unwrap_unchecked() }
    }

    pub fn state_ref() -> &'static State {
        let state = unsafe { STATE.as_ref() };
        debug_assert!(state.is_some(), "El estado no está inicializado");
        unsafe { state.unwrap_unchecked() }
    }
}

// Estructura para enviar el estado al usuario que lo lee
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct IoState {
    pub admins: Vec<ActorId>,
    pub players: Vec<(ActorId, i32)>,
    pub games: Vec<(ActorId, GameResult)>,
    pub seres: BTreeMap<u32, SerAutonomo>,
}

// Implementación de conversión de State a IoState
impl From<State> for IoState {
    fn from(value: State) -> Self {
        let State {
            admins,
            players,
            games,
            seres,
            ..
        } = value;

        Self {
            admins,
            players,
            games,
            seres,
        }
    }
}
