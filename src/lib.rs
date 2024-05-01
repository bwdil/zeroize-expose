use std::{time::Duration, process::{Command, Stdio}, thread::sleep};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn wait(sec: u64) {
    let duration = Duration::from_secs(sec);
    sleep(duration);
}

pub fn pid() -> u32 {
    let pid = std::process::id();
    return pid;
}

pub fn generate_secret() -> String {
    let value: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    return value;
}

pub fn dump(pid: u32, dmp: &str) {
    let mut file_path = String::from("./out/");
    file_path.push_str(&dmp.to_string());
    Command::new("gcore")  // generate core
        .args(["-o", &file_path, "-a", &pid.to_string()])
        .output().ok();
}

pub fn analyze(dmp: &str) {
    let mut file_path = String::from("./out/");
    file_path.push_str(&dmp.to_string());

    let xxd_output = Command::new("xxd")    // hex dump
        .args(["-c 128", &file_path.to_string()])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let grep_output = Command::new("grep") // grep
        .args(["-e", "=>"])
        .stdin(Stdio::from(xxd_output.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output = grep_output.wait_with_output().unwrap();
    let response = String::from_utf8_lossy(&output.stdout);
    
    println!("Hex Dump");
    println!("---------");
    println!("{}", response);
}
