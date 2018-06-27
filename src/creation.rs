use rand::prelude::*;

pub fn creation_de_la_valeur_a_deviner(min: usize, max: usize) -> usize {
    thread_rng().gen_range(min, max)
}
