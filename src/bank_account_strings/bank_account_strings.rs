// https://www.codewars.com/kata/597eeb0136f4ae84f9000001/train/rust

const RAW_DIGITS: &str = concat! {
    " _     _  _     _  _  _  _  _ \n",
    "| |  | _| _||_||_ |_   ||_||_|\n",
    "|_|  ||_  _|  | _||_|  ||_| _|\n"
};

fn structure_digits_10(raw: &str) -> [[[char; 3]; 3]; 10] {
    let char_indices: String = raw
        .chars()
        .filter(|&c| c != '\n')
        .collect();

    char_indices.chars().enumerate().fold([[['.'; 3]; 3]; 10], |mut acc, (index, char)| {
        let line_length = 30;
        let char_index_in_grid = index % line_length;
        let row = index / line_length;
        let col = (char_index_in_grid % line_length) % 3;
        let pos = ((char_index_in_grid % line_length) / 3) % 10;
        acc[pos][row][col] = char.clone();

        acc
    })
}

fn structure_digits_9(raw: &str) -> [[[char; 3]; 3]; 9] {
    let char_indices: String = raw
        .chars()
        .filter(|&c| c != '\n')
        .collect();

    char_indices.chars().enumerate().fold([[['.'; 3]; 3]; 9], |mut acc, (index, char)| {
        let line_length = 27;
        let char_index_in_grid = index % line_length;
        let row = index / line_length;
        let col = (char_index_in_grid % line_length) % 3;
        let pos = ((char_index_in_grid % line_length) / 3) % 10;
        // println!("[row: {}, col: {}, pos: {}, index: {}]", row, col, pos, index);
        acc[pos][row][col] = char.clone();

        acc
    })
}

fn decypher_digit(array3d: &[[[char; 3]; 3]; 10], array2d: &[[char; 3]; 3]) -> Option<usize> {
    match array3d.iter().position(|x| x == array2d) {
        Some(digit) => Some(digit),
        None => {
            for row in array2d.iter() {
                for &element in row.iter() {
                    print!("{} ", element);
                }
                println!();
            }
            None
        }
    }
}

pub fn parse_bank_account(bank_account: &str) -> u64 {
    let cypher = structure_digits_10(RAW_DIGITS);
    let account_digits = structure_digits_9(bank_account);
    account_digits
        .iter()
        .map(|digit| decypher_digit(&cypher, digit).unwrap().to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

#[cfg(test)]
mod bank_account_strings_tests {
    use super::*;

    #[test]
    fn test_parse_bank_account() {
        assert_eq!(
            123456789,
            parse_bank_account(concat!(
                "    _  _     _  _  _  _  _ \n",
                "  | _| _||_||_ |_   ||_||_|\n",
                "  ||_  _|  | _||_|  ||_| _|\n"
            ))
        );
        assert_eq!(
            23056789,
            parse_bank_account(concat!(
                " _  _  _  _  _  _  _  _  _ \n",
                "| | _| _|| ||_ |_   ||_||_|\n",
                "|_||_  _||_| _||_|  ||_| _|\n"
            ))
        );
        assert_eq!(
            823856989,
            parse_bank_account(concat!(
                " _  _  _  _  _  _  _  _  _ \n",
                "|_| _| _||_||_ |_ |_||_||_|\n",
                "|_||_  _||_| _||_| _||_| _|\n"
            ))
        );
    }
}
