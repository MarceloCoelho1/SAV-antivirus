use std::io::{Read, self};
use std::fs::{File, self};
use std::path::{Path};
use sha2::{Digest, Sha256};

fn main() {
    println!("SAV, antivirus");
    let path = Path::new("/home/marcelo/Desktop/side-projects/sav/src/comparing/");
    visit_dirs(path);
    

    
}


fn visit_dirs(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let _ = visit_dirs(&path)?;
            } else {
                let _ = heuristic_based_detection(&path);
            }
        }
    } else {
        eprintln!("This path is not a directory");
    }
    Ok(())
}



// signature-based detection

fn signature_detection(file_path: &Path) -> io::Result<()> {
    let virus_hash_example = "3339730bc15121cf0cf2a29d74b3000bc3a5cd6caad26f305723043c4f70596b";

    match File::open(file_path) {
        Ok(mut file) => {
            let mut buffer = Vec::new();

            if let Err(err) = file.read_to_end(&mut buffer) {
                eprintln!("Error reading the file: {}", err);
                return Err(err);
            }

            let file_hash = create_signature_hash(&buffer);

            if file_hash == virus_hash_example {
                println!("{} <- virus, removing this file", file_hash);
                if let Err(err) = fs::remove_file(file_path) {
                    eprintln!("error when removing file: {}", err);
                    return Err(err);
                }
            } else {
                println!("{} {:?}", file_hash, file_path);
            }
        }
        Err(err) => {
            eprintln!("error opening file: {}", err);
            return Err(err);
        }
    }

    Ok(())
}


fn create_signature_hash(content: &Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();
    let formated_hash = format!("{:x}", result);
    
    formated_hash
    
}



// heuristic based detection


fn heuristic_based_detection(file_path: &Path) -> bool {
    println!("{:?}", file_path);
    let mut virus_file = File::open("/home/marcelo/Desktop/side-projects/sav/src/comparing/b.txt").expect("Error to reading file");
    let mut virus_buffer = Vec::new();


    if let Err(err) = virus_file.read_to_end(&mut virus_buffer) {
        eprintln!("Error reading the file: {}", err);
        return false;
    }

    

    let mut file = File::open(file_path).expect("Error to reading file");
    let mut buffer = Vec::new();

    if let Ok(file_content) = file.read_to_end(&mut buffer) {
        if virus_buffer == buffer {
            println!("is a virus");
            return true
        } else {
            let matching = virus_buffer.iter().zip(&buffer).filter(|&(virus_buffer, buffer)| virus_buffer == buffer).count();
            println!("{}", matching);

            
        }   

    } 

    false

}

