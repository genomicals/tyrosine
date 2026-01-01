use std::{collections::{HashMap, HashSet}, mem};
use rand::seq::{IndexedRandom, SliceRandom};
use crate::{genome::{Genome, GlobalInnovator}, phenotype::Phenotype, species::{Species, SpeciesCounter}};



pub struct Population {
    generation_number: usize,
    innovator: GlobalInnovator,
    species_counter: SpeciesCounter,
    index_cache: HashMap<usize, (usize, usize)>,
    pub species: Vec<Species>,
    pub population_size: usize,
    pub num_inputs: usize,
    pub num_outputs: usize,
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
            generation_number: 0,
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

        assert_eq!(population_counter, self.population_size, "Population size mismatch discovered during caching!");
    }


    /// Feed the input and generate an output for a particular index in the population
    pub fn activate_index(&self, idx: usize, input: &mut Vec<f64>) -> Option<Vec<f64>> {
        let pair = self.index_cache.get(&idx)?; //will fail if user provided number larger than the population size
        let phenotype = &self.species[pair.0].members[pair.1];
        Some(phenotype.activate(input))
    }


    /// Distribute a total among a certain amount of buckets
    pub fn distribute_evenly(total: usize, buckets: usize) -> Vec<usize> {
        let mut result = vec![total / buckets; buckets];
        let remainder = total % buckets;

        //use rand::seq::SliceRandom;
        let mut rng = rand::rng();

        // Randomly pick buckets to get +1
        let mut indices: Vec<_> = (0..buckets).collect();
        indices.shuffle(&mut rng);

        for &i in indices.iter().take(remainder) {
            result[i] += 1;
        }

        result
    }


    /// Evolve the population by one generation with provided fitness
    /// NOTE: the order of specimens received to calculate fitness is the same order here
    pub fn evolve(&mut self, fitnesses: &Vec<f64>) {
        //let fitness_map = (0..self.population_size).into_iter() //attach the fitness to the id (why is this necessary?)
        //    .zip(fitnesses.clone())
        //    .collect::<HashMap<usize, f64>>();

        let fitness_by_species_index = fitnesses.iter()
            .enumerate()
            .map(|(i, x)| (*self.index_cache.get(&i).unwrap(), *x))
            .collect::<Vec<((usize, usize), f64)>>();

        // refactor to a list of lists
        let species_count = fitness_by_species_index.iter()
            .map(|((x, _), _)| *x)
            .collect::<HashSet<usize>>()
            .len();
        let mut fitness_by_species = vec![vec![]; species_count];
        for ((s_i, _), fitness) in fitness_by_species_index {
            if s_i == fitness_by_species.len() {
                fitness_by_species.last_mut().unwrap().push(fitness);
            } else if s_i < fitness_by_species.len() {
                fitness_by_species[s_i].push(fitness);
            } else {
                unreachable!(); //at least it should be
            }
        }

        let mut total_fitness = 0.0;

        // sort all phenotypes within their species and calculate species fitnesses
        for (spec, fits) in self.species.iter_mut().zip(fitness_by_species) {
            let mut zipped: Vec<_> = spec.members.drain(..).zip(fits).collect();
            zipped.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap_or(std::cmp::Ordering::Less));
            let (phens, fits): (Vec<_>, Vec<_>) = zipped.into_iter().unzip();
            spec.members = phens;
            spec.species_fitness = Some(fits.iter().sum::<f64>() / fits.len() as f64);
            total_fitness += spec.species_fitness.unwrap(); //safe unwrap
        }

        // when we implement recording a generation, do it here after all the sorting is done

        // now we commence natural selection
        let mut reproductive_slots: Vec<_> = self.species.iter()
            .map(|s| (s.species_fitness.unwrap() / total_fitness * self.population_size as f64) as usize) //floors
            .collect();

        // see how many slots we have total, and adjust to ensure we have self.population_size
        let total_slots: usize = reproductive_slots.iter().sum();
        let remainder = self.population_size - total_slots;

        if remainder > 0 {
            let extra_slots = Population::distribute_evenly(remainder, self.species.len());
            for (slots, species_slots) in extra_slots.into_iter().zip(&mut reproductive_slots) {
                *species_slots += slots;
            }
        }

        // kill off species with 0 reproductive slots
        let species = mem::take(&mut self.species); //maybe this can be done differently
        let (reproductive_slots, mut species): (Vec<_>, Vec<_>) = reproductive_slots.into_iter()
            .zip(species)
            .filter(|(slots, _)| *slots != 0 as usize)
            .collect();

        // for each species, kill off 50%, choose a new type specimen, and return a new generation with the elite member
        let mut new_population = vec![];
        let mut new_innovations: HashMap<(usize, usize), usize> = HashMap::new(); //ensure duplicate innovations get the same innov number
        for (spec, slots) in species.iter_mut().zip(reproductive_slots) {
            spec.species_fitness = None; //reset this just because
            spec.members.truncate(spec.members.len() / 2); //remove half
            spec.choose_type_specimen();
            spec.populate(&mut new_population, slots, &mut self.innovator, &mut new_innovations);
        }

        // assign all phenotypes to new species
        Species::sort_species(&mut species, new_population, &mut self.species_counter);

        // remember to update cache and increment generation
        self.update_cache();
        self.generation_number += 1;
    }
}


