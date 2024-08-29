use super::*;


#[test]
fn config_test_001_default() {
    let default: Config = Config::new();
    
    assert!(default.directories);
    assert!(default.files);
    assert_eq!(default.recursion, 0);
    assert!(!default.copy);
    assert_eq!(default.start, String::from("."));
    assert!(!default.silent);
    assert!(!default.log);

    assert!(!default.no_run);
    assert_eq!(default.usage, 'n');
}
#[test]
fn config_test_002_files_only() {
    let cnfg: Config = match Config::build(&vec![String::from("-f")]) {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };

    assert!(cnfg.files);
    assert!(!cnfg.directories);
}
#[test]
fn config_test_003_directories_only() {
    let cnfg: Config = match Config::build(&vec![String::from("-d")]) {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };

    assert!(!cnfg.files);
    assert!(cnfg.directories);
}
#[test]
fn config_test_004_invalid_f_and_d() {
    match Config::build(&vec![String::from("-d"), String::from("-f")]) {
        Ok(_) => panic!(""),
        Err(_) => {},
    }
}
#[test]
fn config_test_005_help() {
    let cnfg: Config = match Config::build(&vec![String::from("--help")]) {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };

    assert!(cnfg.no_run);
    assert_eq!(cnfg.usage, 'h');
}
#[test]
fn config_test_006_version() {
    let cnfg: Config = match Config::build(&vec![String::from("--version")]) {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };

    assert!(cnfg.no_run);
    assert_eq!(cnfg.usage, 'v');
}
#[test]
fn config_test_007_start_valid() {
    match Config::build(&vec![String::from("./hello")]) {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };
}
#[test]
fn config_test_008_recursion_valid() {
    let cnfg: Config = match Config::build(&vec![String::from("-r=15")]) {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };

    assert_eq!(cnfg.recursion, 15u8);
}
#[test]
fn config_test_009_replacer_valid() {
    let cnfg: Config = match Config::build(&vec![String::from("-R=cd")]) {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };

    assert_eq!(cnfg.replacer, String::from("cd"));
}
#[test]
#[should_panic]
fn config_test_010_recursion_invalid() {
    match Config::build(&vec![String::from("-r=1c5")]) {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };
}
#[test]
#[should_panic]
fn config_test_011_replacer_invalid() {
    match Config::build(&vec![String::from("-R=c=")]) {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };
}
#[test]
fn config_test_012_no_start() {
    let cnfg: Config = match Config::build(&vec![String::from("-r=2")]) {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };
    assert_eq!(cnfg.start, String::from("."))
}

#[test]
#[should_panic]
fn config_test_013_invalid_flag() {
    Config::build(&vec![String::from("-op")]).unwrap();
}

#[test]
fn config_test_014_combined_simple_flags() {
    let cnfg: Config = match Config::build(&vec![String::from("-IA")]) {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };
    assert_eq!(cnfg.start, String::from("."));
    assert_eq!(cnfg.illegal, false);
    assert_eq!(cnfg.to_ascii, false);
}



