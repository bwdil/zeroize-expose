use zeroize::Zeroize;


fn main() {

    let mut secret: String = String::from(zeroize_expose::SECRET);
    println!("[~] Assigning initial value for {secret}");
    
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("\nUsage:\t./zeroize-test expose");
        println!("\t./zeroize-test zero\n");
        return;
    }

    let mut expose: bool = false;
    if args[1] == "expose" {
        expose = true;
    }

    let pid = zeroize_expose::pid(); // get pid 
    println!("[+] PID {}", pid);

    let secret_exists = zeroize_expose::check_env_var();
    if secret_exists {
        secret = zeroize_expose::get_env_var(); // get secret
    }
    else {
        println!("[-] Set environment var for secret");
        zeroize_expose::set_env_var(); // random value
        println!("[~] Moving secret to heap space");
        secret = zeroize_expose::get_env_var(); // get secret
    }

    println!("[+] Saving a pre-zeroize memory dump");
    zeroize_expose::dump(pid, "pre"); // pre-zeroize dump
    zeroize_expose::wait(1);               

    if expose {
        println!("[!] Exposed secret => {}", secret); // undermines zeroize
    }

    secret.zeroize(); // zero out the object from memory

    // find secret using strings post.123456 | grep SECRET
    println!("[+] Saving a post-zeroize memory dump");
    zeroize_expose::dump(pid, "post"); // post-zeroize dump
    zeroize_expose::wait(1);

    if expose {
        let post_dump = ["post.", &pid.to_string()].concat();
        zeroize_expose::analyze(&post_dump, &secret); // analyze
    }
}
