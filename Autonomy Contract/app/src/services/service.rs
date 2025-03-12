// Crates necesarias
use sails_rs::{
    prelude::*,
    gstd::msg,
};
use sails_rs::gstd::exec;
// Importar el estado
use crate::states::*;
use crate::states::state::{SerAutonomo, Comida,Animo};
use crate::services::service::state::*;
use crate::services::objets::{Objeto, TipoObjeto, Inventario,Pocion};
use crate::services::combat::{obtener_estado_matching,buscar_pelea, aceptar_invitacion, realizar_accion_en_pelea, ModoMatching, Habilidad,Clase,Pelea,Accion};
const EXPERIENCIA_PARA_SUBIR_NIVEL: u32 = 100;

#[derive(Default)]
pub struct SerAutonomoService;

impl SerAutonomoService {
    pub fn seed() {
        State::init_state();
    }
}

#[service]
impl SerAutonomoService {
    pub fn new() -> Self {
        Self
    }

    fn generar_aleatorio(min: u32, max: u32) -> u32 {
        let timestamp = exec::block_timestamp();
        let rango = (max - min + 1) as u64;
    
        let aleatorio = ((timestamp.wrapping_mul(1103515245).wrapping_add(12345)) / 65536) % rango;
    
        (aleatorio as u32) + min
    }

    // Crear un ser con atributos base según su clase
    pub fn crear_ser(&mut self, name: String, tiempo_de_vida_restante: u64, clase: Clase) -> Events {
        let state = State::state_mut();
        let ser_id = state.next_id;
        let life_per_years = Self::generar_aleatorio(10, 100);
        let life_per_seconds = (life_per_years as u64) * 365 * 24 * 60 * 60;
    
        let atributos = clase.atributos_base();
    
        // Obtener las habilidades iniciales de la clase
        let habilidades_iniciales = clase.habilidades_disponibles();
    
        state.seres.insert(
            ser_id,
            SerAutonomo {
                owner: msg::source(),
                energia: 100,
                name,
                tiempo_de_vida_restante,
                esta_vivo: true,
                ultima_comida: exec::block_timestamp(),
                ultima_siesta: exec::block_timestamp(),
                esta_dormido: false,
                estado_animo: "Feliz".to_string(),
                salud: atributos.salud_base,
                mana:atributos.mana_base,
                historial_chat: Vec::new(),
                amigos: Vec::new(),
                animo: Animo::Feliz,
                nivel: 1,
                experiencia: 0,
                ultima_actualizacion: exec::block_timestamp(),
                clase: Some(clase),
                habilidades: habilidades_iniciales, // Asignar las habilidades iniciales
                atributos,
                inventario:Inventario::new()
            },
        );
    
        state.next_id += 1;
        Events::SerCreado(ser_id)
    }


    pub fn alimentar(&mut self, ser_id: u32, comida: Comida) -> Events {
        let state = State::state_mut();
    
        let ser = match state.seres.get_mut(&ser_id) {
            Some(ser) => ser,
            None => return Events::SerNoExiste,
        };
    
        if msg::source() != ser.owner {
            return Events::AccionNoPermitida;
        }
    
        if !ser.esta_vivo {
            return Events::SerMuerto;
        }
        
        ser.ultima_actualizacion = exec::block_timestamp();
    
        // 🔹 Reducir tiempo de vida en 300 segundos
        ser.tiempo_de_vida_restante = ser.tiempo_de_vida_restante.saturating_sub(300);
    
        // Aumentar energía (sin pasar de 100)
        ser.energia = (ser.energia + comida.energia).min(100);

    // Otorgar experiencia por alimentarse
    ser.experiencia += 10;
    self.verificar_subida_nivel(ser);

    ser.estado_animo = "Feliz".to_string();
   
        
        // Verificar si murió por fin de vida
        if ser.tiempo_de_vida_restante == 0 {
            ser.esta_vivo = false;
            ser.estado_animo = "Muerto".to_string();
            return Events::SerMuerto;
        }
    
        Events::SerAlimentado(ser_id)
    }


