#[macro_use]
extern crate lazy_static;
type Sstr = &'static str;
type Sstrs = Vec<&'static str>;

#[derive(Debug)]
enum State {
    Placed(char),
    Toplace(char),
    Bad(char),
}
impl State {
    fn get_char(&self) -> char {
        match self {
            State::Placed(c) | State::Toplace(c) | State::Bad(c) => *c,
        }
    }
}
use std::{io::Write, usize};

use State::*;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    println!("{:?}", &args);
    let (w_size_s, letter) = if args.len() == 3 {
        (args[1].clone(), args[2].clone())
    } else {
        (input("give w_size\n"), input("first letter\n"))
    };
    let c = letter.chars().nth(0).expect("can't acess letter");
    let mut words = get_words(w_size_s.trim().parse().expect("can't parse messaege"));

    words = words
        .into_iter()
        .filter(|i| (i.chars().nth(0).expect("error") == c))
        .collect();
    loop {
        /*for word in &words {
            println!("{}", word);
        }*/
        println!("The try to word: {}", get_guess(&words));

        let guess = input("the word you try:\n").to_ascii_uppercase();
        let pattern = input("the pattern you got:\n").to_ascii_uppercase();
        let states = transform(&guess, &pattern);
        println!("{:?}", &states);
        words = apply(words, states);
    }
    /*let guess = input("guess").to_ascii_uppercase();
    let word = input("word").to_ascii_uppercase();
    let pattern = guess_to_patten(word.trim(), guess.trim());
    println!("{:?}", &pattern);*/
}
fn get_vec_words() -> Sstrs {
    lazy_static! {
        static ref WORDS_FILE: String =
            std::fs::read_to_string("word.txt").expect("can't find the file!");
        static ref WORDS_VEC: Sstrs = WORDS_FILE.split('\n').collect();
    };
    WORDS_VEC.clone()
}
fn get_words(cur: usize) -> Sstrs {
    let mut vec_words = get_vec_words();
    vec_words = vec_words.into_iter().filter(|i| i.len() == cur).collect();
    vec_words
}
fn transform(guess: &str, pattern: &str) -> Vec<State> {
    let mut vec = vec![];

    let guess: Vec<char> = guess.chars().collect();
    let pattern: Vec<char> = pattern.chars().collect();
    //println!("{:?} {:?}", &guess, &pattern);
    std::io::stdout().flush();
    for i in 0..guess.len() {
        let c = guess[i];
        print!("{c}");
        let state = match pattern[i] {
            '-' => Placed(c),
            '_' => Toplace(c),
            ' ' => Bad(c),
            '\n' => break,
            a => panic!("wrong pattern {a}"),
        };
        vec.push(state);
    }
    vec
}
fn guess_to_patten(word: &str, guess: &str) -> Vec<State> {
    let mut arr = [0u8; 26];
    let len = guess.len();
    let word: Vec<_> = word.chars().collect();
    let guess: Vec<_> = guess.chars().collect();
    let mut vec = Vec::with_capacity(len);
    //println!("{:?} {:?}", &word, &guess);
    for i in 0..len {
        if word[i] != guess[i] {
            arr[(word[i] as u8 - 65u8) as usize] += 1;
        }
    }
    for i in 0..len {
        let c = guess[i];
        if c == word[i] {
            vec.push(Placed(c));
        } else {
            let index = (guess[i] as u8 - 65u8) as usize;
            if arr[index] > 0 {
                arr[index] -= 1;
                vec.push(Toplace(c));
            } else {
                vec.push(Bad(c));
            }
        }
    }

    vec
}

fn apply(mut words: Sstrs, key: Vec<State>) -> Sstrs {
    for i in 0..key.len() {
        match key[i] {
            Placed(c) => {
                words = words
                    .into_iter()
                    .filter(|j| j.chars().nth(i).unwrap() == c)
                    .collect();
            }
            Toplace(c) => {
                /*let count = key
                .iter()
                .filter(|i| match i {
                    Bad(_) => false,
                    Placed(k) => *k == c,
                    Toplace(k) => *k == c,
                })
                .count();*/
                words = words
                    .into_iter()
                    .filter(|j| j.chars().nth(i).expect("truc") != c)
                    //.filter(|j| j.chars().filter(|i| *i == c).count() == count)
                    .collect();
            }
            Bad(c) => {
                words = words
                    .into_iter()
                    .filter(|j| j.chars().nth(i).unwrap() != c)
                    //.filter(|j|key.chars().filter(|k|*k==c)==0)
                    .collect();

                let count = key
                    .iter()
                    .filter(|j| match &j {
                        Placed(l) => *l == c,
                        Toplace(l) => *l == c,
                        Bad(_) => false,
                    })
                    .count();
                words = words
                    .into_iter()
                    .filter(|i| i.chars().filter(|k| *k == c).count() == count)
                    .collect();
            }
        }
    }
    words
}
fn input(msg: &str) -> String {
    println!("{msg}");
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).expect("can't read");
    ret
}
fn get_guess(words: &Sstrs) -> Sstr {
    let mut min = usize::MAX;
    let mut min_word = "NO WORD AVAILABLE";
    let mut i = 0;

    for guess in words {
        let mut sum = 0;
        for word in words {
            let words2 = apply(words.clone(), guess_to_patten(word, guess));
            sum += words2.len();
        }
        println!("i:{i} sum:{sum}");
        if sum < min {
            //println!("{}", &guess);
            min_word = guess;
            min = sum;
        }
        i += 1;
    }
    min_word
}
fn apply2(mut words: Vec<&Vec<char>>, key: Vec<State>) -> usize {
    let len = key.len();

    for i in 0..len {
        let mut vec: Vec<&Vec<char>> = Vec::with_capacity(words.len());
        match key[i] {
            Placed(c) => {
                /*words = words
                .into_iter()
                .filter(|j| j.chars().nth(i).unwrap() == c)
                .collect();*/
                for j in 0..words.len() {
                    if words[j][i] == c {
                        vec.push(words[j]);
                    }
                }
                words = vec;
            }
            Toplace(c) => {
                /*let count = key
                .iter()
                .filter(|i| match i {
                    Bad(_) => false,
                    Placed(k) => *k == c,
                    Toplace(k) => *k == c,
                })
                .count();*/
                /*words = words
                .into_iter()
                .filter(|j| j.chars().nth(i).expect("truc") != c)
                //.filter(|j| j.chars().filter(|i| *i == c).count() == count)
                .collect();*/
                for j in 0..words.len() {
                    if words[j][i] != c {
                        vec.push(words[j]);
                    }
                }
                words = vec;
            }
            Bad(c) => {
                /*.into_iter()
                .filter(|j| j.chars().nth(i).unwrap() != c)
                //.filter(|j|key.chars().filter(|k|*k==c)==0)
                .collect();*/
                for j in 0..words.len() {
                    if words[j][i] != c {
                        vec.push(words[j]);
                    }
                }
                words = vec;
                vec = Vec::with_capacity(words.len());

                let count = key
                    .iter()
                    .filter(|j| match &j {
                        Placed(l) => *l == c,
                        Toplace(l) => *l == c,
                        Bad(_) => false,
                    })
                    .count();
                /* words = words
                .into_iter()
                .filter(|i| i.chars().filter(|k| *k == c).count() == count)
                .collect();*/

                for j in 0..words.len() {
                    let mut k = 0;
                    for l in 0..len {
                        if words[j][l] == c {
                            k += 1;
                        }
                    }
                    if k == count {
                        vec.push(words[j]);
                    }
                }
                words = vec;
            }
        }
    }
    words.len()
}
