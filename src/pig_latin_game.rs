use self::pig_latin_game::to_pig_latin;

mod pig_latin_game {
    pub fn pig_latin(word: &str) -> String {
        let vowels = "aiueo";
        if word.len() == 0 {
            return String::new();
        }
        let first_char = word.chars().next().unwrap();
        if vowels.contains(first_char) {
            return format!("{}-hay", word);
        } else {
            let mut chars = word.chars();
            while let Some(c) = chars.next() {
                if vowels.contains(c) {
                    return format!("{}-{}ay",chars.as_str(), &word[..word.len() - chars.as_str().len() - 1]);
                }
            }
            format!("{}-ay", word)
        }

        
    }

    pub fn to_pig_latin(sentence: &str) -> String {
        sentence.split_whitespace()
            .map(pig_latin)
            .collect::<Vec<String>>()
            .join(" ")
    }
}

pub fn pig_latin_exec() {
    let input = "first apple";
    let conv = to_pig_latin(input);
    println!("{}", conv);
}