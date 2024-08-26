use fs_renamer::{Config, run};
use::std::fs::{create_dir, create_dir_all, remove_dir_all, remove_file, File, remove_dir, read_dir};
use::std::path::{Path, PathBuf};


#[test]
fn t001_empty_valid() {
    let work_dir: &str = "./tests_workdir/integration/001";
    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let cnfg: Config = match Config::build(&vec![String::from(work_dir)]) {Ok(c) => c, Err(_) => panic!()};    
    match run(&cnfg) {
        Ok(()) => {},
        Err(e) => {panic!("{}", e)}
    }
    assert!(PathBuf::from(work_dir).exists());
}
#[test]
fn t002_empty_dir_invalid() {
    let work_dir: &str = "./tests_workdir/integration/002";
    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let dir1_str_org: &str = &{let mut s = work_dir.to_owned(); s.push_str("/d仿i+rб1"); s};
    let dir1_str_new: &str = &{let mut s = work_dir.to_owned(); s.push_str("/dir1"); s};
    match create_dir(&dir1_str_org) {Ok(_) => {}, Err(_) => panic!()};
    
    let cnfg: Config = match Config::build(&vec![String::from(dir1_str_org)]) {Ok(c) => c, Err(_) => panic!()};    
    match run(&cnfg) {
        Ok(()) => {},
        Err(e) => {panic!("{}", e)}
    }

    assert!(PathBuf::from(&dir1_str_new).exists());

    assert_eq!(match read_dir(PathBuf::from(work_dir)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 1);
}
#[test]
fn t003_file_invalid() {
    let work_dir: &str = "./tests_workdir/integration/003";
    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let file1_str_org: &str = &{let mut s = work_dir.to_owned(); s.push_str("/f仿i+le1б.txt"); s};
    let file1_str_new: &str = &{let mut s = work_dir.to_owned(); s.push_str("/file1.txt"); s};
    match File::create(&file1_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    
    let cnfg: Config = match Config::build(&vec![String::from(file1_str_org)]) {Ok(c) => c, Err(_) => panic!()};    
    match run(&cnfg) {
        Ok(()) => {},
        Err(e) => {panic!("{}", e)}
    }

    assert!(PathBuf::from(&file1_str_new).exists());

    assert_eq!(match read_dir(PathBuf::from(work_dir)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 1);
}
#[test]
fn t004_dir_w_files_invalid_r0() {
    let work_dir: &str = "./tests_workdir/integration/004";
    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let dir1_str_org: &str = &{let mut s = work_dir.to_owned(); s.push_str("/d仿i+rб1"); s};
    let dir1_str_new: &str = &{let mut s = work_dir.to_owned(); s.push_str("/dir1"); s};
    match create_dir(&dir1_str_org) {Ok(_) => {}, Err(_) => panic!()};

    let file1_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/f仿i+le1б.t仿xt"); s};
    let file1_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/f仿i+le1б.t仿xt"); s};
    match File::create(&file1_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    let file2_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/file2"); s};
    let file2_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/file2"); s};
    match File::create(&file2_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    
    let cnfg: Config = match Config::build(&vec![String::from(dir1_str_org)]) {Ok(c) => c, Err(_) => panic!()};    
    match run(&cnfg) {
        Ok(()) => {},
        Err(e) => {panic!("{}", e)}
    }

    assert!(PathBuf::from(&dir1_str_new).exists());
    assert!(PathBuf::from(&file1_str_new).exists());
    assert!(PathBuf::from(&file2_str_new).exists());

    assert_eq!(match read_dir(PathBuf::from(work_dir)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 1);
    assert_eq!(match read_dir(PathBuf::from(dir1_str_new)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 2);
}
#[test]
fn t005_dir_w_files_invalid_r1() {
    let work_dir: &str = "./tests_workdir/integration/005";
    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let dir1_str_org: &str = &{let mut s = work_dir.to_owned(); s.push_str("/d仿i+rб仿1"); s};
    let dir1_str_new: &str = &{let mut s = work_dir.to_owned(); s.push_str("/dir1"); s};
    match create_dir(&dir1_str_org) {Ok(_) => {}, Err(_) => panic!()};

    let file1_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/f仿i+le1б.t仿xt"); s};
    let file1_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/file1.txt"); s};
    match File::create(&file1_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    let file2_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/file2"); s};
    let file2_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/file2"); s};
    match File::create(&file2_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    
    let cnfg: Config = match Config::build(&vec![String::from(dir1_str_org), String::from("-r=1")
        ]) {Ok(c) => c, Err(_) => panic!()};    
    match run(&cnfg) {
        Ok(()) => {},
        Err(e) => {panic!("{}", e)}
    }

    assert!(PathBuf::from(&dir1_str_new).exists());
    assert!(PathBuf::from(&file1_str_new).exists());
    assert!(PathBuf::from(&file2_str_new).exists());

    assert_eq!(match read_dir(PathBuf::from(work_dir)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 1);
    assert_eq!(match read_dir(PathBuf::from(dir1_str_new)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 2);
}
#[test]
fn t006_dir_w_files_dirs_invalid_r0() {
    let work_dir: &str = "./tests_workdir/integration/006";
    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let dir1_str_org: &str = &{let mut s = work_dir.to_owned(); s.push_str("/d仿i+rб仿1"); s};
    let dir1_str_new: &str = &{let mut s = work_dir.to_owned(); s.push_str("/dir1"); s};
    match create_dir(&dir1_str_org) {Ok(_) => {}, Err(_) => panic!()};

    let file1_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/f仿i+le1б.t仿xt"); s};
    let file1_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/f仿i+le1б.t仿xt"); s};
    match File::create(&file1_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    let file2_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/file2"); s};
    let file2_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/file2"); s};
    match File::create(&file2_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    let dir2_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/dir仿2"); s};
    let dir2_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/dir仿2"); s};
    match create_dir(&dir2_str_org) {Ok(_) => {}, Err(_) => panic!()};

    let file3_str_org: &str = &{let mut s = dir2_str_org.to_owned(); s.push_str("/f仿i+le3б.mdď"); s};
    let file3_str_new: &str = &{let mut s = dir2_str_new.to_owned(); s.push_str("/f仿i+le3б.mdď"); s};
    match File::create(&file3_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let cnfg: Config = match Config::build(&vec![String::from(dir1_str_org), String::from("-r=0")
        ]) {Ok(c) => c, Err(_) => panic!()};    
    match run(&cnfg) {
        Ok(()) => {},
        Err(e) => {panic!("{}", e)}
    }
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 1);

    assert!(PathBuf::from(&dir1_str_new).exists());
    assert_eq!(match read_dir(PathBuf::from(dir1_str_new)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 3);

    assert!(PathBuf::from(&file1_str_new).exists());
    assert!(PathBuf::from(&file2_str_new).exists());
    assert!(PathBuf::from(&dir2_str_new).exists());
    assert_eq!(match read_dir(PathBuf::from(dir2_str_new)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 1);

    assert!(PathBuf::from(&file3_str_new).exists());
}
#[test]
fn t007_dir_w_files_dirs_invalid_r1() {
    let work_dir: &str = "./tests_workdir/integration/007";
    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let dir1_str_org: &str = &{let mut s = work_dir.to_owned(); s.push_str("/d仿i+rб仿1"); s};
    let dir1_str_new: &str = &{let mut s = work_dir.to_owned(); s.push_str("/dir1"); s};
    match create_dir(&dir1_str_org) {Ok(_) => {}, Err(_) => panic!()};

    let file1_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/f仿i+le1б.t仿xt"); s};
    let file1_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/file1.txt"); s};
    match File::create(&file1_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    let file2_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/file2"); s};
    let file2_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/file2"); s};
    match File::create(&file2_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    let dir2_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/dir仿2"); s};
    let dir2_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/dir2"); s};
    match create_dir(&dir2_str_org) {Ok(_) => {}, Err(_) => panic!()};

    let file3_str_org: &str = &{let mut s = dir2_str_org.to_owned(); s.push_str("/f仿i+le3б.mdď"); s};
    let file3_str_new: &str = &{let mut s = dir2_str_new.to_owned(); s.push_str("/f仿i+le3б.mdď"); s};
    match File::create(&file3_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let cnfg: Config = match Config::build(&vec![String::from(dir1_str_org), String::from("-r=1")
        ]) {Ok(c) => c, Err(_) => panic!()};    
    match run(&cnfg) {
        Ok(()) => {},
        Err(e) => {panic!("{}", e)}
    }
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 1);

    assert!(PathBuf::from(&dir1_str_new).exists());
    assert_eq!(match read_dir(PathBuf::from(dir1_str_new)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 3);

    assert!(PathBuf::from(&file1_str_new).exists());
    assert!(PathBuf::from(&file2_str_new).exists());
    assert!(PathBuf::from(&dir2_str_new).exists());
    assert_eq!(match read_dir(PathBuf::from(dir2_str_new)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 1);

    assert!(PathBuf::from(&file3_str_new).exists());
}
#[test]
fn t008_dir_w_files_dirs_invalid_r3() {
    let work_dir: &str = "./tests_workdir/integration/008";
    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let dir1_str_org: &str = &{let mut s = work_dir.to_owned(); s.push_str("/d仿i+rб仿1"); s};
    let dir1_str_new: &str = &{let mut s = work_dir.to_owned(); s.push_str("/dir1"); s};
    match create_dir(&dir1_str_org) {Ok(_) => {}, Err(_) => panic!()};

    let file1_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/f仿i+le1б.t仿xt"); s};
    let file1_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/file1.txt"); s};
    match File::create(&file1_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    let file2_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/file2"); s};
    let file2_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/file2"); s};
    match File::create(&file2_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    let dir2_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/dir仿2"); s};
    let dir2_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/dir2"); s};
    match create_dir(&dir2_str_org) {Ok(_) => {}, Err(_) => panic!()};

    let file3_str_org: &str = &{let mut s = dir2_str_org.to_owned(); s.push_str("/f仿i+le3б.mdб"); s};
    let file3_str_new: &str = &{let mut s = dir2_str_new.to_owned(); s.push_str("/file3.md"); s};
    match File::create(&file3_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let cnfg: Config = match Config::build(&vec![String::from(dir1_str_org), String::from("-r=3")
        ]) {Ok(c) => c, Err(_) => panic!()};    
    match run(&cnfg) {
        Ok(()) => {},
        Err(e) => {panic!("{}", e)}
    }
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 1);

    assert!(PathBuf::from(&dir1_str_new).exists());
    assert_eq!(match read_dir(PathBuf::from(dir1_str_new)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 3);

    assert!(PathBuf::from(&file1_str_new).exists());
    assert!(PathBuf::from(&file2_str_new).exists());
    assert!(PathBuf::from(&dir2_str_new).exists());
    assert_eq!(match read_dir(PathBuf::from(dir2_str_new)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 1);

    assert!(PathBuf::from(&file3_str_new).exists());
}
#[test]
fn t009_shouldnt_overwrite() {
    let work_dir: &str = "./tests_workdir/integration/009";
    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let dir1_str_org: &str = &{let mut s = work_dir.to_owned(); s.push_str("/d仿i+rб仿1"); s};
    let dir1_str_new: &str = &{let mut s = work_dir.to_owned(); s.push_str("/dir1"); s};
    match create_dir(&dir1_str_org) {Ok(_) => {}, Err(_) => panic!()};
    let dir2_str_org: &str = &{let mut s = work_dir.to_owned(); s.push_str("/dir1"); s};
    let dir2_str_new: &str = &{let mut s = work_dir.to_owned(); s.push_str("/dir1"); s};
    match create_dir(&dir2_str_org) {Ok(_) => {}, Err(_) => panic!()};

    let file1_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/f仿i+le1б.t仿xt"); s};
    let file1_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/file1.txt"); s};
    match File::create(&file1_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    let file2_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/file1.txt"); s};
    let file2_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/file1.txt"); s};
    match File::create(&file2_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let cnfg: Config = match Config::build(&vec![String::from(dir1_str_org), String::from("-r=3")
        ]) {Ok(c) => c, Err(_) => panic!()};    
    match run(&cnfg) {
        Ok(()) => {panic!()},
        Err(_) => {}
    }
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 2);
    assert!(PathBuf::from(&dir1_str_org).exists());
    assert!(PathBuf::from(&dir2_str_org).exists());

    assert_eq!(match read_dir(PathBuf::from(dir1_str_org)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 2);
    assert!(PathBuf::from(&file1_str_org).exists());
    assert!(PathBuf::from(&file2_str_org).exists());
}

#[test]
fn t010_copy_empty_valid() {
    let work_dir: &str = "./tests_workdir/integration/010";
    let dir1_str_org: &str = "./tests_workdir/integration/010/dir1";
    let dir1_str_new: &str = "./tests_workdir/integration/010/dir1-nrfcopy";
    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    match create_dir(&dir1_str_org) {Ok(_) => {}, Err(_) => panic!()};
    let cnfg: Config = match Config::build(&vec![String::from(dir1_str_org), String::from("-c")]) {Ok(c) => c, Err(_) => panic!()};    
    match run(&cnfg) {
        Ok(()) => {},
        Err(e) => {panic!("{}", e)}
    }
    assert!(PathBuf::from(dir1_str_org).exists());
    assert!(PathBuf::from(dir1_str_new).exists());
}
#[test]
fn t011_copy_file_invalid() {
    let work_dir: &str = "./tests_workdir/integration/011";
    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let file1_str_org: &str = &{let mut s = work_dir.to_owned(); s.push_str("/f仿i+le1б.t仿xt"); s};
    let file1_str_new: &str = &{let mut s = work_dir.to_owned(); s.push_str("/file1-nrfcopy.txt"); s};
    match File::create(&file1_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    
    let cnfg: Config = match Config::build(&vec![String::from(file1_str_org), String::from("-c")]) {Ok(c) => c, Err(_) => panic!()};    
    match run(&cnfg) {
        Ok(()) => {},
        Err(e) => {panic!("{}", e)}
    }

    assert!(PathBuf::from(&file1_str_new).exists());
    assert!(PathBuf::from(&file1_str_org).exists());

    assert_eq!(match read_dir(PathBuf::from(work_dir)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 2);
}
#[test]
fn t012_copy_dir_w_files_invalid_r0() {
    let work_dir: &str = "./tests_workdir/integration/012";
    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let dir1_str_org: &str = &{let mut s = work_dir.to_owned(); s.push_str("/d仿i+rб仿1"); s};
    let dir1_str_new: &str = &{let mut s = work_dir.to_owned(); s.push_str("/dir1-nrfcopy"); s};
    match create_dir(&dir1_str_org) {Ok(_) => {}, Err(_) => panic!()};

    let file1_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/f仿i+le1б.t仿xt"); s};
    let file1_str_new: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/f仿i+le1б.t仿xt"); s};
    match File::create(&file1_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    let file2_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/file2"); s};
    let file2_str_new: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/file2"); s};
    match File::create(&file2_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    
    let cnfg: Config = match Config::build(&vec![String::from(dir1_str_org), String::from("-c")]) {Ok(c) => c, Err(_) => panic!()};    
    match run(&cnfg) {
        Ok(()) => {},
        Err(e) => {panic!("{}", e)}
    }

    assert!(PathBuf::from(&dir1_str_new).exists());
    assert!(PathBuf::from(&dir1_str_org).exists());
    assert!(PathBuf::from(&file1_str_new).exists());
    assert!(PathBuf::from(&file2_str_new).exists());

    assert_eq!(match read_dir(PathBuf::from(work_dir)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 2);
    assert_eq!(match read_dir(PathBuf::from(dir1_str_new)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 0);
}
#[test]
fn t013_copy_dir_w_files_invalid_r1() {
    let work_dir: &str = "./tests_workdir/integration/013";
    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let dir1_str_org: &str = &{let mut s = work_dir.to_owned(); s.push_str("/d仿i+rб仿1"); s};
    let dir1_str_new: &str = &{let mut s = work_dir.to_owned(); s.push_str("/dir1-nrfcopy"); s};
    match create_dir(&dir1_str_org) {Ok(_) => {}, Err(_) => panic!()};

    let file1_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/f仿i+le1б.t仿xt"); s};
    let file1_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/file1.txt"); s};
    match File::create(&file1_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    let file2_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/file2"); s};
    let file2_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/file2"); s};
    match File::create(&file2_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    
    let cnfg: Config = match Config::build(&vec![String::from(dir1_str_org), String::from("-c"), String::from("-r=1")]) {Ok(c) => c, Err(_) => panic!()};    
    match run(&cnfg) {
        Ok(()) => {},
        Err(e) => {panic!("{}", e)}
    }

    assert!(PathBuf::from(&dir1_str_new).exists());
    assert!(PathBuf::from(&dir1_str_org).exists());
    assert!(PathBuf::from(&file1_str_new).exists());
    assert!(PathBuf::from(&file1_str_org).exists());
    assert!(PathBuf::from(&file2_str_new).exists());
    assert!(PathBuf::from(&file2_str_org).exists());

    assert_eq!(match read_dir(PathBuf::from(work_dir)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 2);
    assert_eq!(match read_dir(PathBuf::from(dir1_str_new)) {
        Ok(reader) => reader.count(),
        Err(_) => panic!()
    }, 2);
}
#[test]
fn t014_copy_dir_w_files_dirs_invalid_r3() {
    let work_dir: &str = "./tests_workdir/integration/014";
    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let dir1_str_org: &str = &{let mut s = work_dir.to_owned(); s.push_str("/d仿i+rб仿1"); s};
    let dir1_str_new: &str = &{let mut s = work_dir.to_owned(); s.push_str("/dir1-nrfcopy"); s};
    match create_dir(&dir1_str_org) {Ok(_) => {}, Err(_) => panic!()};

    let file1_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/f仿i+le1б.t仿xt"); s};
    let file1_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/file1.txt"); s};
    match File::create(&file1_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    let file2_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/file2"); s};
    let file2_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/file2"); s};
    match File::create(&file2_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    let dir2_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/dir仿2"); s};
    let dir2_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/dir2"); s};
    match create_dir(&dir2_str_org) {Ok(_) => {}, Err(_) => panic!()};

    let file3_str_org: &str = &{let mut s = dir2_str_org.to_owned(); s.push_str("/f仿i+le3б.mdб"); s};
    let file3_str_new: &str = &{let mut s = dir2_str_new.to_owned(); s.push_str("/file3.md"); s};
    match File::create(&file3_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let cnfg: Config = match Config::build(&vec![String::from(dir1_str_org), String::from("-r=3"), String::from("-c")
        ]) {Ok(c) => c, Err(_) => panic!()};    
    match run(&cnfg) {
        Ok(()) => {},
        Err(e) => {panic!("{}", e)}
    }
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 2);

    assert!(PathBuf::from(&dir1_str_new).exists());
    assert_eq!(match read_dir(PathBuf::from(dir1_str_new)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 3);

    assert!(PathBuf::from(&file1_str_new).exists());
    assert!(PathBuf::from(&file2_str_new).exists());
    assert!(PathBuf::from(&dir2_str_new).exists());
    assert_eq!(match read_dir(PathBuf::from(dir2_str_new)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 1);

    assert!(PathBuf::from(&file3_str_new).exists());
}
#[test]
fn t015_copy_shouldnt_overwrite() {
    let work_dir: &str = "./tests_workdir/integration/015";
    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let dir1_str_org: &str = &{let mut s = work_dir.to_owned(); s.push_str("/d仿i+rб仿1"); s};
    let dir1_str_new: &str = &{let mut s = work_dir.to_owned(); s.push_str("/dir1-nrfcopy"); s};
    match create_dir(&dir1_str_org) {Ok(_) => {}, Err(_) => panic!()};
    let dir2_str_org: &str = &{let mut s = work_dir.to_owned(); s.push_str("/dir1"); s};
    let dir2_str_new: &str = &{let mut s = work_dir.to_owned(); s.push_str("/dir1"); s};
    match create_dir(&dir2_str_org) {Ok(_) => {}, Err(_) => panic!()};

    let file1_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/f仿i+le1б.t仿xt"); s};
    let file1_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/file1.txt"); s};
    match File::create(&file1_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    let file2_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/file1.txt"); s};
    let file2_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/file1.txt"); s};
    match File::create(&file2_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let cnfg: Config = match Config::build(&vec![String::from(dir1_str_org), String::from("-r=3")
        ]) {Ok(c) => c, Err(_) => panic!()};    
    match run(&cnfg) {
        Ok(()) => {panic!()},
        Err(_) => {}
    }
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 2);
    assert!(PathBuf::from(&dir1_str_org).exists());
    assert!(PathBuf::from(&dir2_str_org).exists());

    assert_eq!(match read_dir(PathBuf::from(dir1_str_org)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 2);
    assert!(PathBuf::from(&file1_str_org).exists());
    assert!(PathBuf::from(&file2_str_org).exists());
}
#[test]
fn t016_not_all() {
    let work_dir: &str = "./tests_workdir/integration/016";
    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let dir1_str_org: &str = &{let mut s = work_dir.to_owned(); s.push_str("/d仿i+rб仿1"); s};
    let dir1_str_new: &str = &{let mut s = work_dir.to_owned(); s.push_str("/dir1"); s};
    match create_dir(&dir1_str_org) {Ok(_) => {}, Err(_) => panic!()};

    let file1_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/.f仿i+le1б.t仿xt"); s};
    let file1_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/.file1.txt"); s};
    match File::create(&file1_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    let file2_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/file2"); s};
    let file2_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/file2"); s};
    match File::create(&file2_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    let dir2_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/dir仿2"); s};
    let dir2_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/dir2"); s};
    match create_dir(&dir2_str_org) {Ok(_) => {}, Err(_) => panic!()};
    let dir3_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/.dir仿3"); s};
    let dir3_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/.dir3"); s};
    match create_dir(&dir3_str_org) {Ok(_) => {}, Err(_) => panic!()};

    let file3_str_org: &str = &{let mut s = dir2_str_org.to_owned(); s.push_str("/f仿i+le3б.mdб"); s};
    let file3_str_new: &str = &{let mut s = dir2_str_new.to_owned(); s.push_str("/file3.md"); s};
    match File::create(&file3_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let cnfg: Config = match Config::build(&vec![String::from(dir1_str_org), String::from("-r=3")
        ]) {Ok(c) => c, Err(_) => panic!()};    
    match run(&cnfg) {
        Ok(()) => {},
        Err(e) => {panic!("{}", e)}
    }
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 1);

    assert!(PathBuf::from(&dir1_str_new).exists());
    assert_eq!(match read_dir(PathBuf::from(dir1_str_new)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 4);

    assert!(!PathBuf::from(&file1_str_new).exists());
    assert!(PathBuf::from(&file2_str_new).exists());
    assert!(PathBuf::from(&dir2_str_new).exists());
    assert!(!PathBuf::from(&dir3_str_new).exists());
    assert_eq!(match read_dir(PathBuf::from(dir2_str_new)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 1);

    assert!(PathBuf::from(&file3_str_new).exists());
}
#[test]
fn t017_all() {
    let work_dir: &str = "./tests_workdir/integration/017";
    match remove_dir_all(Path::new(work_dir)) {_ => {}};
    match create_dir_all(Path::new(work_dir)) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let dir1_str_org: &str = &{let mut s = work_dir.to_owned(); s.push_str("/d仿i+rб仿1"); s};
    let dir1_str_new: &str = &{let mut s = work_dir.to_owned(); s.push_str("/dir1"); s};
    match create_dir(&dir1_str_org) {Ok(_) => {}, Err(_) => panic!()};

    let file1_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/.f仿i+le1б.t仿xt"); s};
    let file1_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/.file1.txt"); s};
    match File::create(&file1_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    let file2_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/file2"); s};
    let file2_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/file2"); s};
    match File::create(&file2_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};
    let dir2_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/dir仿2"); s};
    let dir2_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/dir2"); s};
    match create_dir(&dir2_str_org) {Ok(_) => {}, Err(_) => panic!()};
    let dir3_str_org: &str = &{let mut s = dir1_str_org.to_owned(); s.push_str("/.dir仿3"); s};
    let dir3_str_new: &str = &{let mut s = dir1_str_new.to_owned(); s.push_str("/.dir3"); s};
    match create_dir(&dir3_str_org) {Ok(_) => {}, Err(_) => panic!()};

    let file3_str_org: &str = &{let mut s = dir2_str_org.to_owned(); s.push_str("/f仿i+le3б.mdб"); s};
    let file3_str_new: &str = &{let mut s = dir2_str_new.to_owned(); s.push_str("/file3.md"); s};
    match File::create(&file3_str_org) {Ok(_) => {}, Err(e) => panic!("{}", e)};

    let cnfg: Config = match Config::build(&vec![String::from(dir1_str_org), String::from("-r=3"), String::from("-a")
        ]) {Ok(c) => c, Err(_) => panic!()};    
    match run(&cnfg) {
        Ok(()) => {},
        Err(e) => {panic!("{}", e)}
    }
    assert_eq!(match read_dir(PathBuf::from(work_dir)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 1);

    assert!(PathBuf::from(&dir1_str_new).exists());
    assert_eq!(match read_dir(PathBuf::from(dir1_str_new)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 4);

    assert!(PathBuf::from(&file1_str_new).exists());
    assert!(PathBuf::from(&file2_str_new).exists());
    assert!(PathBuf::from(&dir2_str_new).exists());
    assert!(PathBuf::from(&dir3_str_new).exists());
    assert_eq!(match read_dir(PathBuf::from(dir2_str_new)) {Ok(reader) => reader.count(), Err(_) => panic!()}, 1);

    assert!(PathBuf::from(&file3_str_new).exists());
}