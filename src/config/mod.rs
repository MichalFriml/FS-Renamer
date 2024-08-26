pub const VERSION: &str = "0.1.0";
pub const HELP: &'static str = include_str!("../../DOCUMENTATION.md");
pub const ILLEGAL: [char; 22] = ['#', '%', '&', '{', '}', '\\', '<', '>', '*', '?', '/', ' ',
'$', '!', '\'', '"', ':', '@', '+', '`', '|', '='];
const COPY_IDENTIFIER: &str = "-nrfcopy";


#[derive(Debug)]
pub struct Config {
    pub directories: bool,
    pub files: bool,
    pub recursion: u8,
    pub copy: bool,
    pub start: String,
    pub silent: bool,
    pub log: bool,

    pub to_ascii: bool,
    pub all: bool,
    pub illegal: bool,
    pub replacer: String,
    pub diacritics: bool,

    pub no_run: bool,
    pub usage: char,
    pub copy_identifier: String,
} 


impl Config {
    pub fn new() -> Config {
        Config{
            directories: true, 
            files: true, 
            recursion: 0u8, 
            copy: false, 
            start: String::from("."), 
            silent: false, 
            log: false,
            to_ascii: true,
            all: false,
            illegal: true,
            no_run: false, 
            usage: 'n',
            replacer: String::new(),
            diacritics: true,
            copy_identifier: String::from(COPY_IDENTIFIER),
        }
    }

    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        let mut directories: bool = true;
        let mut files: bool = true;
        let mut recursion: u8 = 0;
        let mut copy: bool = false;
        let mut start: String = String::from(".");
        let mut silent: bool = false;
        let mut log: bool = false;

        let mut to_ascii: bool = true;
        let mut all: bool = false;
        let mut illegal: bool = true;

        let mut no_run: bool = false;
        let mut usage: char = 'n';
        let mut replacer: String = String::new();
        let mut diacritics: bool = true;
        let copy_identifier: String = String::from(COPY_IDENTIFIER);

        for arg in args {
            if arg.starts_with("--help") {
                no_run = true; 
                usage = 'h';
                return Ok(Config{directories, files, recursion, copy, start, silent, log, to_ascii, all, illegal, no_run, usage, replacer, diacritics, copy_identifier});
            } else if arg.starts_with("--version") {
                no_run = true; 
                usage = 'v';
                return Ok(Config{directories, files, recursion, copy, start, silent, log, to_ascii, all, illegal, no_run, usage, replacer, diacritics, copy_identifier});
            }

            else if arg.starts_with("-d") {files = false;}
            else if arg.starts_with("-f") {directories = false;}
            else if arg.starts_with("-c") {copy = true;}
            else if arg.starts_with("-s") {silent = true;}
            else if arg.starts_with("-l") {log = true;}
            else if arg.starts_with("-a") {all = true;}
            else if arg.starts_with("-A") {to_ascii = false;}
            else if arg.starts_with("-I") {illegal = false;}
            else if arg.starts_with("-D") {diacritics = false;}
            else if arg.starts_with("-r") {
                let a: &str = &arg[3..];
                match a.parse::<u8>() {
                    Ok(n) => recursion = n,
                    Err(_) => return Err("Invalid recursion level"),
                }
            } else if arg.starts_with("-R") {
                let a: &str = &arg[3..];
                if !a.is_ascii() || a.contains(ILLEGAL) {return Err("Invalid replacing character")};
                replacer = String::from(a)
            } else {
                start = String::from(arg);
            }
        }

        if !files && !directories {return Err("Cannot use -f and -d simultaneously")};
        Ok(Config{directories, files, recursion, copy, start, silent, log, to_ascii, all, illegal, no_run, usage, replacer, diacritics, copy_identifier})
    }
}


#[cfg(test)]
mod tests;