extern crate rand;

use rand::prelude::*;
use std::cmp::Ordering;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::str::FromStr;

fn creation_de_la_valeur_a_deviner(min: usize, max: usize) -> usize {
    thread_rng().gen_range(min, max)
}

fn conversion_de_la_saisie_utilisateur(saisie: String) -> Option<usize> {
    match usize::from_str(saisie.trim()) {
        Ok(valeur) => Some(valeur),
        Err(_) => None,
    }
}

fn recuperation_de_la_saisie_utilisateur() -> Option<usize> {
    // Petit affichage
    print!("Valeur: ");
    match stdout().flush() {
        Ok(_) => (),
        Err(e) => eprintln!("Erreur durant le flush de stdout: {}", e),
    }

    // Récupération de la saisie
    let mut saisie_utilisateur = String::new();
    match stdin().read_line(&mut saisie_utilisateur) {
        Ok(_) => conversion_de_la_saisie_utilisateur(saisie_utilisateur),
        Err(e) => {
            eprintln!("Erreur de saisie utilisateur: {}", e);
            None
        }
    }
}

fn main() {
    println!("Hello, world!");
}
