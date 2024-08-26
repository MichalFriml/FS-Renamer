use super::*;
use::std::fs::{create_dir, create_dir_all, remove_dir_all, remove_file, File, remove_dir, read_dir};
use::std::path::{Path, PathBuf};


#[test]
fn refname_001_empty() {
    let cnfg: Config = Config::new();

    let mut original: String = String::from("");
    let target: String = String::from("");

    assert_eq!(refactor_name(&mut original, &cnfg), target);
}
#[test]
fn refname_002_perfectly_valid() {
    let cnfg: Config = Config::new();

    let mut original: String = String::from("hello.txt");
    let target: String = String::from("hello.txt");

    assert_eq!(refactor_name(&mut original, &cnfg), target);
}
#[test]
fn refname_003_illegal() {
    let cnfg: Config = Config::new();

    let mut original: String = String::from("h:d!%kf21*.tx@");
    let target: String = String::from("hdkf21.tx");

    assert_eq!(refactor_name(&mut original, &cnfg), target);
}
#[test]
fn refname_004_illegal_w_replacer() {
    let cnfg: Config = match Config::build(&vec![String::from("-R=q")]) {
        Ok(c) => c,
        Err(_) => panic!()
    };
    let mut original: String = String::from("h:d!%kf21*.tx@");
    let target: String = String::from("hqdqqkf21q.txq");

    assert_eq!(refactor_name(&mut original, &cnfg), target);
}
#[test]
fn refname_005_nonascii() {
    let cnfg: Config = Config::new();

    let mut original: String = String::from("hčdúбkf21仿.tx╚");
    let target: String = String::from("hcdukf21.tx");

    assert_eq!(refactor_name(&mut original, &cnfg), target);
}
#[test]
fn refname_006_nonascii_w_replacer() {
    let cnfg: Config = match Config::build(&vec![String::from("-R=q")]) {
        Ok(c) => c,
        Err(_) => panic!()
    };
    let mut original: String = String::from("hčdúбkf21仿.tx╚");
    let target: String = String::from("hcduqkf21q.txq");

    assert_eq!(refactor_name(&mut original, &cnfg), target);
}
#[test]
fn refname_007_nonascii_non_illegal_w_replacer() {
    let cnfg: Config = match Config::build(&vec![String::from("-R=q")]) {
        Ok(c) => c,
        Err(_) => panic!()
    };
    let mut original: String = String::from("hčd%бkf21*.tx╚");
    let target: String = String::from("hcdqqkf21q.txq");

    assert_eq!(refactor_name(&mut original, &cnfg), target);
}
#[test]
fn refname_008_replace_nonascii_only_w_replacer() {
    let cnfg: Config = match Config::build(&vec![String::from("-R=q"), String::from("-I")]) {
        Ok(c) => c,
        Err(_) => panic!()
    };
    let mut original: String = String::from("hčd%бkf21*.tx╚");
    let target: String = String::from("hcd%qkf21*.txq");

    assert_eq!(refactor_name(&mut original, &cnfg), target);
}
#[test]
fn refname_009_replace_illegal_only_w_replacer() {
    let cnfg: Config = match Config::build(&vec![String::from("-R=q"), String::from("-A")]) {
        Ok(c) => c,
        Err(_) => panic!()
    };
    let mut original: String = String::from("hčd%бkf21*.tx╚");
    let target: String = String::from("hcdqбkf21q.tx╚");

    assert_eq!(refactor_name(&mut original, &cnfg), target);
}
#[test]
fn refname_010_let_newline() {
    let cnfg: Config = match Config::build(&vec![String::from("-R=q"), String::from("-A")]) {
        Ok(c) => c,
        Err(_) => panic!()
    };
    let mut original: String = String::from("hčd%бkf
21*.tx╚");
    let target: String = String::from("hcdqбkf
21q.tx╚");

    assert_eq!(refactor_name(&mut original, &cnfg), target);
}
#[test]
fn refname_011_empty_space() {
    let cnfg: Config = match Config::build(&vec![String::from("-R=q"), String::from("-A")]) {
        Ok(c) => c,
        Err(_) => panic!()
    };
    let mut original: String = String::from("hčd%бkf  21*.tx╚");
    let target: String = String::from("hcdqбkfqq21q.tx╚");

    assert_eq!(refactor_name(&mut original, &cnfg), target);
}
#[test]
fn refname_012_diacritics() {
    let cnfg: Config = match Config::build(&vec![]) {
        Ok(c) => c,
        Err(_) => panic!()
    };
    let mut original: String = String::from("hčd%бkf  21*.tx╚");
    let target: String = String::from("hcdkf21.tx");

    assert_eq!(refactor_name(&mut original, &cnfg), target);
}
#[test]
fn refname_013_no_diacritics() {
    let cnfg: Config = match Config::build(&vec![String::from("-D")]) {
        Ok(c) => c,
        Err(_) => panic!()
    };
    let mut original: String = String::from("hčd%бkf  21*.tx╚");
    let target: String = String::from("hdkf21.tx");

    assert_eq!(refactor_name(&mut original, &cnfg), target);
}
#[test]
fn refname_014_no_no_nope() {
    let cnfg: Config = match Config::build(&vec![String::from("-D"), String::from("-I"), String::from("-A")]) {
        Ok(c) => c,
        Err(_) => panic!()
    };
    let mut original: String = String::from("hčd%бkf  21*.tx╚");
    let target: String = String::from("hčd%бkf  21*.tx╚");

    assert_eq!(refactor_name(&mut original, &cnfg), target);
}
#[test]
fn refname_015_only_dia_w_rep() {
    let cnfg: Config = match Config::build(&vec![String::from("-R=q")]) {
        Ok(c) => c,
        Err(_) => panic!()
    };
    let mut original: String = String::from("hčdúkf21.tx");
    let target: String = String::from("hcdukf21.tx");

    assert_eq!(refactor_name(&mut original, &cnfg), target);
}
#[test]
fn refname_016_only_dia_n_rep() {
    let cnfg: Config = match Config::build(&vec![]) {
        Ok(c) => c,
        Err(_) => panic!()
    };
    let mut original: String = String::from("hčdúkf21.tx");
    let target: String = String::from("hcdukf21.tx");

    assert_eq!(refactor_name(&mut original, &cnfg), target);
}

#[test]
fn chngname_001_valid_file() {
    let cnfg: Config = Config::new();    
    let work_dir: &str = "./tests_workdir/unit/renamer/chngname/001";
    let file1_str: &str = "./tests_workdir/unit/renamer/chngname/001/file1.txt";

    match create_dir_all(Path::new(work_dir)) {_ => {}}
    match remove_file(&file1_str) {_ => {}}
    match File::create(&file1_str) {Ok(_) => {}, Err(_) => panic!()}
    
    match change_file_name(&mut PathBuf::from(&file1_str), &cnfg) {Ok(_) => {}, Err(_) => panic!()}
    assert!(PathBuf::from(&file1_str).exists());
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 1);
}
#[test]
fn chngname_002_valid_dir() {
    let cnfg: Config = Config::new();    
    let work_dir: &str = "./tests_workdir/unit/renamer/chngname/002";
    let dir1_str: &str = "./tests_workdir/unit/renamer/chngname/002/dir1";

    match create_dir_all(Path::new(work_dir)) {_ => {}}
    match remove_dir(&dir1_str) {_ => {}}
    match create_dir(&dir1_str) {Ok(_) => {}, Err(_) => panic!()}
    
    match change_file_name(&mut PathBuf::from(&dir1_str), &cnfg) {Ok(_) => {}, Err(_) => panic!()}
    assert!(PathBuf::from(&dir1_str).exists());
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 1);
}
#[test]
fn chngname_003_nonascii_dir_and_file() {
    let cnfg: Config = match Config::build(&vec![String::from("-I")]) {Ok(c) => c, Err(_) => panic!()};    
    let work_dir: &str = "./tests_workdir/unit/renamer/chngname/003";
    let file1_str_org: &str = "./tests_workdir/unit/renamer/chngname/003/fi仿l+e1č.tx╚t";
    let file1_str_new: &str = "./tests_workdir/unit/renamer/chngname/003/fil+e1c.txt";
    let dir1_str_org: &str = "./tests_workdir/unit/renamer/chngname/003/di╚r+1č仿";
    let dir1_str_new: &str = "./tests_workdir/unit/renamer/chngname/003/dir+1c";

    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {_ => {}};

    match File::create(&file1_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    match create_dir(&dir1_str_org) {Ok(_) => {}, Err(_) => panic!()};
    
    match change_file_name(&mut PathBuf::from(&file1_str_org), &cnfg) {Ok(_) => {}, Err(_) => panic!()};
    assert!(PathBuf::from(&file1_str_new).exists());
    match change_file_name(&mut PathBuf::from(&dir1_str_org), &cnfg) {Ok(_) => {}, Err(_) => panic!()};
    assert!(PathBuf::from(&dir1_str_new).exists());
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 2);
}
#[test]
fn chngname_004_illegal_dir_and_file() {
    let cnfg: Config = match Config::build(&vec![String::from("-A")]) {Ok(c) => c, Err(_) => panic!()};    
    let work_dir: &str = "./tests_workdir/unit/renamer/chngname/004";
    let file1_str_org: &str = "./tests_workdir/unit/renamer/chngname/004/fi仿l+e1č.tx╚t";
    let file1_str_new: &str = "./tests_workdir/unit/renamer/chngname/004/fi仿le1c.tx╚t";
    let dir1_str_org: &str = "./tests_workdir/unit/renamer/chngname/004/di╚r+1č仿";
    let dir1_str_new: &str = "./tests_workdir/unit/renamer/chngname/004/di╚r1c仿";

    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {_ => {}};

    match File::create(&file1_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    match create_dir(&dir1_str_org) {Ok(_) => {}, Err(_) => panic!()};
    
    match change_file_name(&mut PathBuf::from(&file1_str_org), &cnfg) {Ok(_) => {}, Err(_) => panic!()};
    assert!(PathBuf::from(&file1_str_new).exists());
    match change_file_name(&mut PathBuf::from(&dir1_str_org), &cnfg) {Ok(_) => {}, Err(_) => panic!()};
    assert!(PathBuf::from(&dir1_str_new).exists());
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 2);
}
#[test]
fn chngname_005_illegal_and_ascii_dir_and_file() {
    let cnfg: Config = Config::new();   
    let work_dir: &str = "./tests_workdir/unit/renamer/chngname/005";
    let file1_str_org: &str = "./tests_workdir/unit/renamer/chngname/005/fi仿l+e1č.tx╚t";
    let file1_str_new: &str = "./tests_workdir/unit/renamer/chngname/005/file1c.txt";
    let dir1_str_org: &str = "./tests_workdir/unit/renamer/chngname/005/di╚r+1č仿";
    let dir1_str_new: &str = "./tests_workdir/unit/renamer/chngname/005/dir1c";

    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {_ => {}};

    match File::create(&file1_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    match create_dir(&dir1_str_org) {Ok(_) => {}, Err(_) => panic!()};
    
    match change_file_name(&mut PathBuf::from(&file1_str_org), &cnfg) {Ok(_) => {}, Err(_) => panic!()};
    assert!(PathBuf::from(&file1_str_new).exists());
    match change_file_name(&mut PathBuf::from(&dir1_str_org), &cnfg) {Ok(_) => {}, Err(_) => panic!()};
    assert!(PathBuf::from(&dir1_str_new).exists());
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 2);
}
#[test]
fn chngname_006_illegal_ascii_dir_file_w_replacer() {
    let cnfg: Config = match Config::build(&vec![String::from("-R=x")]) {Ok(c) => c, Err(_) => panic!()};    
    let work_dir: &str = "./tests_workdir/unit/renamer/chngname/006";
    let file1_str_org: &str = "./tests_workdir/unit/renamer/chngname/006/fi仿l+e1č.tx╚t";
    let file1_str_new: &str = "./tests_workdir/unit/renamer/chngname/006/fixlxe1c.txxt";
    let dir1_str_org: &str = "./tests_workdir/unit/renamer/chngname/006/di╚r+1č仿";
    let dir1_str_new: &str = "./tests_workdir/unit/renamer/chngname/006/dixrx1cx";

    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {_ => {}};

    match File::create(&file1_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    match create_dir(&dir1_str_org) {Ok(_) => {}, Err(_) => panic!()};
    
    match change_file_name(&mut PathBuf::from(&file1_str_org), &cnfg) {Ok(_) => {}, Err(_) => panic!()};
    assert!(PathBuf::from(&file1_str_new).exists());
    match change_file_name(&mut PathBuf::from(&dir1_str_org), &cnfg) {Ok(_) => {}, Err(_) => panic!()};
    assert!(PathBuf::from(&dir1_str_new).exists());
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 2);
}
#[test]
fn chngname_007_only_dia() {
    let cnfg: Config = match Config::build(&vec![String::from("-R=x")]) {Ok(c) => c, Err(_) => panic!()};    
    let work_dir: &str = "./tests_workdir/unit/renamer/chngname/007";
    let file1_str_org: &str = "./tests_workdir/unit/renamer/chngname/007/hčdúkf21.tx";
    let file1_str_new: &str = "./tests_workdir/unit/renamer/chngname/007/hcdukf21.tx";
    let dir1_str_org: &str = "./tests_workdir/unit/renamer/chngname/007/hšdúkf2";
    let dir1_str_new: &str = "./tests_workdir/unit/renamer/chngname/007/hsdukf2";

    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {_ => {}};

    match File::create(&file1_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    match create_dir(&dir1_str_org) {Ok(_) => {}, Err(_) => panic!()};
    
    match change_file_name(&mut PathBuf::from(&file1_str_org), &cnfg) {Ok(_) => {}, Err(_) => panic!()};
    assert!(PathBuf::from(&file1_str_new).exists());
    match change_file_name(&mut PathBuf::from(&dir1_str_org), &cnfg) {Ok(_) => {}, Err(_) => panic!()};
    assert!(PathBuf::from(&dir1_str_new).exists());
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 2);
}

