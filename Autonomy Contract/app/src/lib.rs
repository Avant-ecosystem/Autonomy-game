#![no_std]

// necesary crates
use sails_rs::prelude::*;

// import our modules 
pub mod states;
pub mod services;

// Import service to be used for the program
use services::service::SerAutonomoService;

pub struct Program;

#[program]
impl Program {
    // Application constructor (it is an associated function)
    // It can be called once per application lifetime.
    pub fn new() -> Self {
        // Init the state
        SerAutonomoService::seed();

        Self
    }

    
    #[route("Autonomy")]
    pub fn service_svc(&self) -> SerAutonomoService {
        SerAutonomoService::new()
    }
}