//! Refactors names of files and directories in given destination.
//! 
//! Some features require you to set up an instance of the [`Config`], <br />
//! you can create one either with default values or from given arguments,
//! to do that check Structs - Config section.
//! 

use std::fs::{read_dir, DirEntry};
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;

mod config;
pub use config::{Config, VERSION, HELP, ILLEGAL};
mod renamer;
pub use renamer::{change_file_name, copy_change_file_name, refactor_name, remove_diacritics};


fn recurse_through(old_position: &mut PathBuf, new_position: &mut PathBuf, recursion: u8, cnfg: &Config) -> Result<(), Box<dyn Error>> {
    if old_position.is_file() && cnfg.files {
        if cnfg.copy {
            return copy_change_file_name(old_position, new_position, cnfg, recursion == 0);
        } else {
            return change_file_name(old_position, cnfg)
        }
    
    } else if old_position.is_dir() {
        if cnfg.directories {
            if cnfg.copy {
                copy_change_file_name(old_position, new_position, cnfg, recursion == 0)?
            } else {
                change_file_name(old_position, cnfg)?
            }
        }
        if cnfg.recursion == 255 || recursion < cnfg.recursion {
            for entry in read_dir(old_position)? {
                let entry: DirEntry = match entry {
                    Ok(e) => e,
                    Err(_) => continue,
                };
                if !cnfg.all {
                    match entry.file_name().to_str() {
                        Some(name) => if name.starts_with(".") {continue;}, 
                        _ => continue};
                }

                let mut path: PathBuf = entry.path();

                match recurse_through(&mut path, new_position, if recursion < 255 {recursion+1} else {recursion}, cnfg) {
                    Ok(()) => {},
                    Err(e) => {
                        if cnfg.log {
                            println!("Problem refactoring {}", path.display());
                            println!("Exiting with error");}
                            return Err(e)},
                }
                if cnfg.copy {
                    new_position.pop();
                }
            }
        }
    }

    Ok(())
}


/// Refactors file structure from given start.
/// 
/// Takes config stored in &[`Config`] and returns [`Result<(), Box<dyn Error>>`]
///
///  
/// # Errors
/// Errors can occure during calls of [`change_file_name`] and [`copy_change_file_name`] or during Pathbuff creation. <br />
/// Error message should give you a hint, try to enable logging.
pub fn run(cnfg: &Config) -> Result<(), Box<dyn Error>> {
    if cnfg.no_run {
        match cnfg.usage {
            'h' => {for line in HELP.lines() {println!("{}", line)}},
            'v' => {println!("NameRefactor version {}", VERSION)},
            _ => {},
        }
        return Ok(());
    }

    let position: PathBuf = match PathBuf::from_str(&cnfg.start) {
        Ok(r) => r,
        Err(e) => return Err(Box::new(e)),
    };
    let position: PathBuf = match position.canonicalize() {
        Ok(pos) => pos,
        Err(e) => return Err(Box::new(e)),
    };

    let mut old_position: PathBuf = position.clone();
    let mut new_position: PathBuf = position;

    if cnfg.log {
        println!("FileNameRefactor starting in {}", cnfg.start);
        println!("Copy - {}", cnfg.copy);
        println!("---")}
    
    if &cnfg.start == "." {
        for entry in read_dir(old_position)? {
            let entry: DirEntry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            if !cnfg.all {
                match entry.file_name().to_str() {
                    Some(name) => if name.starts_with(".") {continue;}, 
                    _ => continue};
            }

            let mut old_path: PathBuf = entry.path();
            let mut new_path: PathBuf = entry.path();

            match recurse_through(&mut old_path, &mut new_path, 0, cnfg) {
                Ok(()) => {},
                Err(e) => {
                    if cnfg.log {
                        println!("Problem refactoring {}", old_path.display());
                        println!("Exiting with error");}
                        return Err(e)},
            }
            if cnfg.copy {
                new_position.pop();
            }
        }
        return Ok(());
    }
    
    recurse_through(&mut old_position, &mut new_position, 0, cnfg)
}