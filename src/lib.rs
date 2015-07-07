
/// Returns a quote from the Matrix movie
///
/// ```
/// println!("{}", matrix::quote());
/// ```
pub fn quote() -> &'static str {
    "There is no spoon."
}

#[cfg(test)]
mod test {
    use std::{env, mem};

    #[test]
    pub fn test_something() {
        let arch = env::var("ARCH").unwrap();

        let expect = match &arch[..] {
            "x86_64" => 8,
            "i686" => 4,
            _ => panic!("unexpected $ARCH: {}", arch),
        };

        assert_eq!(mem::size_of::<usize>(), expect);
    }
}