    pub fn dormir(&mut self, ser_id: u32) -> Events {
        let state = State::state_mut();
        let ser = match state.seres.get_mut(&ser_id) {
            Some(ser) => ser,
            None => return Events::SerNoExiste,
        };
    
        if msg::source() != ser.owner {
            return Events::AccionNoPermitida;
        }
    
        if !ser.esta_vivo {
            return Events::SerMuerto;
        }
    
        // Restar 6 horas de vida
        let seis_horas_en_segundos = 6 * 60 * 60;
        ser.tiempo_de_vida_restante = ser.tiempo_de_vida_restante.saturating_sub(seis_horas_en_segundos);
    
        // Darle 100% de energía
        ser.energia = 100;

            // Otorgar experiencia por dormir
    ser.experiencia += 5;
    self.verificar_subida_nivel(ser);
   
        // Poner el ser en estado dormido
        ser.esta_dormido = true;
        ser.ultima_siesta = exec::block_timestamp();
        ser.ultima_actualizacion = exec::block_timestamp();
    
        Events::SerDurmiendo(ser_id)
    }
    


    pub fn despertar(&mut self, ser_id: u32) -> Events {
        let state = State::state_mut();
        let ser = match state.seres.get_mut(&ser_id) {
            Some(ser) => ser,
            None => return Events::SerNoExiste,
        };

        if msg::source() != ser.owner {
            return Events::AccionNoPermitida;
        }

        if !ser.esta_vivo {
            return Events::SerMuerto;
        }
        ser.ultima_actualizacion = exec::block_timestamp();
        ser.esta_dormido = false;
        Events::SerDespierto(ser_id)
    }

    pub fn socializar(&mut self, ser_id: u32, otro_ser_id: u32) -> Events {
        let state = State::state_mut();
    
        // Clonamos los IDs para evitar múltiples referencias mutables al mismo tiempo
        let ser_id_clone = ser_id.clone();
        let otro_ser_id_clone = otro_ser_id.clone();
    
        // Verificar que ambos seres existen antes de mutarlos
        if !state.seres.contains_key(&ser_id_clone) || !state.seres.contains_key(&otro_ser_id_clone) {
            return Events::SerNoExiste;
        }
    
        // Obtener una referencia mutable a cada ser en pasos separados
        let tiempo_a_restar = 600; // 10 minutos
        
        if let Some(ser) = state.seres.get_mut(&ser_id_clone) {
            // Verificar que el dueño sea el que ejecuta la acción
            if msg::source() != ser.owner {
                return Events::AccionNoPermitida;
            }
            ser.ultima_actualizacion = exec::block_timestamp();
            ser.tiempo_de_vida_restante = ser.tiempo_de_vida_restante.saturating_sub(tiempo_a_restar);
        }
    
        if let Some(otro_ser) = state.seres.get_mut(&otro_ser_id_clone) {
            otro_ser.ultima_actualizacion = exec::block_timestamp();
            otro_ser.tiempo_de_vida_restante = otro_ser.tiempo_de_vida_restante.saturating_sub(tiempo_a_restar);
        }

        Events::SerSocializo(ser_id_clone, otro_ser_id_clone)
    }


