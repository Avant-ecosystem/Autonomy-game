// objetos.rs

use sails_rs::{
    prelude::*,
    gstd::msg,
};
use sails_rs::gstd::exec;
use crate::services::combat::{Atributos,Clase};

#[derive(Encode, Decode, TypeInfo, Clone,PartialEq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Pocion {
    Salud(u32), // Cantidad de salud que restaura
    Mana(u32),  // Cantidad de mana que restaura
}

// Definir los tipos de objetos
#[derive(Encode, Decode, TypeInfo, Clone,PartialEq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum TipoObjeto {
    Arma,
    Armadura,
    Accesorio,
    Pocion, // Nuevo tipo para pociones
}

// Definir los objetos
#[derive(Encode, Decode, TypeInfo, Clone,PartialEq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Objeto {
    pub id: u32,
    pub nombre: String,
    pub tipo: TipoObjeto,
    pub atributos: Atributos, // Stats que aumenta el objeto
    pub clase_requerida: Option<Clase>, // Clase requerida para equipar el objeto
    pub cantidad: u32, // Cantidad disponible (solo para pociones)
    pub pocion: Option<Pocion>, // Campo para identificar si es una poción
}

impl Objeto {
    pub fn new(id: u32, nombre: String, tipo: TipoObjeto, atributos: Atributos, clase_requerida: Option<Clase>, cantidad: u32, pocion:Option<Pocion>) -> Self {
        Self {
            id,
            nombre,
            tipo,
            atributos,
            clase_requerida,
            cantidad,
            pocion
        }
    }
}

// Definir el inventario de un personaje
#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Inventario {
    pub objetos_equipables: Vec<Objeto>, // Armaduras, armas, accesorios
    pub pociones: Vec<Objeto>, // Pociones (consumibles)
}

impl Inventario {
    pub fn new() -> Self {
        Self {
            objetos_equipables: Vec::new(),
            pociones: Vec::new(),
        }
    }

    // Agregar un objeto al inventario
    pub fn agregar_objeto(&mut self, objeto: Objeto) {
        match objeto.tipo {
            TipoObjeto::Pocion => self.pociones.push(objeto),
            _ => self.objetos_equipables.push(objeto),
        }
    }

    // Equipar un objeto (solo para armas, armaduras y accesorios)
    pub fn equipar_objeto(&mut self, objeto_id: u32, atributos_personaje: &mut Atributos, clase_personaje: Option<Clase>) -> Result<(), String> {
        if let Some(objeto) = self.objetos_equipables.iter().find(|o| o.id == objeto_id) {
            // Verificar si el personaje cumple con los requisitos de clase
            if let Some(clase_requerida) = &objeto.clase_requerida {
                if clase_personaje != Some(clase_requerida.clone()) {
                    return Err("El personaje no cumple con los requisitos de clase para equipar este objeto.".to_string());
                }
            }

            // Aplicar los stats del objeto al personaje
            atributos_personaje.salud_base += objeto.atributos.salud_base;
            atributos_personaje.ataque_base += objeto.atributos.ataque_base;
            atributos_personaje.defensa_base += objeto.atributos.defensa_base;
            atributos_personaje.mana_base += objeto.atributos.mana_base;

            Ok(())
        } else {
            Err("Objeto no encontrado en el inventario.".to_string())
        }
    }

    // Desequipar un objeto (solo para armas, armaduras y accesorios)
    pub fn desequipar_objeto(&mut self, objeto_id: u32, atributos_personaje: &mut Atributos) -> Result<(), String> {
        if let Some(objeto) = self.objetos_equipables.iter().find(|o| o.id == objeto_id) {
            // Restar los stats del objeto al personaje
            atributos_personaje.salud_base -= objeto.atributos.salud_base;
            atributos_personaje.ataque_base -= objeto.atributos.ataque_base;
            atributos_personaje.defensa_base -= objeto.atributos.defensa_base;
            atributos_personaje.mana_base -= objeto.atributos.mana_base;

            Ok(())
        } else {
            Err("Objeto no encontrado en el inventario.".to_string())
        }
    }

    // Usar una poción (consumible)
    pub fn usar_pocion(&mut self, pocion_id: u32, atributos_personaje: &mut Atributos) -> Result<(), String> {
        if let Some(pocion) = self.pociones.iter_mut().find(|p| p.id == pocion_id) {
            if pocion.cantidad > 0 {
                // Aplicar el efecto de la poción
                match pocion.tipo {
                    TipoObjeto::Pocion => {
                        atributos_personaje.salud_base += pocion.atributos.salud_base;
                        atributos_personaje.mana_base += pocion.atributos.mana_base;
                    }
                    _ => return Err("Este objeto no es una poción.".to_string()),
                }

                // Reducir la cantidad de pociones
                pocion.cantidad -= 1;

                // Si la cantidad llega a 0, eliminar la poción del inventario
                if pocion.cantidad == 0 {
                    self.pociones.retain(|p| p.id != pocion_id);
                }

                Ok(())
            } else {
                Err("No quedan pociones de este tipo.".to_string())
            }
        } else {
            Err("Poción no encontrada en el inventario.".to_string())
        }
    }
}