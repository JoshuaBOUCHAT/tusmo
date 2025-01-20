const FILE: &'static str = include_str!("../list.txt");
mod precalculate;
#[derive(PartialEq, Eq, Debug, Clone)]
enum State {
    Placed,
    ToPlaced,
    Wrong,
}
use std::{
    char,
    collections::HashSet,
    str::{from_utf8, FromStr},
    sync::{
        atomic::{AtomicU8, AtomicUsize},
        Arc, Mutex,
    },
};
type Words = Vec<&'static [u8]>;
type Pattern = Vec<(u8, State)>;
type Word = &'static [u8];

use rayon::{iter::ParallelIterator, slice::ParallelSlice};
use std::io::Write;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

use State::*;

fn main() {
    let env: Vec<String> = std::env::args().collect();
    if env.len() == 1 {
        run();
        return;
    }

    //println!("The computed word: {}", compute_for_five());
    if env.len() != 3 {
        eprintln!("wron number of arg, usage ./tusmo <n> <c>\n");
        return;
    }
    let w_size: usize = env[1].trim().parse().expect("can't parse the number");
    let first_char: char = env[2].trim().chars().nth(0).expect("can't get first char");
    let words: Words = FILE
        .split('\n')
        .filter_map(|s| {
            if s.len() == w_size && (w_size == 5 || s.chars().nth(0).unwrap() == first_char) {
                Some(s.as_bytes())
            } else {
                None
            }
        })
        .collect();
    println!(
        "The estimate word is: {}\n",
        from_utf8(rayon_estiamte(&words, &words)).unwrap()
    )
}

fn run() {
    //pre_calculate();
    let w_size: usize = input("size of the word", false).expect("wrong size pass");

    let mut all_words: Words = FILE
        .split('\n')
        .filter_map(|s| {
            if s.len() == w_size {
                Some(s.as_bytes())
            } else {
                None
            }
        })
        .collect();

    let mut words = all_words.clone();

    let precalculate_word = if w_size != 5 {
        let first_char: char = input("first char of the word", false).expect("not good");
        words.retain(|&i| i[0] == first_char as u8);
        all_words.retain(|&i| i[0] == first_char as u8);
        precalculate::FIRST_WORD_AWNSER[w_size - 6][(first_char as u8 - 97) as usize]
    } else {
        "raies"
    };

    println!("The first word to try: {}", precalculate_word);
    let mut min_word = precalculate_word.as_bytes();
    println!(
        "Here is better word to try:{}",
        from_utf8(min_word).expect("tructruc")
    );
    while words.len() > 1 {
        println!("\nIl y a {} mots dans la liste", words.len());
        let pattern_s: String = input("The pattern:", false).expect("wrong pattern provided");
        let min_word_s: String = input("The word try", true).expect("wrong pattern provided");
        if !min_word_s.is_empty() {
            min_word = min_word_s.as_bytes();
        }
        let pattern: Pattern =
            guess_to_pattern(from_utf8(&min_word).expect("convert error"), &pattern_s)
                .expect("something happend!");

        words = filter_with_pattern(words, &pattern);
        println!(
            "Après le filtre la taille est :{}, voici la liste:",
            words.len()
        );
        //print_words(&words);
        println!("Computing {} words by {} ", words.len(), all_words.len());
        min_word = rayon_estiamte(&words, &all_words);
        println!(
            "Here is better word to try: {}",
            from_utf8(min_word).expect("tructruc")
        );
        if words.len() < 100 {
            for &word in &words {
                println!("{}", from_utf8(word).unwrap());
            }
        }
        println!("");
    }
}

fn _compute_for_five() -> String {
    let mut vec: Vec<&'static str> = FILE.split('\n').collect();
    vec = vec.into_iter().filter(|&i| i.len() == 5).collect();
    let words: Vec<_> = vec.into_iter().map(|s| s.as_bytes()).collect();
    let res = rayon_estiamte(&words, &words);
    String::from(std::str::from_utf8(res).unwrap())
}

