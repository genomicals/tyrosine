use std::collections::HashMap;
use rand::seq::IndexedRandom;
use crate::{genome::{Genome, GlobalInnovator}, phenotype::Phenotype, species::{Species, SpeciesCounter}};



pub struct Population {
    innovator: GlobalInnovator,
    species_counter: SpeciesCounter,
    species: Vec<Species>,
    population_size: usize,
    num_inputs: usize,
    num_outputs: usize,
    index_cache: HashMap<usize, (usize, usize)>,
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
        
        let mut population = Population {
            innovator,
            species_counter,
            population_size,
            species,
            num_inputs,
            num_outputs,
            index_cache: HashMap::with_capacity(population_size),
        };
        population.update_cache(); //easy indexing

        population
    }


    /// Update index cache to speed up phenotype indexing
    pub fn update_cache(&mut self) {
        let mut population_counter = 0;
        self.index_cache.clear(); //apparently nearly no slowdown, safer this way
        
        for (i, s) in self.species.iter().enumerate() {
            self.index_cache.extend((0..s.members.len()).into_iter()
                .map(|x| (population_counter+x, (i, x))) //convert member index to (global index, (species index, member index))
            );
            population_counter += s.members.len();
        }

        assert_eq!(population_counter, 1000, "Population size mismatch during caching!");
    }


    /// Feed the input and generate an output for a particular index in the population.
    pub fn activate_index(&self, idx: usize, input: &mut Vec<f64>) -> Option<Vec<f64>> {
        let pair = self.index_cache.get(&idx)?; //will fail if user provided number larger than the population size
        let phenotype = &self.species[pair.0].members[pair.1];
        Some(phenotype.activate(input))
    }


    /// TODO
    /// Evolve the population by one generation with one fitness metric
    /// NOTE: the order of specimens received to calculate fitness is the same order here
    pub fn evolve(&mut self, fitnesses: &Vec<f64>) {
        let mut new_innovations: HashMap<(usize, usize), usize> = HashMap::new(); //ensure duplicate innovations get the same innov number
    }


    ///// TODO, unless it's pointless? the user can just make the primary one scale
    /////     larger than the secondary and sum them and use the normal evolve function
    /////
    ///// Evolve the population by one generation with two fitness metrics
    ///// The primary metric is used first, with ties broken by the second metric
    ///// NOTE: the order of specimens received to calculate fitness is the same order here
    //pub fn evolve2(&mut self, fitnesses_primary: Vec<f64>, fitnesses_secondary: Vec<f64>) {

    //}
}
