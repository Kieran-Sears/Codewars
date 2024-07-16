use crate::bank_account_strings::bank_account_strings::_parse_bank_account;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(123456789, _parse_bank_account(concat!(
            "    _  _     _  _  _  _  _ \n",
            "  | _| _||_||_ |_   ||_||_|\n",
            "  ||_  _|  | _||_|  ||_| _|\n"
        )));
        assert_eq!(23056789, _parse_bank_account(concat!(
            " _  _  _  _  _  _  _  _  _ \n",
            "| | _| _|| ||_ |_   ||_||_|\n",
            "|_||_  _||_| _||_|  ||_| _|\n"
        )));
        assert_eq!(823856989, _parse_bank_account(concat!(
            " _  _  _  _  _  _  _  _  _ \n",
            "|_| _| _||_||_ |_ |_||_||_|\n",
            "|_||_  _||_| _||_| _||_| _|\n"
        )));
    }
}