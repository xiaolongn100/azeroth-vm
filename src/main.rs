extern crate azerothvm;
extern crate regex;

use azerothvm::*;
use regex::Regex;

fn main() {
    // TODO resovle args
    match std::env::var("JAVA_HOME") {
        Ok(home) => {
            start_vm("java.lang.String", "", &home);
        }
        Err(_) => {
            panic!("JAVA_HOME not set");
        }
    }
}

fn resolve_system_classpath(java_home: &str) -> Vec<String> {
    let mut java_home_dir = std::path::PathBuf::from(java_home);
    java_home_dir.push("jre/lib");
    if let Ok(files) = std::fs::read_dir(java_home_dir) {
        return files
            .map(|f| f.unwrap().path())
            .filter(|f| f.extension() == Some("jar".as_ref()))
            .map(|f| f.to_str().unwrap().to_string())
            .collect::<Vec<String>>();
    }
    panic!("JAVA_HOME not recognized");
}

fn resolve_user_classpath(user_classpath: &str) -> Vec<String> {
    return user_classpath
        .split(":")
        .map(|p| p.to_string())
        .collect::<Vec<String>>();
}

fn start_vm(class_name: &str, user_classpath: &str, java_home: &str) {
    let system_paths = resolve_system_classpath(java_home);
    let user_paths = resolve_user_classpath(user_classpath);
    let mut arena = mem::metaspace::ClassArena::init(user_paths, system_paths);
    let main_class = Regex::new(r"\.").unwrap().replace_all(class_name, "/");
    if let Some(klass) = arena.find_class(main_class.as_ref()) {
        // TODO allocate the main-thread stack to run class
        let main_stack = mem::stack::JavaStack::allocate(128 * 1024);
    } else {
        panic!("java.lang.ClassNotFoundException");
    }
}