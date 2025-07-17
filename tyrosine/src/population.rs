use std::collections::HashMap;
use rand::seq::IndexedRandom;

use crate::{genome::{Genome, GlobalInnovator}, phenotype::Phenotype, species::{Species, SpeciesCounter}};



pub struct Population {
    pub innovator: GlobalInnovator,
    pub species_counter: SpeciesCounter,
    pub population_size: usize,
    pub species: Vec<Species>,
}
impl Population {
    /// Create a new population of genomes
    pub fn new(num_inputs: usize, num_outputs: usize, population_size: usize) -> Self {
        let mut innovator = GlobalInnovator::new();
        let mut species_counter = SpeciesCounter::new();

        // initialize
        let population = (0..population_size).into_iter()
            .map(|_| Genome::new(num_inputs, num_outputs))
            .collect::<Vec<Genome>>();

        // mutate
        let mut innovations = HashMap::new(); //ensure innovation numbers are reused
        let mutated_population = population.into_iter()
            .map(|genome| Phenotype::from_mutation(&genome, &mut innovator, &mut innovations))
            .collect::<Vec<Phenotype>>();

        // assign species
        let mut rng = rand::rng();
        let chosen = mutated_population.choose(&mut rng).unwrap(); //safe unwrap
        let mut species = vec![Species::new(&chosen.genome, species_counter.next())];
        Species::sort_species(&mut species, mutated_population, &mut species_counter);
        
        Population {
            innovator,
            species_counter,
            population_size,
            species,
        }
    }


    /// Retrieve all specimens for fitness evaluation
    pub fn get_specimens(&mut self) -> Vec<&mut Phenotype> {
        let specimens = self.species.iter_mut()
            .flat_map(|s| &mut s.members)
            .collect::<Vec<&mut Phenotype>>();
        specimens
    }


    /// Evolve the population by one generation
    /// NOTE: the order of specimens received to calculate fitness is the same order here
    pub fn evolve(&mut self, fitnesses: Vec<f64>) {
        let mut new_innovations: HashMap<(usize, usize), usize> = HashMap::new(); //ensure duplicate innovations get the same innov number
    }
}
