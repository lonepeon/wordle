use clap::Parser;
use rand::Rng;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Seed to use for randomization
    #[clap(short, long, default_value_t = 0)]
    seed: usize,
}

fn main() {
    let args = Args::parse();
    let seed = if args.seed == 0 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..1024)
    } else {
        args.seed
    };

    let word = wordle::dictionary::pick_word(seed);

    wordle::cli::play(word);
}