#[test]
fn copy_chngname_001_valid_file_first() {
    let cnfg: Config = Config::new();    
    let work_dir: &str = "./tests_workdir/unit/renamer/copychngname/001";
    let file1_str: &str = "./tests_workdir/unit/renamer/copychngname/001/file1.txt";
    let file1_str_copied: &str = "./tests_workdir/unit/renamer/copychngname/001/file1-nrfcopy.txt";

    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {_ => {}}
    match File::create(&file1_str) {Ok(_) => {}, Err(_) => panic!()}
    
    match copy_change_file_name(&mut PathBuf::from(&file1_str), &mut PathBuf::from(&file1_str), &cnfg, true) {Ok(_) => {}, Err(_) => panic!()}
    assert!(PathBuf::from(&file1_str).exists());
    assert!(PathBuf::from(&file1_str_copied).exists());
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 2);
}
#[test]
fn copy_chngname_002_invalid_file_first() {
    let cnfg: Config = Config::new();    
    let work_dir: &str = "./tests_workdir/unit/renamer/copychngname/002";
    let file1_str: &str = "./tests_workdir/unit/renamer/copychngname/002/fi仿le1.tx仿t";
    let file1_str_copied: &str = "./tests_workdir/unit/renamer/copychngname/002/file1-nrfcopy.txt";

    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {_ => {}};
    match File::create(&file1_str) {Ok(_) => {}, Err(_) => panic!()};
    
    match copy_change_file_name(&mut PathBuf::from(&file1_str), &mut PathBuf::from(&file1_str), &cnfg, true) {Ok(_) => {}, Err(_) => panic!()}
    assert!(PathBuf::from(&file1_str).exists());
    assert!(PathBuf::from(&file1_str_copied).exists());
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 2);
}
#[test]
fn copy_chngname_003_valid_dir_first() {
    let cnfg: Config = Config::new();    
    let work_dir: &str = "./tests_workdir/unit/renamer/copychngname/003";
    let dir1_str: &str = "./tests_workdir/unit/renamer/copychngname/003/dir1";
    let dir1_str_copied: &str = "./tests_workdir/unit/renamer/copychngname/003/dir1";

    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir(&dir1_str) {Ok(_) => {}, Err(_) => panic!()};
    
    match copy_change_file_name(&mut PathBuf::from(&dir1_str), &mut PathBuf::from(&dir1_str), &cnfg, true) {Ok(_) => {}, Err(_) => panic!()}
    assert!(PathBuf::from(&dir1_str).exists());
    assert!(PathBuf::from(&dir1_str_copied).exists());
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 2);
}
#[test]
fn copy_chngname_004_invalid_dir_first() {
    let cnfg: Config = Config::new();    
    let work_dir: &str = "./tests_workdir/unit/renamer/copychngname/004";
    let dir1_str: &str = "./tests_workdir/unit/renamer/copychngname/004/d仿ir仿1";
    let dir1_str_copied: &str = "./tests_workdir/unit/renamer/copychngname/004/dir1-nrfcopy";

    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir(&dir1_str) {Ok(_) => {}, Err(_) => panic!()};
    
    match copy_change_file_name(&mut PathBuf::from(&dir1_str), &mut PathBuf::from(&dir1_str), &cnfg, true) {Ok(_) => {}, Err(_) => panic!()}
    assert!(PathBuf::from(&dir1_str).exists());
    assert!(PathBuf::from(&dir1_str_copied).exists());
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 2);
}
#[test]
fn copy_chngname_005_invalid_file_first() {
    let cnfg: Config = Config::new();    
    let work_dir: &str = "./tests_workdir/unit/renamer/copychngname/005";
    let file1_str: &str = "./tests_workdir/unit/renamer/copychngname/005/fi仿le1.tx仿t";
    let file1_copy: &str = "./tests_workdir/unit/renamer/copychngname/005";
    let file1_str_copied: &str = "./tests_workdir/unit/renamer/copychngname/005/file1.txt";

    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {_ => {}};
    match File::create(&file1_str) {Ok(_) => {}, Err(_) => panic!()};
    
    match copy_change_file_name(&mut PathBuf::from(&file1_str), &mut PathBuf::from(&file1_copy), &cnfg, false) {Ok(_) => {}, Err(_) => panic!()}
    assert!(PathBuf::from(&file1_str).exists());
    assert!(PathBuf::from(&file1_str_copied).exists());
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 2);
}