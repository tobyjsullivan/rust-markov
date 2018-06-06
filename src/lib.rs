extern crate rand;

use std::collections::HashMap;
use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq)]
pub enum MarkovErr {
    Error,
    NotImplemented,
    NotSeen{w: String}
}

struct Chain {
    nodes: HashMap<String, i32>,
    edges: HashMap<(String, String), i32>
}

impl Chain {
    fn new() -> Chain {
        Chain {
            nodes: HashMap::new(),
            edges: HashMap::new()
        }
    }

    /*
    marks an ordered string pair as seen once
    */
    fn see(&mut self, a: &str, b: &str) {
        let key = (a.to_string(), b.to_string());
        let counter = self.nodes.entry(a.to_string()).or_insert(0);
        let weight = self.edges.entry(key).or_insert(0);
        *counter += 1;
        *weight += 1;
    }

    /*
    returns a random word, weighted by the probability that it is the next word to occur based on 
    what we've seen.
    */
    fn next(&self, seed: &str) -> Result<String, MarkovErr> {
        /*
        This part gets a bit cray. We're going to simulate a slot machine to choose the next word.
        We do this by picking a random value in the range [0..1) and using that as an index for
        the output word.
        Now, the way these indices work is you can think of all possible next words stacked with occurance proportional to
        their probabilities. If there is only one possible next word, it will fill the full range p(w) = 1.0.
        If there are two, equally likely words, they would each take up 0.5 of the range and so on.
        We have the total number of occurences of our key word, so we simply iterate through all edges starting
        at that word and add the probability of the the destination node to a running total.
        As soon as we exceed our target value, we know that's the one we want.
        */
        let index: f32 = thread_rng().gen_range(0.0, 1.0);
        let counter: i32 = *self.nodes.get(seed).unwrap_or(&0);
        if counter == 0 {
            return Err(MarkovErr::NotSeen{w: seed.to_string()});
        }

        let mut cursor: f32 = 0.0;
        for key in self.edges.keys() {
            if key.0 != seed {
                continue;
            }

            let weight: i32 = *self.edges.get(key).unwrap_or(&0);
            cursor += weight as f32 / counter as f32;
            if cursor > index {
                return Ok(key.1.clone());
            }
        }

        Err(MarkovErr::NotSeen{w: seed.to_string()})
    }
}

fn split(input: &str) -> Vec<String> {
    let mut s = input.to_lowercase();
    s.retain(|c| (c >= 'a' && c <= 'z') || c == ' ');
    let mut out = vec![];
    for word in s.split_whitespace() {
        out.push(word.to_string());
    }
    out
}

pub fn gen(input: &str, init: &str, length: i32) -> Result<Vec<String>, MarkovErr> {
    let mut chain = Chain::new();
    let mut first = "".to_string();
    let mut prev = "".to_string();
    for word in split(input) {
        if prev != "" {
            chain.see(&prev, &word);
        } else {
            first = word.clone();
        }
        prev = word;
    }
    chain.see(&prev, &first);
    
    let mut out = vec![init.to_string()];
    let mut w = init.to_string();
    for _ in 1..length {
        w = chain.next(&w)?;
        out.push(w.clone());
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_word() {
        assert_eq!(gen("hello", "hello", 1), Ok(vec!["hello".to_string()]));
    }

    #[test]
    fn test_two_words() {
        assert_eq!(gen("hello bob", "hello", 2), Ok(vec!["hello".to_string(), "bob".to_string()]));
    }

    #[test]
    fn test_split() {
        assert_eq!(split("Hello, world!"), vec!["hello".to_string(), "world".to_string()]);
    }
}

#[cfg(test)]
mod chain_tests {
    use super::*;

    #[test]
    fn test_new() {
        let chain = Chain::new();
        assert_eq!(chain.nodes.len(), 0);
        assert_eq!(chain.edges.len(), 0);
    }

    #[test]
    fn test_see_one() {
        let mut chain = Chain::new();
        chain.see("hello", "bob");
        assert_eq!(chain.nodes.entry("hello".to_string()).or_insert(0), &1);
        assert_eq!(chain.edges.entry(("hello".to_string(), "bob".to_string())).or_insert(0), &1);
    }

    #[test]
    fn test_see_two() {
        let mut chain = Chain::new();
        chain.see("australian", "koala");
        chain.see("australian", "kangaroo");
        assert_eq!(chain.nodes.entry("australian".to_string()).or_insert(0), &2);
        assert_eq!(chain.edges.entry(("australian".to_string(), "koala".to_string())).or_insert(0), &1);
        assert_eq!(chain.edges.entry(("australian".to_string(), "kangaroo".to_string())).or_insert(0), &1);
    }

    #[test]
    fn test_next() {
        let mut chain = Chain::new();
        chain.see("canadian", "hockey");
        assert_eq!(chain.next("canadian"), Ok("hockey".to_string()));
    }
}
