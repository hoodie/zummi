const VOWELS: &str = "bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ";
const CONSONANTS: &str = "aeiouyAEIOUY";

fn stem(word: &str) -> (String, String) {
    let start = word
        .chars()
        .take_while(|c| VOWELS.contains(*c))
        .collect::<String>();
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

/// Broken version where I accidentally cause an unnecessary allocation.
///
/// # Example
///
/// ```
/// assert_eq!(zummi::zummi_broke("hello world"), Some(String::from("wello horld")));
/// ```
///
pub fn zummi_broke(phrase: &str) -> Option<String> {
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

/// Splits a phase of two words and shuffles their beginnings.
///
/// # Example
///
/// ```
/// assert_eq!(zummi::zummi("hello world"), Some(String::from("wello horld")));
/// ```
///
pub fn zummi(phrase: &str) -> Option<String> {
    let mut words = phrase.split_whitespace();
    let (first, second) = (words.next(), words.next());
    if let (Some(first), Some(second)) = (first, second) {
        let (f, irst) = stem2(first);
        let (s, econd) = stem2(second);
        let capacity = f.len() + irst.len() + s.len() + econd.len() + " ".len();
        let spoonerism = String::with_capacity(capacity) + s + irst + " " + f + econd;
        Some(spoonerism)
    } else {
        None
    }
}
