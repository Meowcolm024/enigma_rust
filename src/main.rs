mod enigma;
use std::io;
use std::io::Write;

macro_rules! parseNum {
    ($l: expr, u8) => {
        ($l).trim().parse::<u8>().expect("error: not a number");
    };
    ($l: expr, u64) => {
        ($l).trim().parse::<u64>().expect("error: not a number");
    };
}

fn main() {
    println!("enigma machine simulator");
    let input = prompt("seed: ".to_string());
    let seed: u64 = parseNum!(input, u64);
    let mut pos: [u8; 3] = [0; 3];
    for i in 1..4 {
        let input = prompt(format!("rotor position {}: ", i).to_string());
        pos[i - 1] = parseNum!(input, u8);
    }
    // main loop
    while true {
        let input = prompt("input text (q to quit): ".to_string());
        if input.trim() == "q" {
            break;
        } else {
            let mut machine = enigma::enigma(seed, (pos[0] % 26, pos[1] % 26, pos[2] % 26));
            let result = enigma::cipherstr(input.trim().to_string().to_uppercase(), &mut machine);
            println!("cipher text: {}", result);
        }
    }
}

fn prompt(msg: String) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    input.clone()
}
