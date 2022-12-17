use std::env;
use std::io;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    // When the length of the command line argument slice is less than 1,
    // the command line argument input is accepted until it is greater than 1.
    while args.len() <= 1 {
        let mut s: String = String::new();
        io::stdin().read_line(&mut s).ok();
        args.push(s.trim().to_string());
    }

    let base_url = &args[1];

    // Combine URL and id
    let rslt = "https://drive.google.com/uc?export=view&id=".to_string() + &base_url[32..65];

    println!("id: {:?}", rslt);
}
