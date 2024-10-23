use std::collections::HashMap;
use rand::Rng;

struct Chain {
    corpus: String,
    dataset: HashMap<[String; 2], Vec<String>>
}

impl Chain {
    fn train(&mut self) {
	let words = self.corpus.split_whitespace().collect::<Vec<_>>(); // Split based on whitespaces
        let words_windows = words.as_slice().windows(3);

        for window in words_windows {
            let key: [String; 2] = [window[0].to_string(), window[1].to_string()];
            let value = window[2].to_string();

            self.dataset
                .entry(key)
                .or_insert_with(Vec::new)
                .push(value);
        }
    }


    fn predict(&self, n_words: usize) {
            let mut rng = rand::thread_rng();
            let mut sentence: Vec<String> = Vec::new();

            let mut previous_words = self.dataset.keys().nth(rng.gen_range(0..self.dataset.keys().len())).unwrap().clone();

            for _ in 0..(n_words - 1) {
                let possibilities = &self.dataset.get(&previous_words).unwrap();
                let next_word = &possibilities[rng.gen_range(0..possibilities.len())];
                
                sentence.push(next_word.clone());
                previous_words = [previous_words[1].clone(), next_word.clone()];

                println!("{:?}", sentence);
            }
    }
}


fn main() {
        let mut chain = Chain {
            corpus:  "The quick brown fox jumps over the lazy dog".to_lowercase(),
            dataset: HashMap::new(),
        };

        chain.train();

	println!("{:?}", chain.dataset);
	chain.predict(10);
}
