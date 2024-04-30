use std::{time::Duration, env, process::{Command, Stdio}, io::{prelude::*, BufReader}, fs::File, thread::sleep};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use regex::Regex;

pub const SECRET: &str = "secret";

pub fn wait(sec: u64) {
    let duration = Duration::from_secs(sec);
    sleep(duration);
}

pub fn pid() -> u32 {
    let pid = std::process::id();
    return pid;
}

pub fn check_env_var() -> bool {
    match env::var("SECRET") {
        Ok(s) => s == "exists",
        _ => false
    }
}

pub fn set_env_var() {
    let key = "SECRET";
    let value: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    env::set_var(key, value);
}

pub fn get_env_var() -> String {
    return env::var("SECRET").unwrap();
}

pub fn dump(pid: u32, dmp: &str) {
    let mut file_path = String::from("./out/");
    file_path.push_str(&dmp.to_string());
    Command::new("gcore")  // generate core
        .args(["-o", &file_path, "-a", &pid.to_string()])
        .output().ok();
}

pub fn analyze(dmp: &str, pattern: &str) {
    let mut file_path = String::from("./out/");
    file_path.push_str(&dmp.to_string());
    
    let dump = File::open(&file_path).unwrap();
    let regex = Regex::new(pattern).unwrap();

    let mut reader = BufReader::new(dump);
    let mut buffer = vec![];

    while let Ok(_) = reader.read_until(b'\n', &mut buffer) {
        if buffer.is_empty() { break; }

        let line = String::from_utf8_lossy(&buffer);

        if regex.is_match(&line) {
            println!("[!] Secret found in post-zeroize memory dump\n");

            let xxd_output = Command::new("xxd")    // hex dump
                .args(["-c 64", &file_path.to_string()])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            let grep_output = Command::new("grep") // grep
                //.args(["-e", &env::var("SECRET").unwrap()])
                .args(["-e", "SECRET"])
                .stdin(Stdio::from(xxd_output.stdout.unwrap()))
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            let output = grep_output.wait_with_output().unwrap();
            let response = String::from_utf8_lossy(&output.stdout);
            
            println!("Hex Dump\n--------");
            println!("{}", response);
        
            break;
        }
    }

    buffer.clear();
}