use crate::creature::Creature;
use std::{rc::Rc, cell::RefCell};


/// Generic trait for all generation manager types
pub trait GenerationManager<'a, T:Creature> {
    fn evolve(rankings: &Vec<u32>);
    fn get_population() -> Vec<&'a T>;
}


/// A generation manager with references to creatures
pub struct RefGenerationManager<T: Creature> {
    pub population: Vec<Rc<RefCell<T>>>,
    pub population_size: u32,
    pub input_count: u32,
    pub output_count: u32,

    //population_size: usize,
    //input_count: usize, //true input count (excluding bias)
    //output_count: usize,
    //population: Vec<BCreature>,
    //reproduction_helper: ReproductionHelper,
}
impl<'a, T:Creature> GenerationManager<'a, T> for RefGenerationManager<T> {
    fn evolve(rankings: &Vec<u32>) {
    }

    fn get_population() -> Vec<&'a T> {
        Vec::new()
    }
}


/// A generation manager with contiguous creatures
pub struct ContigGenerationManager<T> {
    pub population: Vec<T>,
    pub population_size: u32,
    pub input_count: u32,
    pub output_count: u32,

}
impl<'a, T:Creature> GenerationManager<'a, T> for ContigGenerationManager<T> {
    fn evolve(rankings: &Vec<u32>) {
    }

    fn get_population() -> Vec<&'a T> {
        Vec::new()
    }
}


