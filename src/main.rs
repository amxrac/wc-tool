use clap::Parser;
use std::fs;

fn main() {
    let args = Args::parse();

    if let Some(cmd) = args.number_of_bytes {
        println!(
            "{:?}",
            fs::read_to_string(cmd)
                .expect("unable to read file")
                .bytes()
                .len()
        );
    } else if let Some(cmd) = args.number_of_lines {
        let mut count = 0;
        for _ in fs::read_to_string(cmd)
            .expect("unable to read file")
            .lines()
        {
            count += 1;
        }
        println!("{:?}", count);
    } else if let Some(cmd) = args.number_of_words {
        let content = fs::read_to_string(cmd).expect("unable to read file");
        let content: Vec<_> = content.split_whitespace().collect();
        print!("{:?}", content.len());
    } else if let Some(cmd) = args.number_of_characters {
        let content = fs::read_to_string(cmd).expect("unable to read file");
        let content: Vec<_> = content.chars().collect();
        print!("{:?}", content.len());
    } else {
        let content = fs::read_to_string(args.filepath.unwrap()).expect("unable to read file");
        let mut count = 0;
        for _ in content.lines() {
            count += 1;
        }
        let word_content: Vec<_> = content.split_whitespace().collect();
        println!(
            "{:?} {:?} {:?}",
            content.bytes().len(),
            count,
            word_content.len()
        );
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'c')]
    number_of_bytes: Option<String>,

    #[arg(short = 'l')]
    number_of_lines: Option<String>,

    #[arg(short = 'w')]
    number_of_words: Option<String>,

    #[arg(short = 'm')]
    number_of_characters: Option<String>,

    filepath: Option<String>,
}
