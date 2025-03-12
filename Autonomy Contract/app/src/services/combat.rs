use sails_rs::{
    prelude::*,
    gstd::msg,
};
use sails_rs::gstd::exec;
use crate::states::*;
use crate::states::state::{SerAutonomo, Comida,Animo};
use crate::services::objets::{Objeto, TipoObjeto, Inventario,Pocion};
use crate::services::combat::state::State;
use crate::services::service::Events;

const EXPERIENCIA_POR_PELEA: u32 = 20;
const RANGO_NIVEL_MATCH: u32 = 2; // Rango de niveles para matching aleatorio

// Atributos base de cada clase
#[derive(Encode, Decode, TypeInfo, Clone,PartialEq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Atributos {
    pub salud_base: u32,
    pub ataque_base: u32,
    pub defensa_base: u32,
    pub mana_base: u32, // Para magos y curanderos
}


#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Accion {
    Atacar(Habilidad), // Usar una habilidad
    UsarPocion(Pocion), // Usar una poción
}

impl Atributos {
    pub fn new(salud: u32, ataque: u32, defensa: u32, mana: u32) -> Self {
        Self {
             salud_base: salud,
             ataque_base: ataque,
             defensa_base: defensa,
             mana_base: mana,
        }
    }
}

// Clases disponibles
#[derive(Encode, Decode, TypeInfo, Clone,PartialEq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Clase {
    Guerrero,
    Mago,
    Arquero,
    Curandero,
}

impl Clase {
    // Obtener atributos base para cada clase
    pub fn atributos_base(&self) -> Atributos {
        match self {
            Clase::Guerrero => Atributos::new(120, 15, 10, 0),  // Alta salud y ataque, sin mana
            Clase::Mago => Atributos::new(80, 10, 5, 100),      // Baja salud, alto mana
            Clase::Arquero => Atributos::new(90, 12, 8, 0),     // Salud media, ataque medio
            Clase::Curandero => Atributos::new(100, 8, 6, 80),  // Salud alta, bajo ataque, mana medio
        }
    }

    // Obtener habilidades disponibles para cada clase
    pub fn habilidades_disponibles(&self) -> Vec<Habilidad> {
        match self {
            Clase::Guerrero => vec![Habilidad::AtaqueBasico, Habilidad::GolpeFeroz],
            Clase::Mago => vec![Habilidad::BolaDeFuego, Habilidad::EscudoMagico],
            Clase::Arquero => vec![Habilidad::FlechaRapida, Habilidad::DisparoPreciso],
            Clase::Curandero => vec![Habilidad::Curacion, Habilidad::Bendicion],
        }
    }
}

// Habilidades disponibles
#[derive(Encode, Decode, TypeInfo, Clone,PartialEq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Habilidad {
    AtaqueBasico,      // Todas las clases
    GolpeFeroz,        // Guerrero
    BolaDeFuego,       // Mago
    EscudoMagico,      // Mago
    FlechaRapida,      // Arquero
    DisparoPreciso,    // Arquero
    Curacion,          // Curandero
    Bendicion,         // Curandero
}

impl Habilidad {
    // Calcular daño o efecto de la habilidad
    pub fn calcular_efecto(&self, nivel: u32, atributos: &Atributos) -> u32 {
        match self {
            Habilidad::AtaqueBasico => atributos.ataque_base + nivel * 2,
            Habilidad::GolpeFeroz => atributos.ataque_base * 2 + nivel * 3,
            Habilidad::BolaDeFuego => atributos.mana_base / 2 + nivel * 4,
            Habilidad::EscudoMagico => atributos.defensa_base + nivel * 3,
            Habilidad::FlechaRapida => atributos.ataque_base + nivel * 2,
            Habilidad::DisparoPreciso => atributos.ataque_base * 3 + nivel * 4,
            Habilidad::Curacion => atributos.mana_base / 4 + nivel * 5,
            Habilidad::Bendicion => atributos.mana_base / 3 + nivel * 6,
        }
    }
}

// Estado de una pelea
#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Pelea {
    ser_atacante_id: u32,
    ser_defensor_id: u32,
    turno: u32, // 0: atacante, 1: defensor
    habilidades_usadas: Vec<Habilidad>, // Habilidades usadas en la pelea
    salud_inicial_atacante: u32, // Salud inicial del atacante
    salud_inicial_defensor: u32, // Salud inicial del defensor
    stats_iniciales_atacante: Atributos, // Stats iniciales del atacante
    stats_iniciales_defensor: Atributos, // Stats iniciales del defensor
}

