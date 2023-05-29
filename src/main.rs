use std::collections::HashMap;
use std::{fs, io};
use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};


fn main() {
    // Répertoire de référence en dur (à lire sur la ligne de commande)
    let rep_reference = std::path::Path::new("C:\\Users\\frag\\seadrive_root\\Emmanuel\\Mes bibliothèques\\Photos\\_Animaux");

    // Calcul des hashs du répértoire de référence
    let mut hashes: HashMap<String, PathBuf> = HashMap::new();
    analyse_repertoire(&rep_reference, &mut hashes);

    // Affichage
    for (hash, file) in &hashes {
        println!("{} : {}", file.to_string_lossy(), hash);
    }
    println!("{} element(s).", hashes.len());

}

fn analyse_repertoire( repertoire: &Path, hashes: &mut HashMap<String, PathBuf> ) {
    // Itére le contenu du répertoire
    if let Ok(dir_entries) = fs::read_dir(repertoire) {
        for dir_entry in dir_entries {
            if let Ok(dir_entry) = dir_entry {
                if let Ok(file_type) = dir_entry.file_type() {
                    if file_type.is_dir() {
                        // Parcours récursif
                        analyse_repertoire(dir_entry.path().as_path(), hashes);
                    } else {
                        // Calcul du hash
                        let mut hasher = Sha256::new();
                        if let Ok(mut file) = fs::File::open(dir_entry.path()) {
                            let _bytes_written = io::copy(&mut file, &mut hasher);
                            let hash_bytes = hasher.finalize();

                                
                            // Ajout dans la HashMap
                            hashes.insert( String::from(format!("{:X}", hash_bytes)), dir_entry.path() );
                        }
                    }
                }
            }
        }
    }
}