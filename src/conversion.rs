use std::str::FromStr;

pub fn conversion_de_la_saisie_utilisateur(saisie: Option<String>) -> Option<usize> {
    if let Some(saisie_string) = saisie {
        match usize::from_str(saisie_string.trim()) {
            Ok(valeur) => Some(valeur),
            Err(_) => None,
        }
    } else {
        None
    }
}
