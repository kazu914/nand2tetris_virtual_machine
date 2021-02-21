pub struct ReturnAddressGenerator {
    counter: usize,
}

impl ReturnAddressGenerator {
    pub fn new() -> ReturnAddressGenerator {
        ReturnAddressGenerator { counter: 0 }
    }

    pub fn generate_new_return_address(&mut self) -> String {
        self.counter += 1;
        format!("Return_address.{}", self.counter)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_new_return_address() {
        let mut return_address_generator = ReturnAddressGenerator::new();
        let return_address: String = return_address_generator.generate_new_return_address();
        assert_eq!("Return_address.1", return_address)
    }

    #[test]
    fn increment_counter() {
        let mut return_address_generator = ReturnAddressGenerator::new();
        let _ = return_address_generator.generate_new_return_address();
        assert_eq!(1, return_address_generator.counter);

        let _ = return_address_generator.generate_new_return_address();
        assert_eq!(2, return_address_generator.counter)
    }
}
