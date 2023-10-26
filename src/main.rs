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
    let virus_hash_example = "82b1b06a8e5863f3ac23a93e5810cb1850e9d0783d0f0fb905bd797dd1e4fc6b";

    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    let file_hash = create_signature_hash(&buffer);
  

    if file_hash == virus_hash_example {
        println!("{} <- virus, removing this file", file_hash);
        fs::remove_file(file_path);
        
    } else {
        println!("{}", file_hash);
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