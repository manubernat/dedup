use std::collections::HashMap;
//use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    // Répertoire de référence en dur (à lire sur la ligne de commande)
    let rep_reference = std::path::Path::new("C:\\Users\\frag\\seadrive_root\\Emmanuel\\Mes bibliothèques\\Photos\\_Animaux");
    //println!("Répertoire de référence : {rep_reference}");

    let hashes: HashMap<&Path, String> = HashMap::new();

    analyse_repertoire(&rep_reference, &hashes);

//    let mut file_hashes: HashMap<String, String> = HashMap::new();
//    for (hash, file) in &file_hashes {
//        println!("{hash} -> {file}");
//    }
//    println!("{} element(s).", file_hashes.len());

}

fn analyse_repertoire( repertoire: &Path, hashes: &HashMap<&Path, String> ) {
    // Itére le contenu du répertoire
    if let Ok(dir_entries) = fs::read_dir(repertoire) {
        for dir_entry in dir_entries {
            if let Ok(dir_entry) = dir_entry {
                if let Ok(file_type) = dir_entry.file_type() {
                    if file_type.is_dir() {
                        // Parcours récursif
                        analyse_repertoire(dir_entry.path().as_path(), hashes);
                    } else {
                        // Ajout dans la HashMap
                        println!("Fichier : {}",dir_entry.path().display())  ;
                    }
                }
            }
        }
    }
}