pub fn something() -> String {
    String::from("This is just a test binary")
}

#[cfg(test)]
mod tests {
    use super::something;

    #[test]
    fn it_works() {
        assert_eq!(something(), "This is just a test binary");
    }
}
