extern crate rand;

mod recuperation;
use recuperation::recuperation_de_la_saisie_utilisateur;

mod conversion;
use conversion::conversion_de_la_saisie_utilisateur;

mod creation;
use creation::creation_de_la_valeur_a_deviner;

use std::cmp::Ordering;

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
        // Possibilité de changer la valeur max
        let valeur_max =
            match conversion_de_la_saisie_utilisateur(recuperation_de_la_saisie_utilisateur()) {
                Some(valeur) => valeur,
                None => 10,
            };

        // création de la valeur à deviner
        let valeur_a_deviner = creation_de_la_valeur_a_deviner(0, valeur_max);

        // lancement du jeu
        jeu(valeur_a_deviner, 10);

        // Possibilité de rejouer
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
