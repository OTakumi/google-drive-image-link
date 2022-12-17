use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let base_url = &args[1];

    // Combine URL and id
    let rslt = "https://drive.google.com/uc?export=view&id=".to_string() + &base_url[32..65];

    println!("id: {:?}", rslt);
}
