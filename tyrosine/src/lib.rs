pub mod genome;
pub mod phenotype;
pub mod species;
pub mod population;



pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::population::Population;

    use super::*;

    #[test]
    fn generate_population() {
        let pop = Population::new(100, 7, 1000);
    }

    #[test]
    fn population_size() {
        let pop = Population::new(100, 7, 2000);
        assert_eq!(pop.population_size, 2000, "Population size.");
    }

    #[test]
    fn population_evolve() {
        let mut pop = Population::new(100, 7, 10);
        let species_pre = pop.species.clone();
        pop.evolve(&vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
        let species_post = pop.species.clone();
        if species_pre.len() != species_post.len() {
            return; // something has changed, so evolution has taken place
        }
        for (pre, post) in species_pre.iter().zip(species_post.iter()) {
            if pre.members.len() != post.members.len() {
                return; // something has changed, so evolution has taken place
            }
            for (phen_pre, phen_post) in pre.members.iter().zip(post.members.iter()) {
                if phen_pre.genome != phen_post.genome {
                    return; // something has changed, so evolution has taken place
                }
            }
        }
        assert!(false, "Population should be different after evolving.");
    }
}


