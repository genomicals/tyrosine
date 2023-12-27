use std::{fs::{File, self}, os::unix::fs::FileExt, io::Write};

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
    fn configure(&mut self, population_size: u32, handling: Handling, selection: Selection);
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

    pub population_size: u32,
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
    fn configure(&mut self, population_size: u32, handling: Handling, selection: Selection) {
        self.population_size = population_size;
        self.handling = handling;
        self.selection = selection;
    }


    /// Completely reset the generation manager.
    fn fresh_population(&mut self) {
        let mut new_pop = Vec::with_capacity(self.population_size as usize);
        for _ in 0..self.population_size {
            let genome = Genome::new_random(self.input_size, self.output_size, &mut self.rng);
            new_pop.push(AtomicCreature::from_genome(genome, self.input_size, self.output_size));
        }
        // CONTINUE TO WRITE THIS, TODO
        todo!()
    }


    fn load_generation(&mut self, file: &str) -> Result<(), TyrosineError> {
        //let bytes: Vec<u8> = fs::read(file)
        //    .map_err(|_| TyrosineError::CouldntReadFile)?
        //    .iter()
        //    .rev()
        //    .map(|x| *x)
        //    .collect();
        let bytes: Vec<u8> = fs::read(file).map_err(|_| TyrosineError::CouldntReadFile)?;
        let splits: Vec<&[u8]> = bytes.split(|x| *x == b'\n').collect();

        // process config
        let config = *splits.get(0).ok_or(TyrosineError::InvalidFileFormat)?;
        if config.len() != 12 { //invalid format
            return Err(TyrosineError::InvalidFileFormat);
        }
        let chunked_config: Vec<&[u8]> = config.chunks_exact(4).collect();
        if chunked_config.len() != 3 {
            return Err(TyrosineError::InvalidFileFormat);
        }
        let mut buf: [u8; 4] = [0; 4];
        buf[0] = chunked_config[0][0];
        buf[1] = chunked_config[0][1];
        buf[2] = chunked_config[0][2];
        buf[3] = chunked_config[0][3];
        let new_input_size = u32::from_le_bytes(buf);
        buf[0] = chunked_config[1][0];
        buf[1] = chunked_config[1][1];
        buf[2] = chunked_config[1][2];
        buf[3] = chunked_config[1][3];
        let new_output_size = u32::from_le_bytes(buf);
        buf[0] = chunked_config[2][0];
        buf[1] = chunked_config[2][1];
        buf[2] = chunked_config[2][2];
        buf[3] = chunked_config[2][3];
        let new_population_size = u32::from_le_bytes(buf);

        // if no population generate a new one
        if splits.len() == 1 {
            self.input_size = new_input_size;
            self.output_size = new_output_size;
            self.population_size = new_population_size;
            self.fresh_population();
            return Ok(());
        }

        // process genomes
        if new_population_size != (splits.len() - 1) as u32 {
            return Err(TyrosineError::InvalidFileFormat);
        }
        let mut genomes = Vec::with_capacity(new_population_size as usize);
        for i in 1..splits.len() {
            genomes.push(Genome::from_bytes(splits[i]).ok_or(TyrosineError::InvalidGenomeFormat)?);
        }
        //let genomes: Vec<Genome> = splits[1..splits.len()]
        //    .iter()
        //    .map(|x| Genome::from_bytes(x).ok_or(TyrosineError::InvalidGenomeFormat)?)
        //    .collect();
        let mut new_population = Vec::with_capacity(new_population_size as usize);
        for genome in genomes {
            let creature = T::from_genome(genome, new_input_size, new_output_size) //generic creature trait used
                .ok_or(TyrosineError::InvalidGenome)?;
            new_population.push(creature);
        }

        // save to self
        self.input_size = new_input_size;
        self.output_size = new_output_size;
        self.population_size = new_population_size;
        self.population = new_population;

        Ok(())
    }


    fn save_generation(&self, file: &str, rankings: &[u32]) -> Result<(), TyrosineError> {
        // TODO CREATE A RANKING FUNCTION AND CALL IT IN THIS FUNCTION
        let mut file = File::create(file).map_err(|_| TyrosineError::CouldntCreateFile)?;
        let mut buf = Vec::new();
        buf.extend(self.input_size.to_le_bytes()); //push config
        buf.extend(self.output_size.to_le_bytes());
        buf.extend(self.population_size.to_le_bytes());
        for i in 0..self.population_size as usize {
            buf.push(b'\n');
            buf.extend(&self.population[i].get_genome().as_bytes()); //push this genome
        }
        file.write_all(&buf).map_err(|_| TyrosineError::CouldntWriteFile)?;
        Ok(())
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





