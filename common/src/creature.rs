use crate::genome::Genome;



/* =====================
         TRAITS
===================== */


pub trait Creature {
    fn new() -> Self;
    fn from_genome(genome: Genome) -> Self;
}



/* =====================
        STRUCTS
===================== */




