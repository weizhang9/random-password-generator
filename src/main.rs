extern crate rand;
use std::io;

fn get_len() -> u8 {
    loop {
        println!("Input desired password length:");
    
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("couldn't understand stdin");
    
        match guess.trim().parse::<u8>() {
            Ok(v) => return v,
            Err(e) => println!("Couldn't understand input {}", e),
        };
    }
}

fn generate(len: u8) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789!@$%^&*()~#";
    let mut rng = rand::thread_rng();

    let password: String = (0..len)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    return password;
}

fn is_generated(out: usize, input: u8) -> bool {
    let o : u8 = out as u8;
    o == input
}

fn main() {
    loop {
        let password_len: u8 = get_len();
        let password: String = generate(password_len);
        println!("{:?}", password);
        if is_generated(password.chars().count(), password_len) {
            break;
        }
    }
}