    pub fn enviar_mensaje_a_otro_ser(&mut self, ser_id_emisor: u32, ser_id_receptor: u32, mensaje: String , respuesta:String) -> Events {
        let state = State::state_mut();
    
        // Extraer uno de los seres para evitar dos referencias mutables al mismo tiempo
        let ser_emisor = match state.seres.remove(&ser_id_emisor) {
            Some(ser) => ser,
            None => return Events::SerNoExiste,
        };
    
        // Verificar que el emisor es el dueño del ser que envía el mensaje
        if msg::source() != ser_emisor.owner {
            state.seres.insert(ser_id_emisor, ser_emisor); // Reinsertar antes de salir
            return Events::AccionNoPermitida;
        }
    
        // Verificar que el emisor está vivo
        if !ser_emisor.esta_vivo {
            state.seres.insert(ser_id_emisor, ser_emisor);
            return Events::SerMuerto;
        }
    
        let ser_receptor = match state.seres.get_mut(&ser_id_receptor) {
            Some(ser) => ser,
            None => {
                state.seres.insert(ser_id_emisor, ser_emisor);
                return Events::SerNoExiste;
            }
        };
    
        // Verificar que el receptor está vivo
        if !ser_receptor.esta_vivo {
            state.seres.insert(ser_id_emisor, ser_emisor);
            return Events::SerMuerto;
        }
    
        let mut ser_emisor = ser_emisor; // Ahora podemos modificarlo
    
        // Limitar el historial de chat
        if ser_emisor.historial_chat.len() >= 10 {
            let len = ser_emisor.historial_chat.len();
            let num_to_remove = (len * 9 + 9) / 10;
            ser_emisor.historial_chat.drain(0..num_to_remove);
        }
    
        if ser_receptor.historial_chat.len() >= 10 {
            let len = ser_receptor.historial_chat.len();
            let num_to_remove = (len * 9 + 9) / 10;
            ser_receptor.historial_chat.drain(0..num_to_remove);
        }
    
        // Agregar el mensaje
        ser_emisor.historial_chat.push(format!("Tú a {}: {}", ser_receptor.name, mensaje));
        ser_receptor.historial_chat.push(format!("{} a ti: {}", ser_emisor.name, respuesta.clone()));
    
        // Otorgar experiencia
        ser_emisor.experiencia += 3;
        ser_receptor.experiencia += 3;
        self.verificar_subida_nivel(&mut ser_emisor);
        self.verificar_subida_nivel(ser_receptor);
    
        // Reinsertar `ser_emisor` en el `HashMap`
        state.seres.insert(ser_id_emisor, ser_emisor);
    
        Events::MensajeEnviado(ser_id_emisor, ser_id_receptor, respuesta)
    }

    pub fn paso_de_tiempo(&mut self, segundos: u64) {
        let state = State::state_mut();
    
        for (_id, ser) in state.seres.iter_mut() {
            if !ser.esta_vivo {
                continue;
            }
    
            // 🔹 Reducimos la energía según si está dormido o no
            let energia_perdida_por_hora = if ser.esta_dormido { 1 } else { 3 };
            let energia_perdida = (energia_perdida_por_hora * segundos as u32) / 3600;
            ser.energia = ser.energia.saturating_sub(energia_perdida);
    
            // 🔹 Reducimos el tiempo de vida restante
            ser.tiempo_de_vida_restante = ser.tiempo_de_vida_restante.saturating_sub(segundos);

            ser.ultima_actualizacion = exec::block_timestamp();
            // 🔹 Si la energía llega a 0, el ser se duerme automáticamente
            if ser.energia == 0 {
                ser.esta_dormido = true;
            }
    
            // 🔹 Si se acaba su tiempo de vida, muere
            if ser.tiempo_de_vida_restante == 0 {
                ser.esta_vivo = false;
                ser.estado_animo = "Muerto".to_string();
                continue;
            }
    
            // 🔹 Actualizamos el estado de ánimo basado en energía y alimentación
            ser.estado_animo = if ser.energia > 80 {
                "Feliz".to_string()
            } else if ser.energia < 20 && !ser.esta_dormido {
                "Cansado".to_string()
            } else if ser.ultima_comida > segundos && (ser.ultima_comida - segundos) >= 21600 { // 6 horas en segundos
                "Hambriento".to_string()
            } else {
                "Neutral".to_string()
            };
        }
    }

