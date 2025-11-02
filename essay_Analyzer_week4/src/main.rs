use std::io;

struct EssayAnalyzer {
    sentence: String,
}

impl EssayAnalyzer {
    // Constructor
    fn new(sentence: String) -> Self {
        EssayAnalyzer { sentence }
    }

    // Clean the sentence by removing punctuation
    fn clean_sentence(&self) -> String {
        self.sentence
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect()
    }

    // Find the longest word
    fn longest_word(&self) -> Option<String> {
        let cleaned = self.clean_sentence();
        cleaned
            .split_whitespace()
            .max_by_key(|word| word.len())
            .map(|w| w.to_string())
    }

    // Find the shortest word
    fn shortest_word(&self) -> Option<String> {
        let cleaned = self.clean_sentence();
        cleaned
            .split_whitespace()
            .min_by_key(|word| word.len())
            .map(|w| w.to_string())
    }

    // Analyze and display the results
    fn analyze(&self) {
        match (self.longest_word(), self.shortest_word()) {
            (Some(longest), Some(shortest)) => {
                println!("Longest word: {}", longest);
                println!("Shortest word: {}", shortest);
            }
            _ => println!("No valid words found."),
        }
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter a sentence to analyze:");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let analyzer = EssayAnalyzer::new(input.trim().to_string());
    analyzer.analyze();
}
