use std::collections::HashMap;
use std::{fs, io, env};
use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};

fn analyse_repertoire( repertoire: &Path, hashes: &mut HashMap<String, Vec<PathBuf>> ) {
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
                            let hash_str = String::from(format!("{:X}", hash_bytes));
                                
                            // Ajout dans la HashMap
                            if let Some(v) = hashes.get_mut(&hash_str) {
                                v.push(dir_entry.path());
                            } else {
                                let v = vec![ dir_entry.path() ];
                                hashes.insert(hash_str, v);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn affiche_dups( hashes: &HashMap<String, Vec<PathBuf>> ) {
    for ( hash_val, dups ) in hashes {
        if dups.len()>1 {
            println!("{} duplicas (hash = {}) :", dups.len(), hash_val);
            for dup_path in dups {
                println!("{}", dup_path.to_string_lossy());
            }
            println!("");
        }
    }
}

fn main() {
    let mut dup_detector: HashMap<String, Vec<PathBuf>> = HashMap::new();

    let args: Vec<String> = env::args().collect();
    for rep in args {
        let rep_path = Path::new(&rep);
        analyse_repertoire(&rep_path, &mut dup_detector);
    }

    affiche_dups(&dup_detector);
}

