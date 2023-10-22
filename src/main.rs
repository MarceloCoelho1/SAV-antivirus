use std::io;
use std::fs;
use std::path::{Path};
use sha2::{Digest, Sha256};

fn main() {
    println!("SAV, antivirus");
    // let path = Path::new("/home/marcelo/Desktop/side-projects/sav/src");
    // visit_dirs(path);
    create_signature_hash();
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





fn create_signature_hash() {
    let mut hasher = Sha256::new();

    hasher.update("Hello world!");

    let result = hasher.finalize();

    println!("hash key: {:x}", result);
}