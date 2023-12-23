use crate::creature::Creature;
use std::{rc::Rc, cell::RefCell};


/* =====================
         TRAITS
===================== */


/// Generic trait for all generation manager types
pub trait GenerationManager<T:Creature+Clone> {
    fn with_creatures(&mut self, creatures: Vec<T>);
    fn get_population(&self) -> Vec<T>;
    fn evolve(rankings: &Vec<u32>);
}



/* =====================
        STRUCTS
===================== */


/// A generation manager with references to creatures
pub struct RefGenerationManager<T: Creature> {
    pub population: Vec<Rc<RefCell<T>>>,
    pub population_size: usize,
    pub input_count: u32,
    pub output_count: u32,
}
impl<T:Creature+Clone> GenerationManager<T> for RefGenerationManager<T> {
    fn with_creatures(&mut self, creatures: Vec<T>) {
        self.population = creatures
            .into_iter()
            .map(|x| Rc::new(RefCell::new(x)))
            .collect();
    }


    fn get_population(&self) -> Vec<T> {
        self.population.iter().map(|x| x.borrow().clone()).collect()
    }


    fn evolve(rankings: &Vec<u32>) {
    }

}


/// A generation manager with contiguous creatures
pub struct ContigGenerationManager<T: Creature> {
    pub population: Vec<T>,
    pub population_size: usize,
    pub input_count: u32,
    pub output_count: u32,

}
impl<T:Creature+Clone> GenerationManager<T> for ContigGenerationManager<T> {
    fn with_creatures(&mut self, creatures: Vec<T>) {
        self.population = creatures;
    }


    fn get_population(&self) -> Vec<T> {
        self.population.iter().map(|x| x.clone()).collect()
    }


    fn evolve(rankings: &Vec<u32>) {
    }

}



