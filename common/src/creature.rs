use crate::genome::Genome;

pub trait Creature {
    fn new() -> Self;
    fn from_genome(genome: Genome) -> Self;
}



