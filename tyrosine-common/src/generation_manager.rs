use std::fs::File;

use rand::rngs::ThreadRng;

use crate::{creature::{Creature, AtomicCreature}, errors::TyrosineError, genome::Genome};
//use std::{rc::Rc, cell::RefCell};


/* =====================
         TRAITS
===================== */

/// How to handle creatures from the current generation.
pub enum Handling {
    /// All generations must have unique creatures.
    UniqueCreatures,
    /// Creatures may continue existing into the next generation.
    PersistentCreatures,
}


/// How to select which creatures contribute to the next generation.
pub enum Selection {
    /// Set a hard threshold from 0.1 to 0.9.
    Threshold(f32),
    /// Smooth gradient with the center set from 0.1 to 0.9.
    Gradient(f32),
}


/// Generic trait for all generation manager types
pub trait GenerationManager<T:Creature+Clone> {
    fn configure(&mut self, population_size: usize, handling: Handling, selection: Selection);
    fn fresh_population(&mut self);
    fn load_generation(&mut self, file: &str) -> Result<(), TyrosineError>;
    fn save_generation(&self, file: &str, rankings: &[u32]) -> Result<(), TyrosineError>;
    fn get_population(&self) -> Vec<T>;
    fn evolve(&mut self, rankings: &[u32]) -> Result<(), TyrosineError>;
}


/* =====================
        STRUCTS
===================== */


/// A generation manager with contiguous creatures
pub struct ContigGenerationManager<T: Creature> {
    pub population: Vec<T>,
    pub input_size: u32,
    pub output_size: u32,
    pub rng: ThreadRng,

    pub population_size: usize,
    pub handling: Handling,
    pub selection: Selection,
}
impl<T:Creature+Clone> ContigGenerationManager<T> {
    /// Creates a contiguous generation manager.
    ///
    /// Default configuration:
    /// population-size: 500
    /// handling: persistent
    /// selection: gradient 0.5
    pub fn new(input_count: u32, output_count: u32) -> Self {
        ContigGenerationManager {
            population: Vec::new(),
            input_size: input_count,
            output_size: output_count,
            rng: rand::thread_rng(),

            population_size: 500,
            handling: Handling::PersistentCreatures,
            selection: Selection::Gradient(0.5),
        }
    }
}
impl<T:Creature+Clone> GenerationManager<T> for ContigGenerationManager<T> {
    fn configure(&mut self, population_size: usize, handling: Handling, selection: Selection) {
        self.population_size = population_size;
        self.handling = handling;
        self.selection = selection;
    }


    /// Completely reset the generation manager.
    fn fresh_population(&mut self) {
        let mut new_pop = Vec::with_capacity(self.population_size);
        for _ in 0..self.population_size {
            let genome = Genome::new_random(self.input_size, self.output_size, &mut self.rng);
            new_pop.push(AtomicCreature::from_genome(&genome, self.input_size, self.output_size));
        }
    }


    fn load_generation(&mut self, file: &str) -> Result<(), TyrosineError> {
        return Ok(()); //TODO temporary
    }


    fn save_generation(&self, file: &str, rankings: &[u32]) -> Result<(), TyrosineError> {
        let mut file = File::create(file).map_err(|_| TyrosineError::CouldntCreateFile)?;
        // push all important info related to the generation and not
        // evolving into the file, config can change
        return Ok(());
    }


    fn get_population(&self) -> Vec<T> {
        self.population.clone()
    }


    fn evolve(&mut self, rankings: &[u32]) -> Result<(), TyrosineError> {
        return Ok(()); //TODO temporary
    }

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





