use crate::creature::Creature;
use std::{rc::Rc, cell::RefCell};


/* =====================
         TRAITS
===================== */


/// Generic trait for all generation manager types
pub trait GenerationManager<T:Creature+Clone> {
    fn fresh_population();
    fn load_generation(file: &str) -> Result<(), TyrosineError>;
    fn save_generation(file: &str, rankings: &[u32]);
    fn get_population(&self) -> Vec<T>;
    fn evolve(rankings: &[u32]) -> Result<(), TyrosineError>;
}



/* =====================
        STRUCTS
===================== */


/// Centralized error enum for Tyrosine.
pub enum TyrosineError {
    InvalidGenome,
    EmptyPopulation,
}


///// A generation manager with references to creatures
//pub struct RefGenerationManager<T: Creature> {
//    pub population: Vec<Rc<RefCell<T>>>,
//    pub population_size: usize,
//    pub input_count: u32,
//    pub output_count: u32,
//}
//impl<T:Creature+Clone> GenerationManager<T> for RefGenerationManager<T> {
//    fn get_population(&self) -> Vec<T> {
//        self.population.iter().map(|x| x.borrow().clone()).collect()
//    }
//
//
//    fn evolve(rankings: &[u32]) {
//    }
//
//}


/// A generation manager with contiguous creatures
pub struct ContigGenerationManager<T: Creature> {
    pub population: Vec<T>,
    pub population_size: usize,
    pub input_count: u32,
    pub output_count: u32,

}
impl<T:Creature+Clone> ContigGenerationManager<T> {
    pub fn new(population_size: usize, input_count: u32, output_count: u32) -> Self {
        ContigGenerationManager {
            population: Vec::new(),
            population_size,
            input_count,
            output_count
        }
    }
}
impl<T:Creature+Clone> GenerationManager<T> for ContigGenerationManager<T> {
    fn fresh_population() {
        
    }

    fn load_generation(file: &str) -> Result<(), TyrosineError> {
        return Ok(()); //TODO temporary
    }

    fn save_generation(file: &str, rankings: &[u32]) {
        
    }

    fn get_population(&self) -> Vec<T> {
        self.population.clone()
    }


    fn evolve(rankings: &[u32]) -> Result<(), TyrosineError> {
        return Ok(()); //TODO temporary
    }

}