    pub fn enviar_mensaje(&mut self, ser_id: u32, mensaje: String, respuesta: String) -> Events {
        let state = State::state_mut();
        let ser = match state.seres.get_mut(&ser_id) {
            Some(ser) => ser,
            None => return Events::SerNoExiste,
        };
    
        // Verificar que solo el dueño pueda interactuar
        if msg::source() != ser.owner {
            return Events::AccionNoPermitida;
        }
    
        if !ser.esta_vivo {
            return Events::SerMuerto;
        }
    
        // Actualizar la fecha de última actualización
        ser.ultima_actualizacion = exec::block_timestamp();
    
        // Limitar el historial de chat a 10 mensajes
        if ser.historial_chat.len() >= 10 {
            let len = ser.historial_chat.len();
            let num_to_remove = (len * 9 + 9) / 10; // Redondeo hacia arriba sin flotantes
            ser.historial_chat.drain(0..num_to_remove);
        }
    
        ser.historial_chat.push(format!("Tú: {}", mensaje));
        let respuesta = Self::generar_respuesta(respuesta);
        ser.historial_chat.push(format!("{}: {}", ser.name, respuesta.clone()));
    
        // Otorgar experiencia
        ser.experiencia += 3;
        self.verificar_subida_nivel(ser);
    
        Events::MensajeRecibido(ser_id, respuesta)
    }

    fn verificar_subida_nivel(&self, ser: &mut SerAutonomo) {
        if ser.experiencia >= EXPERIENCIA_PARA_SUBIR_NIVEL {
            ser.nivel += 1;
            ser.experiencia = 0;
            // Mejorar atributos al subir de nivel
            ser.atributos.salud_base += 10;
            ser.atributos.ataque_base += 2;
            ser.atributos.defensa_base += 1;
            if let sails_rs::Some(Clase::Mago) | sails_rs::Some(Clase::Curandero) = ser.clase {
                ser.atributos.mana_base += 5;
            }
        }
    }

    fn generar_respuesta(respuesta: String) -> String {
        return respuesta
    }

    fn generar_respuesta_automatica(ser_receptor: &SerAutonomo, mensaje: &str) -> String {
        // Lógica para generar una respuesta automática basada en el estado del receptor
        if ser_receptor.esta_dormido {
            return "Zzz... Estoy durmiendo.".to_string();
        }
    
        if ser_receptor.energia < 20 {
            return "Estoy cansado, hablamos luego.".to_string();
        }
    
        // Respuesta predeterminada
        format!("Hola, soy {}. Me dijiste: '{}'", ser_receptor.name, mensaje)
    }

    pub fn obtener_estado(&self, ser_id: u32) -> Option<state::SerAutonomo> {
        State::state_ref().seres.get(&ser_id).cloned()
    }

    pub fn obtener_todos_los_seres(&self) -> Vec<(u32, SerAutonomo)> {
        let state = State::state_ref();
        state.seres.iter().map(|(id, ser)| (*id, ser.clone())).collect()
    }

    pub fn obtener_matching(&self) -> Events {
        obtener_estado_matching()
    }




    pub fn importar_seres(&mut self, seres: Vec<(u32, SerAutonomo)>) -> Events {
            let state = State::state_mut();
    
            for (id, ser) in seres {
                // Verificar si el ID ya existe en el nuevo contrato
                if state.seres.contains_key(&id) {
                    continue; // O puedes manejar esto de otra manera, como devolver un error
                }
    
                // Insertar el ser en el nuevo contrato
                state.seres.insert(id, ser);
            }
    
            Events::SeresImportados
        }
        // Buscar pelea (modo invitación o aleatorio)
        pub fn buscar_pelea(&mut self, ser_id: u32, modo: ModoMatching) -> Events {
            buscar_pelea(ser_id, modo)
        }
    
        // Aceptar invitación de pelea
        pub fn aceptar_invitacion(&mut self, ser_invitado: u32) -> Events {
            aceptar_invitacion(ser_invitado)
        }
    
        // Atacar en una pelea
        pub fn atacar_en_pelea(&mut self, ser_id: u32, accion: Accion) -> Events {
            realizar_accion_en_pelea(ser_id, accion)
        }

        pub fn agregar_objeto(&mut self, ser_id: u32, objeto: Objeto) -> Events {
            let state = State::state_mut();
            let ser = match state.seres.get_mut(&ser_id) {
                Some(ser) => ser,
                None => return Events::SerNoExiste,
            };
    
            // Verificar que solo el dueño pueda agregar objetos
            if msg::source() != ser.owner {
                return Events::AccionNoPermitida;
            }
    
            ser.inventario.agregar_objeto(objeto.clone());
            Events::ObjetoAgregado(ser_id, objeto)
        }
    
