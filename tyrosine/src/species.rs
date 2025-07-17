use std::collections::{BTreeSet, HashMap};
use rand::seq::SliceRandom;
use crate::{genome::Genome, phenotype::Phenotype};



pub struct SpeciesCounter {
    pub id: usize,
}
impl SpeciesCounter {
    pub fn new() -> Self {
        SpeciesCounter { id: 0 }
    }

    /// Get the next species id and increment internally
    pub fn next(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        id
    }
}


pub struct Species {
    pub type_specimen: Genome, //may be part of the active population, or not
    pub members: Vec<Phenotype>,
    pub id: usize, //for non-crucial historical reasons
}
const C1: f64 = 1.0; //excess weight
const C2: f64 = 1.0; //disjoint weight
const C3: f64 = 0.4; //weight difference multiplier
const SPECIES_THRESHOLD: f64 = 3.0; //used to determine if two genomes are the same species
impl Species {
    /// Create a new species from a genome
    pub fn new(genome: &Genome, id: usize) -> Self {
        Species {
            type_specimen: genome.clone(),
            members: Vec::new(),
            id,
        }
    }


    /// Take phenotypes and sort them into the right species
    pub fn sort_species(species: &mut Vec<Species>, mut phenotypes: Vec<Phenotype>, species_counter: &mut SpeciesCounter) {
        let mut rng = rand::rng();
        phenotypes.shuffle(&mut rng); //delete biases here

        'phen_loop: for phenotype in phenotypes {
            let mut indices: Vec<usize> = (0..species.len()).collect();
            indices.shuffle(&mut rng); //shuffle to reduce biases (in a way that doesn't cause borrow errors)

            for i in indices {
                let cur_species = &mut species[i];
                if Species::compatibility_distance(&phenotype.genome, &cur_species.type_specimen) < SPECIES_THRESHOLD {
                    cur_species.members.push(phenotype); //push to species
                    continue 'phen_loop;
                }
            }

            // didn't match any existing species, create new species
            let mut new_species = Species::new(&phenotype.genome, species_counter.next());
            new_species.members.push(phenotype); //push this phenotype
            species.push(new_species);
        }
    }


    /// Calculates how genetically different two genomes are, using NEAT's formula:
    /// δ = c1*E/N + c2*D/N + c3*W
    /// E = excess genes, D = disjoint genes, W = avg weight diff, N = normalizer
    pub fn compatibility_distance(g1: &Genome, g2: &Genome) -> f64 {
        // map innovation numbers to genes for both genomes
        let mut g1_map = HashMap::new();
        for conn in &g1.connection_genes {
            g1_map.insert(conn.innov, conn);
        }
        let mut g2_map = HashMap::new();
        for conn in &g2.connection_genes {
            g2_map.insert(conn.innov, conn);
        }

        // all innovation numbers from both genomes combined
        let all_innovs: BTreeSet<usize> = g1_map.keys().chain(g2_map.keys()).cloned().collect();

        // declare variables to be calculated
        let mut matching = 0;
        let mut weight_diff = 0.0;
        let mut disjoint = 0;
        let mut excess = 0;

        // determine the largest innovation numbers from each genome
        let max1 = g1_map.keys().max().cloned().unwrap_or(0);
        let max2 = g2_map.keys().max().cloned().unwrap_or(0);

        // for each innovation number, classify it as matching, disjoint, or excess
        for innov in &all_innovs {
            match (g1_map.get(innov), g2_map.get(innov)) {
                (Some(a), Some(b)) => { //both genomes have this gene, matching
                    matching += 1;
                    weight_diff += (a.weight - b.weight).abs();
                }
                (Some(_), None) if *innov <= max2 => {
                    disjoint += 1;
                },
                (None, Some(_)) if *innov <= max1 => {
                    disjoint += 1;
                },
                _ => {
                    excess += 1; //it's in one genome after the other's max innov number
                },
            }
        }

        let n = all_innovs.len().max(1); //normalizer, size of larger genome (or 1 to avoid division by 0)
        let w = if matching > 0 { //average weight difference for matching genes
            weight_diff / matching as f64
        } else {
            100.0 //fallback if no matching genes, arbitrarily large number
        };

        // compatibility distance formula from NEAT paper
        C1 * (excess as f64) / n as f64 +
        C2 * (disjoint as f64) / n as f64 +
        C3 * w
    }
}








