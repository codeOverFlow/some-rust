extern crate rand;

use rand::prelude::*;
use std::cmp::Ordering;
use std::str::FromStr;
mod recuperation;
use recuperation::recuperation_de_la_saisie_utilisateur;

fn creation_de_la_valeur_a_deviner(min: usize, max: usize) -> usize {
    thread_rng().gen_range(min, max)
}

fn conversion_de_la_saisie_utilisateur(saisie: Option<String>) -> Option<usize> {
    if let Some(saisie_string) = saisie {
        match usize::from_str(saisie_string.trim()) {
            Ok(valeur) => Some(valeur),
            Err(_) => None,
        }
    } else {
        None
    }
}

fn recuperation_de_la_saisie_utilisateur() -> Option<String> {
    // Petit affichage
    print!("Valeur: ");
    match stdout().flush() {
        Ok(_) => (),
        Err(e) => eprintln!("Erreur durant le flush de stdout: {}", e),
    }

    // Récupération de la saisie
    let mut saisie_utilisateur = String::new();
    match stdin().read_line(&mut saisie_utilisateur) {
        Ok(_) => Some(saisie_utilisateur),
        Err(e) => {
            eprintln!("Erreur de saisie utilisateur: {}", e);
            None
        }
    }
}

fn jeu(valeur_a_deviner: usize, mut nombre_de_coup: isize) {
    while nombre_de_coup >= 0 {
        println!("{} coups restant", nombre_de_coup);
        if let Some(valeur_saisie) =
            conversion_de_la_saisie_utilisateur(recuperation_de_la_saisie_utilisateur())
        {
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

    if nombre_de_coup < 0 {
        println!("Perdu le nombre à deviner était: {}", valeur_a_deviner);
    }
}

fn main() {
    loop {
        let valeur_a_deviner = creation_de_la_valeur_a_deviner(0, 10);
        jeu(valeur_a_deviner, 10);
        if let Some(rejouer) = recuperation_de_la_saisie_utilisateur() {
            match rejouer.trim() {
                "O" => (),
                _ => break,
            }
        } else {
            panic!("Saisie invalide")
        }
    }
}
