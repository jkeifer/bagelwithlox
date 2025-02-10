use super::reader::Source;

pub type Tokens = ();

pub fn tokenize(source: Source) -> Tokens {
    println!("Tokenizing...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        tokenize(());
    }
}
