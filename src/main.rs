use std::io;
use std::fs;
use std::path::{Path};

fn main() {
    println!("SAV, antivirus");
    let path = Path::new("directory_path");
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


