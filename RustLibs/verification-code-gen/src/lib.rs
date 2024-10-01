pub mod verification_code_generator;

#[cfg(test)]
mod tests {
    use crate::verification_code_generator::VerificationCodeGenerator;

    #[test]
    fn test_code_gen() {
        let new_code = VerificationCodeGenerator::generate();
        // Ensure that it has the correct number of digits
        assert_eq!(new_code.len(), 6);
    }
}
