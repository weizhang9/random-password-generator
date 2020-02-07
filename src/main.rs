extern crate rand;
use std::io;

fn get_len() -> u8 {
    loop {
        println!("Input desired password length:");
    
        let mut length = String::new();
        io::stdin().read_line(&mut length).expect("couldn't understand stdin");
    
        match length.trim().parse::<u8>() {
            Ok(v) => return v,
            Err(e) => println!("Couldn't understand input: {}", e),
        };
    }
}

fn get_charset() -> u8 {
                                                                    
    loop {
        println!("Choose desired charset for your password:\n\
        1) Alphanumeric + Special characters\n\
        2) Alphanumeric\n\
        3) Alphabetic\n\
        4) Numberic");
    
        let mut charset = String::new();
        io::stdin().read_line(&mut charset).expect("couldn't understand stdin");
    
        match charset.trim().parse::<u8>() {
            Ok(v) => return v,
            Err(e) => println!("Couldn't understand input: {}", e),
        };
    }
}

fn generate(len: u8, option: u8) -> String {
    use rand::Rng;
    const ALPHANUMSPEC: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789!()-.?[]_`~;:!@#$%^&*+=";
    const ALPHANUM: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789";
    const ALPHA: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz";
    const NUM: &[u8] = b"0123456789"; 

    let charset: &[u8] = match option {
        1 => ALPHANUMSPEC,
        2 => ALPHANUM,
        3 => ALPHA,
        4 => NUM,
        _ => b""
    };
    let mut rng = rand::thread_rng();

    let password: String = (0..len)
        .map(|_| {
            let idx = rng.gen_range(0, charset.len());
            charset[idx] as char
        })
        .collect();
    password
}

fn is_generated(out: usize, input: u8) -> bool {
    let o : u8 = out as u8;
    o == input
}

fn main() {
    loop {
        let password_len: u8 = get_len();
        let charset: u8 = get_charset();
        let password: String = generate(password_len, charset);
        println!("{:?}", password);
        if is_generated(password.chars().count(), password_len) {
            break;
        }
    }
}