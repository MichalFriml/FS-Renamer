use std::fs::{rename, copy as fscopy, create_dir};
use std::error::Error;
use std::path::PathBuf;
use super::{Config, ILLEGAL};


/// Takes &String and returns copied String where diacritics are replaced by their ASCII variants
pub fn remove_diacritics(original: &String) -> String {
    let mut new: String = String::with_capacity(original.len());

    for char in original.chars() {
        new.push(match char {
            'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' => 'A',
            'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' => 'O',
            'È' | 'É' | 'Ê' | 'Ë' | 'Ě' => 'E',
            'Ç' | 'Č' => 'C', 
            'Ð' | 'Ď' => 'D',
            'Ì' | 'Í' | 'Î' | 'Ï' => 'I',
            'Ù' | 'Ú' | 'Û' | 'Ü' | 'Ů' => 'U',
            'Ñ' => 'N',
            'Ř' => 'R',
            'Š' => 'S',
            'Ť' => 'T',
            'Ÿ' | 'Ý' => 'Y',
            'Ž' => 'Z',
        
            'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => 'a',
            'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' => 'o',
            'è' | 'é' | 'ê' | 'ë' | 'ð' | 'ě' => 'e',
            'ç' | 'č' => 'c',
            'ď' => 'd',
            'ì' | 'í' | 'î' | 'ï' => 'i',
            'ù' | 'ú' | 'û' | 'ü' | 'ů' => 'u',
            'ñ' => 'n',
            'ř' => 'r',
            'š' => 's',
            'ť' => 't',
            'ÿ' | 'ý' => 'y',
            'ž' => 'z',
        
            _ => char,
        })
    }
    new
}


/// Takes &mut String, &[`Config`] and returns copied String. <br />
/// Depending on the Config, returned String can have diacritics replaced with their ASCII variant, <br />
/// nonascii and/or illegal characters removed
pub fn refactor_name(original: &mut String, cnfg: &Config) -> String {
    let nodia: String = if cnfg.diacritics {remove_diacritics(original)} else {original.to_owned()};

    let ascii: String = if cnfg.to_ascii && cnfg.illegal {
        nodia.replace(|c: char| !c.is_ascii() || ILLEGAL.contains(&c), &cnfg.replacer)
    } else if cnfg.to_ascii {
        nodia.replace(|c: char| !c.is_ascii(), &cnfg.replacer)
    } else if cnfg.illegal {
        nodia.replace(|c: char| ILLEGAL.contains(&c), &cnfg.replacer)
    } else {nodia.to_owned()};

    ascii
}


/// Takes &mut Pathbuf and &[`Config`], where Pathbuf is the path to target file or dir. <br />
/// Refactors the file/dir's name in-place using [`refactor_name`]. <br />
pub fn copy_change_file_name(old_position: &mut PathBuf, new_position: &mut PathBuf, cnfg: &Config, first: bool) 
-> Result<(), Box<dyn Error>> {
    //old_position is dir1/file.txt -> new_position is dir2/
    //old_position is dir1/innerdir -> new_position is dir2/

    if first {
        let mut name: String = match old_position.file_stem() {
            Some(os) => {match os.to_owned().into_string() {
                Ok(n) => n,
                Err(_) => String::new(),
            }},
            _ => return Err(String::from("Invalid file path").into()),
        };
        let mut ext: String = match old_position.extension() {
            Some(os) => {match os.to_owned().into_string() {
                Ok(n) => n,
                Err(_) => String::new(),
            }},
            _ => String::new(),
        };

        let mut new_name: String = refactor_name(&mut name, cnfg);
        new_name.push_str(&cnfg.copy_identifier);
        new_position.set_file_name(new_name);
        new_position.set_extension(refactor_name(&mut ext, cnfg));
    } else {
        let mut name: String = match old_position.file_name() {
            Some(os) => { match os.to_owned().into_string() {
                Ok(s) => s,
                Err(_) => String::new(),
            }},
            _ => return Err(String::from("Invalid file path").into()),
        };

        let new_name: String = refactor_name(&mut name, cnfg);
        new_position.push(new_name);
    }

    match new_position.try_exists() {
        Ok(b) => if b {return Err(String::from("File would overwrite existing one").into())}
        Err(e) => return Err(Box::new(e)) 
    }

    if cnfg.log {println!("Refactored {} to {}", old_position.display(), new_position.display())};
    if old_position.is_file() {
        match fscopy(old_position, new_position) {
            Ok(_) => {Ok(())},
            Err(e) => return Err(Box::new(e))
        }
    } else {
        match create_dir(new_position) {
            Ok(_) => {Ok(())},
            Err(e) => return Err(Box::new(e))
        }
    }
    
}


/// Takes &mut Pathbuf and &[`Config`], where Pathbuf is the path to target file or dir. <br />
/// Refactors the file/dir's name in-place using [`refactor_name`]. <br />
/// 
pub fn change_file_name(position: &mut PathBuf, cnfg: &Config) -> Result<(), Box<dyn Error>> {
    let mut name: String = match position.file_name() {
        Some(os) => { match os.to_owned().into_string() {
            Ok(s) => s,
            Err(_) => String::new(),
        }},
        _ => return Err(String::from("Invalid file path").into()),
    };
    let new_name: String = refactor_name(&mut name, cnfg);
    if new_name.eq(&name) {
        return Ok(())}

    let old_position: PathBuf = position.clone();
    position.set_file_name(new_name);
    match position.try_exists() {
        Ok(b) => if b {return Err(String::from("File would overwrite existing one").into())}
        Err(e) => return Err(Box::new(e)) 
    }
    match rename(&old_position, &position) {
        Ok(_) => {
            if cnfg.log {println!("Refactored {} to {}", old_position.display(), position.display())};
            Ok(())},
        Err(e) => return Err(Box::new(e))
    }
}


#[cfg(test)]
mod tests;