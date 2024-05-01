use zeroize::Zeroize;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("\nUsage:\t./zeroize-expose leak");
        println!("\t./zeroize-expose zero\n");
        return;
    }

    let mut leak: bool = false;
    if args[1] == "leak" {
        leak = true;
    }

    let pid = zeroize_expose::pid();
    println!("[+] PID {}", pid);

    let mut secret = zeroize_expose::generate_secret();

    println!("[+] Saving a pre-zeroize memory dump");
    zeroize_expose::wait(1);
    zeroize_expose::dump(pid, "pre"); 
    zeroize_expose::wait(1);          
    
    if leak {
        println!("\n\t=> {secret} <=\n"); // undermines zeroize
    }

    secret.zeroize(); // zero out the object from memory

    println!("[+] Saving a post-zeroize memory dump");
    zeroize_expose::wait(1); 
    zeroize_expose::dump(pid, "post");
    zeroize_expose::wait(1);

    if leak {
        println!("\n\tLooking for secret...\n");
        let post = ["post.", &pid.to_string()].concat();
        zeroize_expose::analyze(&post); // analyze
    }
}
