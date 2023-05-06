use clap::Parser;
use rand::Rng;

/// Simple program that generates a bip39 compatible mneumonic phrase.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Length of the generated phrase. [max: 2048].
    #[arg(short, long, default_value_t = 24)]
    length: u16,
}

const WORDS: &str = include_str!("../resources/words.txt");
const NUM_WORDS: u16 = 2048;

fn main() {
    let args = Args::parse();
    if args.length > NUM_WORDS {
        println!("Error: Max length is {}", NUM_WORDS)
    }

    let mut words = get_words();
    let mut rng = rand::thread_rng();
    let mut chosen_words: Vec<String> = Vec::with_capacity(args.length.into());
    for _ in 0..args.length {
        loop {
            let index = rng.gen_range(0..NUM_WORDS) as usize;
            let word = words.get_mut(index).unwrap();
            if word.is_some() {
                chosen_words.push(word.take().unwrap());
                break;
            }
        }
    }
    println!("{}", chosen_words.join(" ").trim_end_matches('\0'));
}

fn get_words() -> Vec<Option<String>> {
    WORDS.lines().map(|line| Some(line.to_string())).collect()
}