fn _pre_calculate() {
    let mut file = std::fs::File::create("computation.txt").expect("truc");
    write!(&mut file, "[").expect("can't write");
    let wordss = _get_wordss();
    let mut map: [[&str; 26]; 5] = [[""; 26]; 5];
    for i in 6..=10 {
        for c in 97u8..=122 {
            let mut words = wordss[i - 6].clone();
            words.retain(|&t| t[0] == c);
            let slice = rayon_estiamte(&words, &words);
            map[i - 6][(c - 97) as usize] = from_utf8(slice).unwrap();
            println!("computed {i},{}", c as char);
            write!(&mut file, "\"{}\",", from_utf8(slice).unwrap())
                .expect("can't save computation");
        }
    }
    write!(&mut file, "]").expect("can't write");
}
fn filter_with_pattern(mut words: Words, pattern: &Pattern) -> Words {
    let used_table = [false; 26];
    for i in 0..pattern.len() {
        if let Placed = pattern[i].1 {
            let temp = pattern[i].0;
            words = words.into_iter().filter(|&s| s[i] == temp).collect();
            continue;
        }

        let c = pattern[i].0;
        if used_table[(c - 97) as usize] {
            continue;
        }
        let (count, strict) = count_char_pattern(&pattern, c);

        words = if strict {
            words
                .into_iter()
                .filter(|&s| count_char(s, c) == count && s[i] != c)
                .collect()
        } else {
            words
                .into_iter()
                .filter(|&s| count_char(s, c) >= count && s[i] != c)
                .collect()
        }
    }
    words
}
fn count_char_pattern(pattern: &Pattern, c: u8) -> (i32, bool) {
    let mut count = 0;
    let mut strict = false;
    for entry in pattern {
        if entry.0 == c {
            match entry.1 {
                Wrong => strict = true,
                _ => count += 1,
            }
        }
    }
    (count, strict)
}
fn count_char(word: Word, c: u8) -> i32 {
    let mut count = 0;
    for byte in word {
        if *byte == c {
            count += 1;
        }
    }
    count
}

fn _get_wordss() -> Vec<Vec<&'static [u8]>> {
    let mut vec: Vec<&'static str> = FILE.split('\n').collect();
    vec = vec
        .into_iter()
        .filter(|&i| i.len() > 5 && i.len() < 11)
        .collect();
    let mut ret = Vec::with_capacity(5);
    for _ in 0..5 {
        ret.push(vec![]);
    }
    for word in vec {
        ret[word.len() - 6].push(word.as_bytes())
    }
    ret
}
fn guess_to_pattern(word: &str, pattern: &str) -> Result<Vec<(u8, State)>, &'static str> {
    if word.len() != pattern.len() {
        eprintln!("word len:{} pattern size: {}", word.len(), pattern.len());
        return Err("pattern size doesn't match word size");
    }
    let word = word.as_bytes();
    let pattern = pattern.as_bytes();
    let ret = word
        .iter()
        .zip(pattern)
        .map(|a| match *a.1 {
            //_ for ToPlaced and - for already well placed
            b'_' => (*a.0, ToPlaced),
            b'-' => (*a.0, Placed),
            _ => (*a.0, Wrong),
        })
        .collect();
    Ok(ret)
}
fn input<T: FromStr>(msg: &str, nullable: bool) -> Result<T, T::Err> {
    loop {
        println!("{}", msg);
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("cant read through buffer");

        // Remove only the newline character
        let trimmed = buffer.trim_end_matches('\n');

        if !trimmed.is_empty() || nullable {
            return trimmed.parse::<T>();
        } else {
            println!("Empty input is not allowed. Please try again.");
        }
    }
}
fn get_pattern(guess: Word, awnser: Word) -> Pattern {
    let mut ret = Vec::with_capacity(awnser.len());
    let mut map = [0u8; 26];
    for i in 0..guess.len() {
        if guess[i] != awnser[i] {
            map[awnser[i] as usize - 97] += 1;
        }
    }
    for i in 0..guess.len() {
        let temp = guess[i];
        if awnser[i] == temp {
            ret.push((temp, Placed));
            continue;
        }
        let index = (temp - 97) as usize;
        if map[index] > 0 {
            map[index] -= 1;
            ret.push((temp, ToPlaced));
            continue;
        }
        ret.push((temp, Wrong));
    }

    ret
}
fn rayon_estiamte(words: &Words, all_words: &Words) -> Word {
    let min: AtomicUsize = AtomicUsize::new(usize::MAX);
    min.load(std::sync::atomic::Ordering::Relaxed);
    let min_word = Arc::new(Mutex::new(all_words[0]));
    let count_chunck: AtomicU8 = AtomicU8::new(0);
    let chunk_size = all_words.len() / 100;
    let chunk_size = if chunk_size == 0 { 1 } else { chunk_size };
    let verbose = words.len() > 500;
    let set: HashSet<&[u8]> = std::collections::HashSet::from_iter(words.iter().map(|&s| s));
    all_words.par_chunks(chunk_size).for_each(|chunck| {
        'loo: for &guess in chunck {
            let mut count = 0;
            for &test in words {
                count += filter_with_pattern(words.clone(), &get_pattern(guess, test)).len();
                if count > min.load(Relaxed) {
                    continue 'loo;
                }
            }
            if count < min.load(Relaxed) {
                min.store(count, Release);
                let mut lock = min_word.lock().unwrap();
                *lock = guess;
                println!(
                    "the min is: {} for word: {} so it's about {} possible for the next try",
                    count,
                    from_utf8(*lock).unwrap(),
                    count as f64 / all_words.len() as f64
                );
            }
        }
        count_chunck.fetch_add(1, Acquire);
        if verbose {
            println!("{}%", count_chunck.load(Relaxed));
        }
    });
    let str = *min_word.lock().unwrap();
    str
}
