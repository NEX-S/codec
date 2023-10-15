mod base {
    pub mod base16;
    pub mod base32;
    pub mod base58;
    pub mod base64;
}

fn main ()
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <encoded-string>", args[0]);
        std::process::exit(1);
    }

    let encoded_string = &args[1];

    match base::base16::decode(encoded_string) {
        Some(decoded) => println!("base16: {}", String::from_utf8_lossy(&decoded)),
        None => println!("base16: Invalid base16 encoded string"),
    }

    match base::base32::decode(encoded_string) {
        Some(decoded) => println!("base32: {}", String::from_utf8_lossy(&decoded)),
        None => println!("base32: Invalid base32 encoded string"),
    }

    match base::base58::decode(encoded_string) {
        Some(decoded) => println!("base58: {}", String::from_utf8_lossy(&decoded)),
        None => println!("base58: Invalid base58 encoded string"),
    }

    match base::base64::decode(encoded_string) {
        Some(decoded) => println!("base64: {}", String::from_utf8_lossy(&decoded)),
        None => println!("base64: Invalid base64 encoded string"),
    }
}

