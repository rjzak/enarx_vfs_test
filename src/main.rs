use std::fs;

pub const ES256: &[u8] = b"\x00\x00\x00\x07";

fn list_directory(path: &str) {
    let paths = match fs::read_dir("/key") {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to list {path}: {e}");
            return;
        }
    };
    println!("Reading {path}");
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}

fn main() {
    println!("Testing key subsystem...");
    list_directory("/key");

    println!("Generating a Elliptic Curve P256 keypair");
    fs::write("/key/generate", ES256).unwrap();
    let uuid = fs::read("/key/generate").unwrap();
    let uuid = String::from_utf8(uuid).unwrap();
    println!("Key generated, UUID: {uuid}");

    println!("Re-Reading /key");
    list_directory("/key");
}