impl Pelea {
    pub fn new(ser_atacante_id: u32, ser_defensor_id: u32) -> Self {
        let state = State::state_ref(); // Obtener una referencia al estado global

        // Obtener los datos del atacante y defensor
        let ser_atacante = state.seres.get(&ser_atacante_id).unwrap();
        let ser_defensor = state.seres.get(&ser_defensor_id).unwrap();

        Self {
            ser_atacante_id,
            ser_defensor_id,
            turno: Self::decidir_primer_turno(),
            habilidades_usadas: Vec::new(),
            salud_inicial_atacante: ser_atacante.salud, // Inicializar salud inicial del atacante
            salud_inicial_defensor: ser_defensor.salud, // Inicializar salud inicial del defensor
            stats_iniciales_atacante: ser_atacante.atributos.clone(), // Inicializar stats iniciales del atacante
            stats_iniciales_defensor: ser_defensor.atributos.clone(), // Inicializar stats iniciales del defensor
        }
    }

    // Decidir al azar quién ataca primero
    fn decidir_primer_turno() -> u32 {
        let timestamp = exec::block_timestamp();
        if timestamp % 2 == 0 { 0 } else { 1 }
    }
}

// Sistema de matching
#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ModoMatching {
    Invitacion(u32), // ID del ser invitado
    Aleatorio,
}

// Estado global de matching
#[derive(Default,Clone)]
pub struct MatchingState {
    pub cola_aleatoria: Vec<u32>, // Cola de seres buscando pelea aleatoria
    pub invitaciones: Vec<(u32, u32)>, // (ser_invitador, ser_invitado)
    pub peleas_activas: Vec<Pelea>, // Peleas en curso
}


pub fn obtener_estado_matching() -> Events {
    let state = State::state_ref();
    let matching_state = &state.matching_state;

    let cola_aleatoria = matching_state.cola_aleatoria.clone();
    let invitaciones = matching_state.invitaciones.clone();
    let peleas_activas = matching_state.peleas_activas.clone();

    Events::EstadoMatching {
        cola_aleatoria,
        invitaciones,
        peleas_activas,
    }
}


impl MatchingState {
    // Buscar match aleatorio
    pub fn buscar_match_aleatorio(&mut self, ser_id: u32, nivel: u32) -> Option<u32> {
        for (i, &otro_ser_id) in self.cola_aleatoria.iter().enumerate() {
            let otro_ser = State::state_ref().seres.get(&otro_ser_id).unwrap();
            if (otro_ser.nivel as i32 - nivel as i32).abs() <= RANGO_NIVEL_MATCH as i32 {
                self.cola_aleatoria.remove(i);
                return Some(otro_ser_id);
            }
        }
        self.cola_aleatoria.push(ser_id);
        None
    }

    // Enviar invitación
    pub fn enviar_invitacion(&mut self, ser_invitador: u32, ser_invitado: u32) -> bool {
        if State::state_ref().seres.contains_key(&ser_invitado) {
            self.invitaciones.push((ser_invitador, ser_invitado));
            true
        } else {
            false
        }
    }

    pub fn aceptar_invitacion(&mut self, ser_invitado: u32) -> Option<(u32, u32)> {
        // Clonar los valores necesarios antes de modificar `self.invitaciones`
        let invitacion = self.invitaciones
            .iter()
            .find(|&&(_, id)| id == ser_invitado)
            .cloned(); // Clonar la tupla (invitador, invitado)
    
        if let Some((invitador, invitado)) = invitacion {
            // Ahora puedes modificar `self.invitaciones` sin problemas
            self.invitaciones.retain(|&(_, id)| id != ser_invitado);
            Some((invitador, invitado))
        } else {
            None
        }
    }

    // Iniciar pelea
    pub fn iniciar_pelea(&mut self, ser_atacante_id: u32, ser_defensor_id: u32) {
        let state = State::state_ref();
        let ser_atacante = state.seres.get(&ser_atacante_id).unwrap();
        let ser_defensor = state.seres.get(&ser_defensor_id).unwrap();

        let pelea = Pelea {
            ser_atacante_id,
            ser_defensor_id,
            turno: Pelea::decidir_primer_turno(),
            habilidades_usadas: Vec::new(),
            salud_inicial_atacante: ser_atacante.salud,
            salud_inicial_defensor: ser_defensor.salud,
            stats_iniciales_atacante: ser_atacante.atributos.clone(),
            stats_iniciales_defensor: ser_defensor.atributos.clone(),
        };

        self.peleas_activas.push(pelea);
    }
}

// Función para buscar pelea
pub fn buscar_pelea(ser_id: u32, modo: ModoMatching) -> Events {
    let state = State::state_mut();
    let matching_state = &mut state.matching_state;

    match modo {
        ModoMatching::Invitacion(invitado_id) => {
            if matching_state.enviar_invitacion(ser_id, invitado_id) {
                Events::InvitacionEnviada(ser_id, invitado_id)
            } else {
                Events::SerNoExiste
            }
        }
        ModoMatching::Aleatorio => {
            let ser = match state.seres.get(&ser_id) {
                Some(ser) => ser,
                None => return Events::SerNoExiste,
            };

            if let Some(oponente_id) = matching_state.buscar_match_aleatorio(ser_id, ser.nivel) {
                matching_state.iniciar_pelea(ser_id, oponente_id);
                Events::PeleaIniciada(ser_id, oponente_id)
            } else {
                Events::BuscandoPelea(ser_id)
            }
        }
    }
}

