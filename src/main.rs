use rand::prelude::*;
use std::io::stdin;

const WIN: &str = "🟩 🟩 🟩 🟩";
const GREEN: char = '🟩';
const YELLOW: char = '🟨';
const WHITE: char = '⬜';
const SEPARATOR: char = ' ';

fn calc_green_and_yellow(guess: &[u8], secret: &[u8]) -> String {
    assert!(
        guess.len() == secret.len(),
        "answer and solution are of different length"
    );

    let mut secret = secret.to_vec();
    let mut guess = guess.to_vec();

    let mut out = String::from("");

    let mut i = 0;

    while i < secret.len() {
        if out.len() > 0 {
            out.push(SEPARATOR)
        }

        if guess[i] == secret[i] {
            out.push(GREEN);
            secret.remove(i);
            guess.remove(i);
        } else if secret.contains(&guess[i]) {
            out.push(YELLOW);
            i += 1;
        } else {
            out.push(WHITE);
            i += 1;
        }
    }

    out
}

fn generate_solution(random: fn(u8) -> u8) -> [u8; 4] {
    [random(4), random(4), random(4), random(4)]
}

fn random_generator(max: u8) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=max)
}

#[test]
fn test_green_and_yellow() {
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 3, 4], &[1, 2, 3, 4]),
        "🟩 🟩 🟩 🟩".to_string()
    );
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 3, 5], &[1, 2, 3, 4]),
        "🟩 🟩 🟩 ⬜".to_string()
    );
    assert_eq!(
        calc_green_and_yellow(&[4, 3, 2, 1], &[1, 2, 3, 4]),
        "🟨 🟨 🟨 🟨".to_string()
    );
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 3, 1], &[1, 2, 3, 4]),
        "🟩 🟩 🟩 ⬜".to_string()
    );
    assert_eq!(
        calc_green_and_yellow(&[1, 1, 1, 1], &[1, 2, 3, 4]),
        "🟩 ⬜ ⬜ ⬜".to_string()
    );
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 2, 2], &[2, 2, 2, 1]),
        "🟨 🟩 🟩 🟨".to_string()
    );
    assert_eq!(
        calc_green_and_yellow(&[1, 3, 3, 2], &[2, 2, 2, 1]),
        "🟨 ⬜ ⬜ 🟨".to_string()
    );
}

fn main() {
    let stdin = stdin();

    let mut buf = String::new();

    let secret = generate_solution(random_generator);
    println!("solution: {:?}", secret);

    loop {
        buf.clear();
        println!("guess: ");
        stdin.read_line(&mut buf).unwrap();
        let guess: Result<Vec<u8>, _> = buf.trim().split(' ').map(|s| s.parse()).collect();
        let guess = guess.unwrap();
        let squares = calc_green_and_yellow(&guess, &secret);
        println!("{:?} got {}", &guess, squares);
        if squares == WIN {
            print!("Congrats!! You got it all right..");
            break;
        }
    }
}
