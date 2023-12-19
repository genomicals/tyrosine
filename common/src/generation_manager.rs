use crate::creature::Creature;
use std::{rc::Rc, cell::RefCell};


/// Generic trait for all generation manager types
pub trait GenerationManager<'a, T:Creature> {
    fn with_creatures(&mut self, creatures: Vec<T>);
    fn get_population(&self) -> Vec<&'a T>;
    fn evolve(rankings: &Vec<u32>);
}


/// A generation manager with references to creatures
pub struct RefGenerationManager<T: Creature> {
    pub population: Vec<Rc<RefCell<T>>>,
    pub population_size: usize,
    pub input_count: u32,
    pub output_count: u32,
}
impl<'a, T:Creature> GenerationManager<'a, T> for RefGenerationManager<T> {
    fn with_creatures(&mut self, creatures: Vec<T>) {
        self.population = creatures
            .into_iter()
            .map(|x| Rc::new(RefCell::new(x)))
            .collect();
    }


    fn get_population(&self) -> Vec<&'a T> {

        //let x = self.population[0].clone();
        //let y = x.borrow();
        //let z = &*y;

        self.population.iter().map(|x| x.borrow().into()).collect()
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
impl<'a, T:Creature> GenerationManager<'a, T> for ContigGenerationManager<T> {
    fn with_creatures(&mut self, creatures: Vec<T>) {
        self.population = creatures;
    }
    fn get_population(&self) -> Vec<&'a T> {
        let mut ret = Vec::with_capacity(self.population_size);
        for c in &self.population {
            ret.push(c);
        }
        ret
        //self.population.iter().map(|x| x.into()).collect()
    }
    fn evolve(rankings: &Vec<u32>) {
    }

}


