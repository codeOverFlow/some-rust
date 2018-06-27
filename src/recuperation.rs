use std::io::stdin;
use std::io::stdout;
use std::io::Write;

pub fn recuperation_de_la_saisie_utilisateur() -> Option<String> {
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
