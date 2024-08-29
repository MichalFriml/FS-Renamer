use std::char;

/// Holds current version of the crate
pub const VERSION: &str = "0.2.1";
/// Loads and holds DOCUMENTATION.md from the crate's directory
pub const HELP: &'static str = include_str!("../../DOCUMENTATION.md");
/// An array of ASCII chars, which should not be used in directory and file names
pub const ILLEGAL: [char; 22] = ['#', '%', '&', '{', '}', '\\', '<', '>', '*', '?', '/', ' ',
'$', '!', '\'', '"', ':', '@', '+', '`', '|', '='];
const COPY_IDENTIFIER: &str = "-nrfcopy";


/// Holds control values needed for a script's logic
/// 
/// Create a new one either with default values ::new() or <br />
/// with given vec! of String arguments ::build(&args) as a Result<Config, &'static str> 
/// ```text
/// let mut def_cnfg: Config = Config::new();
/// def_cnfg.start = String::from("./dir1");
/// 
/// let cstm_cnfg: Result<Config, &'static str> = Config::build(&vec![String::from("./dir1"), String::from("-r=2")]);
/// ```
/// 
pub struct Config { 
    /// affect directories
    pub directories: bool,
    /// affect files
    pub files: bool,
    /// recurse into directories by that many levels, 255 means unlimited
    pub recursion: u8,
    /// copy everything instead of renaming in place
    pub copy: bool,
    /// set a start location
    pub start: String,
    /// supress error messages
    pub silent: bool,
    /// enable logging
    pub log: bool,

    /// remove non-ASCII characters
    pub to_ascii: bool,
    /// affect things starting with '.'
    pub all: bool,
    /// remove illegal characters
    pub illegal: bool,
    /// replace removed chars by
    pub replacer: String,
    /// replace chars with diacritics with their ASCII variant
    pub diacritics: bool,
    /// what to append to first level cop
    pub copy_identifier: String,

    /// do not run (just output something)
    pub no_run: bool,
    /// what to do istead of running (display help, version etc.)
    pub usage: char,
} 


impl Config {
    /// Creates a new instance of Config with default values
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

    /// Creates a new instance of Config from a vector of Strings defined to take command-line arguments.
    /// 
    /// Arguments needs to be passed as flags, value can be appended with = as -r=2. <br />
    /// Single flags can be joined into compound flags, but new flag cannot be appended after one with value: -r=2R=c is not valid. <br />
    /// Starting path can be passed without any flag, for example: fsrenamer -sr=2 "./dir1" -> silent=true, recursion=2, start="./dir"
    /// 
    /// # Errors 
    /// - Error: Cannot use -f and -d simultaneously - cannot use both flags, would have no target
    /// - Error: Invalid recursion level - the value in -r=u8 is invalid
    /// - Error: Invalid replacing character - the value in -R=char is invalid
    /// - Error: Invalid flag, check documentation - unrecognised flag used
    /// 
    /// NOTE that invalid flag may be treated as a start path <br />
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
            if arg.starts_with("--") {
                if arg.eq("--help") {
                    no_run = true; 
                    usage = 'h';
                    return Ok(Config{directories, files, recursion, copy, start, silent, log, to_ascii, all, illegal, no_run, usage, replacer, diacritics, copy_identifier});
                } else if arg.eq("--version") {
                    no_run = true; 
                    usage = 'v';
                    return Ok(Config{directories, files, recursion, copy, start, silent, log, to_ascii, all, illegal, no_run, usage, replacer, diacritics, copy_identifier});
                }
            } else if arg.starts_with("-") {
                let mut iter = arg.chars();
                iter.next();
                while let Some(char) = iter.next() {
                    match char {
                        'd' => files = false,
                        'f' => directories = false,
                        'c' => copy = true,
                        's' => silent = true,
                        'l' => log = true,
                        'a' => all = true,
                        'A' => to_ascii = false,
                        'I' => illegal = false,
                        'D' => diacritics = false,
                        'r' | 'R' => {
                            if let Some(ch) = iter.next() {
                                if ch.ne(&'=') {break;}
                            } else {break;}
                            let remain: String = iter.collect();

                            match char {
                                'r' => {
                                    match remain.parse::<u8>() {
                                    Ok(n) => recursion = n,
                                    Err(_) => return Err("Invalid recursion level"),
                                }},
                                'R' => {
                                    if !remain.is_ascii() || remain.contains(ILLEGAL) {return Err("Invalid replacing character")};
                                    replacer = String::from(remain);
                                },
                                _ => {}
                            }
                            break;
                        },
                        _ => return Err("Invalid flag, check documentation"),
                    }
                }
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