pub type Source = ();

pub fn read_source(filename : &str) -> Source {
    println!("Reading source '{}'...", &filename);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_source() {
        read_source("file.bwl");
    }
}
