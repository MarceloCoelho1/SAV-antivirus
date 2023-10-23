use std::io;
use std::fs;
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
    // let virus_hash = "c0535e4be2b79ffd93291305436bf889314e4a3faec05ecffcbb7df31ad9e51a";
    let content = fs::read_to_string(file_path).unwrap();

    let file_hash = create_signature_hash(&content);
    println!("{}", file_hash);

    Ok(())
}


fn create_signature_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();
    let formated_hash = format!("{:x}", result);
    
    formated_hash
    
}