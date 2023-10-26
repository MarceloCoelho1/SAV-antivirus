use std::io::{Read, self};
use std::fs::{File, self};
use std::path::{Path};
use sha2::{Digest, Sha256};

fn main() {
    println!("SAV, antivirus");
    let path = Path::new("/home/marcelo/Desktop/side-projects/sav/src");
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
                let _ = signature_detection(&path)?;
            }
        }
    } else {
        eprintln!("This path is not a directory");
    }
    Ok(())
}



// signature-based detection

fn signature_detection(file_path: &Path) -> io::Result<()> {
    let virus_hash_example = "2898a07b2cf23dda8530b14b6aa522e67b002886d170c02219acc3598fdb50f3";

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
                println!("{}", file_hash);
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