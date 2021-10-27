pub struct Tokenizer {
    pub pos: usize,
    pub input: String,
}

impl Tokenizer {
    pub fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    pub fn consume_while<F>(&mut self, f: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();

        while !self.eof() && f(self.next_char()) {
            result.push(self.consume_char());
        }

        return result;
    }

    pub fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));

        self.pos += next_pos;
        return cur_char;
    }

    pub fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    pub fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    pub fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }
}
