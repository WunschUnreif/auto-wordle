use rand::prelude::*;
use std::{fs, io, path::Path};

fn main() -> Result<(), io::Error> {
    let dictionary = load_words(Path::new("english-words/words_alpha.txt"))?;

    println!("Total word count: {}", dictionary.len());

    println!("Length of the word to guess:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let word_length: usize = input.trim().parse().expect("Please input a valid number!");

    let mut candidates = dictionary
        .into_iter()
        .filter(|s| s.len() == word_length)
        .collect::<Vec<_>>();

    while !candidates.is_empty() {
        println!("Remaining words: {}", candidates.len());

        let choice = random_choose_word(&candidates).clone();
        println!("{}", choice);

        input.clear();
        io::stdin().read_line(&mut input)?;

        if input.trim().is_empty() {
            candidates = candidates.into_iter().filter(|s| s != &choice).collect();
            continue;
        }

        candidates = candidates
            .into_iter()
            .filter(|s| generate_judge(s, &choice) == input.trim())
            .collect();
    }

    Ok(())
}

fn load_words(from_path: &Path) -> Result<Vec<String>, io::Error> {
    let contents = fs::read_to_string(from_path)?;

    Ok(contents
        .lines()
        .filter(|s| s.len() > 0)
        .map(|s| s.trim().to_owned())
        .collect())
}

fn random_choose_word(from_dic: &Vec<String>) -> &String {
    from_dic.choose(&mut thread_rng()).unwrap()
}

fn generate_judge(truth: &String, test: &String) -> String {
    let mut truth = truth.chars().collect::<Vec<_>>();
    let mut answer = vec![' '; test.len()];

    for (i, ch) in test.chars().enumerate() {
        if truth[i] == ch {
            answer[i] = 'y';
            truth[i] = ' ';
        } else {
            if let Some(j) = truth.iter().position(|c| c == &ch) {
                answer[i] = 'x';
                truth[j] = ' ';
            } else {
                answer[i] = 'n';
            }
        }
    }

    answer.iter().collect()
}