        // Equipar un objeto en un ser
        pub fn equipar_objeto(&mut self, ser_id: u32, objeto_id: u32) -> Events {
            let state = State::state_mut();
            let ser = match state.seres.get_mut(&ser_id) {
                Some(ser) => ser,
                None => return Events::SerNoExiste,
            };
    
            // Verificar que solo el dueño pueda equipar objetos
            if msg::source() != ser.owner {
                return Events::AccionNoPermitida;
            }
    
            match ser.inventario.equipar_objeto(objeto_id, &mut ser.atributos, ser.clase.clone()) {
                Ok(_) => Events::ObjetoEquipado(ser_id, objeto_id),
                Err(e) => Events::Error(e),
            }
        }
    
        // Desequipar un objeto de un ser
        pub fn desequipar_objeto(&mut self, ser_id: u32, objeto_id: u32) -> Events {
            let state = State::state_mut();
            let ser = match state.seres.get_mut(&ser_id) {
                Some(ser) => ser,
                None => return Events::SerNoExiste,
            };
    
            // Verificar que solo el dueño pueda desequipar objetos
            if msg::source() != ser.owner {
                return Events::AccionNoPermitida;
            }
    
            match ser.inventario.desequipar_objeto(objeto_id, &mut ser.atributos) {
                Ok(_) => Events::ObjetoDesequipado(ser_id, objeto_id),
                Err(e) => Events::Error(e),
            }
        }
    
        // Usar una poción en un ser
        pub fn usar_pocion(&mut self, ser_id: u32, pocion_id: u32) -> Events {
            let state = State::state_mut();
            let ser = match state.seres.get_mut(&ser_id) {
                Some(ser) => ser,
                None => return Events::SerNoExiste,
            };
    
            // Verificar que solo el dueño pueda usar pociones
            if msg::source() != ser.owner {
                return Events::AccionNoPermitida;
            }
    
            match ser.inventario.usar_pocion(pocion_id, &mut ser.atributos) {
                Ok(_) => Events::PocionUse(ser_id, pocion_id),
                Err(e) => Events::Error(e),
            }
        }
    
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Events {
    SerCreado(u32),
    SerAlimentado(u32),
    SerDurmiendo(u32),
    SerDespierto(u32),
    SerMuerto,
    SerNoExiste,
    SerNoTieneHambre,
    SerSocializo(u32, u32),
    SerEnfermo(u32, String),
    SerCurado(u32),
    MensajeRecibido(u32, String),
    AccionNoPermitida,
    SubioDeNivel(u32, u32),
    SeresImportados,
    MensajeEnviado(u32, u32, String),
    PeleaIniciada(u32, u32),
    InvitacionEnviada(u32, u32),
    InvitacionNoEncontrada,
    BuscandoPelea(u32),
    AtaqueRealizado(u32, u32, u32),
    PeleaTerminada(u32, u32),
    NoEsTuTurno,
    PeleaNoEncontrada,
    HabilidadNoDisponible,
    HabilidadYaUsada,
    ManaInsuficiente,
    EstadoMatching {
        cola_aleatoria: Vec<u32>,
        invitaciones: Vec<(u32, u32)>,
        peleas_activas: Vec<Pelea>,
    },
    ObjetoAgregado(u32, Objeto), // (ser_id, objeto)
    ObjetoEquipado(u32, u32),    // (ser_id, objeto_id)
    ObjetoDesequipado(u32, u32), // (ser_id, objeto_id)
    PocionUse(u32, u32),       // (ser_id, pocion_id)
    Error(String),               // Mensaje de error
    PocionUsada(u32, Pocion),      // (ser_id, pocion_usada)
    PocionNoEncontrada,            // No se encontró la poción en el inventario            // No hay suficiente mana para usar la habilidad

}

