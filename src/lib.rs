#![cfg_attr(feature = "test", feature(test))]

#[cfg(feature = "nightly")]
#[cfg(test)]
extern crate test;

const VOWELS: &str = "bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ";
const CONSONANTS: &str = "aeiouyAEIOUY";

fn stem(word: &str) -> (String, String) {
    let start = word.chars().take_while(|c| VOWELS.contains(*c)).collect::<String>();
    let end = word.chars().skip(start.len()).collect::<String>();
    (start, end)
}

fn stem2(word: &str) -> (&str, &str) {
    let pattern = |c: char| CONSONANTS.contains(c);
    let index = word.find(pattern).unwrap_or(0);
    (&word[0..index], &word[index..])
}

/// Splits a phase of two words and shuffles their beginnings.
///
/// Naive implementation
///
/// # Example
///
/// ```
/// extern crate zummi;
/// assert_eq!(zummi::zummi_naive("hello world"), Some(String::from("wello horld")));
/// ```
///
pub fn zummi_naive(phrase: &str) -> Option<String> {
    let mut words = phrase.split_whitespace();
    let (first, second) = (words.next(), words.next());
    if let (Some(first), Some(second)) = (first, second) {
        let (f, irst) = stem(first);
        let (s, econd) = stem(second);
        Some(s + &irst + " " + &f + &econd)
    } else {
        None
    }
}

/// Splits a phase of two words and shuffles their beginnings.
///
/// # Example
///
/// ```
/// extern crate zummi;
/// assert_eq!(zummi::zummi("hello world"), Some(String::from("wello horld")));
/// ```
///
pub fn zummi(phrase: &str) -> Option<String> {
    let mut words = phrase.split_whitespace();
    let (first, second) = (words.next(), words.next());
    if let (Some(first), Some(second)) = (first, second) {
        let (f, irst) = stem2(first);
        let (s, econd) = stem2(second);
        let capacity = f.len() + irst.len() + s.len() + econd.len();
        let spoonerism = String::with_capacity(capacity) + s + irst + " " + f + econd;
        Some(spoonerism)
    } else {
        None
    }
}

#[cfg(test)]
#[cfg(feature = "nightly")]
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
