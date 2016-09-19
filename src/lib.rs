const VOWELS: &'static str = "bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ";
const CONSONANTS: &'static str = "aeiouyAEIOUY";

fn stem(word: &str) -> (String, String) {
    let start = word.chars()
        .take_while(|c| VOWELS.contains(*c))
        .collect::<String>();
    let end = word.chars()
        .skip(start.len())
        .collect::<String>();
    (start, end)
}

fn stem2(word: &str) -> (&str, &str) {
    let pattern = |c: char| CONSONANTS.contains(c);
    let index = word.find(pattern).unwrap_or(0);
    (&word[0..index], &word[index..])
}


pub fn zummi_naiv(phrase: &str) -> Option<String> {
    let mut words = phrase.split_whitespace();
    let (first, second) = (words.next(), words.next());
    if let (Some(first), Some(second)) = (first, second) {
        let (f, irst) = stem(first);
        let (s, econd) = stem(second);
        return Some(s + &irst + " " + &f + &econd);
    }
    None
}

pub fn zummi(phrase: &str) -> Option<String> {
    let mut words = phrase.split_whitespace();
    let (first, second) = (words.next(), words.next());
    if let (Some(first), Some(second)) = (first, second) {
        let (f, irst) = stem2(first);
        let (s, econd) = stem2(second);
        return Some(
            String::with_capacity(f.len() + irst.len() + s.len() + econd.len()) +
            &s + &irst + " " + &f + &econd
            );
    }
    None
}

//#![feature(test)]
//extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    use test::black_box;
    use test::Bencher;

    #[bench]
    fn zummi_bench(b: &mut Bencher) {
        b.iter(|| {
            let mix = zummi("stinkender fisch");
            black_box(mix);
        });
    }

    #[bench]
    fn zummi_naiv_bench(b: &mut Bencher) {
        b.iter(|| {
            let mix = zummi_naiv("stinkender fisch");
            black_box(mix);
        });
    }

}
