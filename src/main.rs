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

fn jeu(valeur_a_deviner: usize, mut nombre_de_coup: usize) {
    while nombre_de_coup > 0 {
        println!("{} coups restant", nombre_de_coup);
        if let Some(valeur_saisie) = recuperation_de_la_saisie_utilisateur() {
            match valeur_a_deviner.cmp(&valeur_saisie) {
                Ordering::Greater => println!("Plus grand"),
                Ordering::Equal => {
                    println!("Gagné !");
                    break;
                }
                Ordering::Less => println!("Plus petit"),
            }
        } else {
            panic!("Erreur lors de la saisie utilisateur !")
        }
        nombre_de_coup -= 1;
    }
}

fn main() {
    let valeur_a_deviner = creation_de_la_valeur_a_deviner(0, 10);
    jeu(valeur_a_deviner, 10);
}
