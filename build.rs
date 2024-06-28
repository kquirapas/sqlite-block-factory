use anyhow::Result;
use std::env;
use std::fs::{create_dir, File};
use std::io::ErrorKind;

// generated by `sqlx migrate build-script`
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=path/to/Cargo.lock");

    dotenvy::dotenv()?;
    let db_folder = env::var("DATABASE_FOLDER")?;
    let db_name = env::var("DATABASE_NAME")?;
    // create DB folder in root if it doesn't yet exist
    if let Err(err) = create_dir(&db_folder) {
        if err.kind() != ErrorKind::AlreadyExists {
            panic!("{}", err);
        }
        // else skipping...
        println!(
            "The {}/ folder already exists in root. Skipping...",
            db_folder
        );
    }

    // create DB file in db/ folder
    if let Err(err) = File::create_new(format!("{}/{}", db_folder, db_name)) {
        if err.kind() != ErrorKind::AlreadyExists {
            panic!("{}", err);
        }
        // else skipping...
        println!(
            "The {} file already exists in {}/. Skipping...",
            db_name, db_folder
        );
    }
    Ok(())
}
