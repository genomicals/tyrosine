use crate::{genome::GlobalInnovator, phenotype::Phenotype, species::{Species, SpeciesCounter}};



pub struct Population {
    pub innovator: GlobalInnovator,
    pub species_counter: SpeciesCounter,
    pub population_size: usize,
    pub species: Vec<Species>,
}
impl Population {
    /// Create a new population of genomes
    /// TODO: create all the creatures, mutate them, and sort them into species
    pub fn new(population_size: usize) -> Self {
        

        todo!();

        Population {
            innovator: GlobalInnovator::new(),
            species_counter: SpeciesCounter::new(),
            population_size,
            species: Vec::new(),
        };
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
        let mut new_innovations: Vec<(usize, usize)> = Vec::new(); //figure out how this is supposed to work
    }
}
