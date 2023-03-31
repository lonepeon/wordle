#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Hint {
    Misplaced(char),
    Exact(char),
    Absent(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Guess([Hint; 5]);

impl Guess {
    pub fn guessed_letters(&self) -> &[Hint; 5] {
        &self.0
    }

    pub fn word(&self) -> [char; 5] {
        self.0.map(|hint| match hint {
            Hint::Misplaced(c) => c,
            Hint::Exact(c) => c,
            Hint::Absent(c) => c,
        })
    }

    pub fn is_guessed(&self) -> bool {
        for hint in self.0 {
            if let Hint::Exact(_) = hint {
                continue;
            }

            return false;
        }

        true
    }
}

pub struct Word([char; 5]);

impl Word {
    pub fn new(s: [char; 5]) -> Self {
        Word(s)
    }

    pub fn assess(&self, guess: [char; 5]) -> Guess {
        let mut distribution = WordDistribution::new();
        self.0.iter().for_each(|c| distribution.incr(c));

        let mut hints = guess.map(Hint::Absent);

        for i in 0..self.0.len() {
            if guess[i] == self.0[i] {
                hints[i] = Hint::Exact(guess[i]);
                distribution.decr(&guess[i]);
            }
        }

        for (guess_index, guess_char) in guess.iter().enumerate() {
            match hints[guess_index] {
                Hint::Exact(_) | Hint::Misplaced(_) => continue,
                Hint::Absent(_) => (),
            };

            self.0.iter().any(|expected_char| {
                if expected_char == guess_char && distribution.decr(guess_char) {
                    hints[guess_index] = Hint::Misplaced(guess[guess_index]);
                    return true;
                }

                false
            });
        }

        Guess(hints)
    }
}

struct FrequencyDistribution<'a> {
    pub value: &'a char,
    pub counter: i32,
}

struct WordDistribution<'a>(Vec<FrequencyDistribution<'a>>);

impl<'a> WordDistribution<'a> {
    pub fn new() -> Self {
        WordDistribution(Vec::new())
    }

    pub fn incr(&mut self, c: &'a char) {
        for mut frequency in self.0.iter_mut() {
            if frequency.value == c {
                frequency.counter += 1;
                return;
            }
        }

        self.0.push(FrequencyDistribution {
            value: c,
            counter: 1,
        });
    }

    pub fn decr(&mut self, c: &char) -> bool {
        for mut counter in self.0.iter_mut() {
            if counter.value != c {
                continue;
            }

            if counter.counter - 1 < 0 {
                return false;
            }
            counter.counter -= 1;
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    mod word {
        use super::super::{
            Hint::{Absent, Exact, Misplaced},
            *,
        };

        #[test]
        fn guess_match() {
            let guess = ['a', 'b', 'c', 'd', 'e'];
            let hint = Word::new(['a', 'b', 'c', 'd', 'e']).assess(guess);

            assert_eq!(Guess(guess.map(Exact)), hint);
        }

        #[test]
        fn guess_no_match() {
            let guess = ['f', 'g', 'h', 'i', 'j'];
            let hint = Word::new(['a', 'b', 'c', 'd', 'e']).assess(guess);

            assert_eq!(Guess(guess.map(Absent)), hint);
        }

        #[test]
        fn guess_misplaced() {
            let guess = ['b', 'c', 'd', 'e', 'a'];
            let hint = Word::new(['a', 'b', 'c', 'd', 'e']).assess(guess);

            assert_eq!(Guess(guess.map(Misplaced)), hint);
        }

        #[test]
        fn guess_a_bit_of_everything() {
            let hint = Word::new(['a', 'b', 'c', 'd', 'e']).assess(['a', 'b', 'd', 'c', 'f']);

            assert_eq!(
                Guess([
                    Exact('a'),
                    Exact('b'),
                    Misplaced('d'),
                    Misplaced('c'),
                    Absent('f'),
                ]),
                hint,
            );
        }

        #[test]
        fn guess_same_letters_multiple_times() {
            let hint = Word::new(['a', 'b', 'a', 'd', 'e']).assess(['a', 'a', 'b', 'c', 'a']);
            assert_eq!(
                Guess([
                    Exact('a'),
                    Misplaced('a'),
                    Misplaced('b'),
                    Absent('c'),
                    Absent('a'),
                ]),
                hint,
            );
        }
    }

    mod guess {
        use super::super::*;

        #[test]
        fn word() {
            let guess = Guess([
                Hint::Misplaced('r'),
                Hint::Exact('i'),
                Hint::Absent('g'),
                Hint::Exact('h'),
                Hint::Misplaced('t'),
            ]);

            assert_eq!(['r', 'i', 'g', 'h', 't'], guess.word())
        }

        #[test]
        fn state_is_guessed_true() {
            let guess = Guess([
                Hint::Exact('r'),
                Hint::Exact('i'),
                Hint::Exact('g'),
                Hint::Exact('h'),
                Hint::Exact('t'),
            ]);

            assert!(guess.is_guessed(), "word should be guessed")
        }

        #[test]
        fn state_is_guessed_false() {
            let guess = Guess([
                Hint::Misplaced('r'),
                Hint::Exact('i'),
                Hint::Absent('g'),
                Hint::Exact('h'),
                Hint::Misplaced('t'),
            ]);

            assert!(!guess.is_guessed(), "word should not be guessed")
        }
    }
}