// Función para aceptar invitación
pub fn aceptar_invitacion(ser_invitado: u32) -> Events {
    let state = State::state_mut();
    let matching_state = &mut state.matching_state;

    if let Some((invitador, invitado)) = matching_state.aceptar_invitacion(ser_invitado) {
        matching_state.iniciar_pelea(invitador, invitado);
        Events::PeleaIniciada(invitador, invitado)
    } else {
        Events::InvitacionNoEncontrada
    }
}

pub fn realizar_accion_en_pelea(ser_id: u32, accion: Accion) -> Events {
    let state = State::state_mut();
    let matching_state = &mut state.matching_state;

    // Obtener la pelea activa
    let pelea_index = matching_state.peleas_activas
        .iter()
        .position(|p| p.ser_atacante_id == ser_id || p.ser_defensor_id == ser_id);

    if let Some(index) = pelea_index {
        let pelea = &mut matching_state.peleas_activas[index];

        // Determinar quién es el atacante y quién es el defensor en este turno
        let (atacante_id, defensor_id) = if pelea.turno == 0 {
            (pelea.ser_atacante_id, pelea.ser_defensor_id)
        } else {
            (pelea.ser_defensor_id, pelea.ser_atacante_id)
        };

        // Verificar si es el turno del ser
        if atacante_id != ser_id {
            return Events::NoEsTuTurno;
        }

        // Obtener el ser atacante
        let ser_atacante = state.seres.get_mut(&atacante_id).unwrap();

        match accion {
            Accion::Atacar(habilidad) => {
                // Verificar si la habilidad ya fue usada en el turno anterior
                if pelea.habilidades_usadas.contains(&habilidad) {
                    return Events::HabilidadYaUsada;
                }

                // Verificar si la habilidad es válida para la clase del atacante
                let habilidades_disponibles = ser_atacante.clase.as_ref().map(|c| c.habilidades_disponibles()).unwrap_or_default();
                if !habilidades_disponibles.contains(&habilidad) {
                    return Events::HabilidadNoDisponible;
                }

                // Verificar si el atacante tiene suficiente mana (solo para magos y curanderos)
                if let Some(Clase::Mago) | Some(Clase::Curandero) = ser_atacante.clase {
                    let costo_mana = match habilidad {
                        Habilidad::BolaDeFuego => 20,
                        Habilidad::EscudoMagico => 15,
                        Habilidad::Curacion => 25,
                        Habilidad::Bendicion => 30,
                        _ => 0,
                    };

                    if ser_atacante.mana < costo_mana {
                        return Events::ManaInsuficiente;
                    }

                    // Reducir el mana
                    ser_atacante.mana -= costo_mana;
                }

                // Calcular el daño
                let daño = habilidad.calcular_efecto(ser_atacante.nivel, &ser_atacante.atributos);

                // Obtener el ser defensor
                let mut ser_defensor = state.seres.get_mut(&defensor_id).unwrap();
                ser_defensor.salud = ser_defensor.salud.saturating_sub(daño);

                // Registrar la habilidad usada
                pelea.habilidades_usadas.push(habilidad);

                // Cambiar el turno
                pelea.turno = 1 - pelea.turno;

                // Verificar si la pelea ha terminado
                if ser_defensor.salud == 0 {
                    matching_state.peleas_activas.remove(index);
                    return Events::PeleaTerminada(atacante_id, defensor_id);
                }

                Events::AtaqueRealizado(atacante_id, defensor_id, daño)
            }
            Accion::UsarPocion(pocion) => {
                // Buscar la poción en el inventario
                let pocion_index = ser_atacante.inventario.pociones.iter().position(|p| {
                    p.pocion.as_ref() == Some(&pocion)
                });

                if let Some(index) = pocion_index {
                    // Verificar si hay suficientes pociones
                    if ser_atacante.inventario.pociones[index].cantidad == 0 {
                        return Events::PocionNoEncontrada;
                    }

                    // Aplicar el efecto de la poción
                    match pocion {
                        Pocion::Salud(cantidad) => {
                            ser_atacante.salud = ser_atacante.salud.saturating_add(cantidad);
                        }
                        Pocion::Mana(cantidad) => {
                            ser_atacante.mana = ser_atacante.mana.saturating_add(cantidad);
                        }
                    }

                    // Reducir la cantidad de pociones
                    ser_atacante.inventario.pociones[index].cantidad -= 1;

                    // Si la cantidad llega a 0, eliminar la poción del inventario
                    if ser_atacante.inventario.pociones[index].cantidad == 0 {
                        ser_atacante.inventario.pociones.remove(index);
                    }

                    // Cambiar el turno
                    pelea.turno = 1 - pelea.turno;

                    Events::PocionUsada(ser_id, pocion)
                } else {
                    Events::PocionNoEncontrada
                }
            }
        }
    } else {
        Events::PeleaNoEncontrada
    }
}
