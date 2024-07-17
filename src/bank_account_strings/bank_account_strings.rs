// https://www.codewars.com/kata/597eeb0136f4ae84f9000001/train/rust

const RAW_DIGITS: &str = concat! {
    " _     _  _     _  _  _  _  _ \n",
    "| |  | _| _||_||_ |_   ||_||_|\n",
    "|_|  ||_  _|  | _||_|  ||_| _|\n"
};

fn structure_digits(raw: &str) -> Vec<[[char; 3]; 3]> {
    let char_indices: String = raw.chars().filter(|&c| c != '\n').collect();
    let line_length = char_indices.len() / 3;
    let num_digits = line_length / 3;

    char_indices.chars().enumerate().fold(
        vec![[['.'; 3]; 3]; num_digits],
        |mut acc, (index, char)| {
            let char_index_in_grid = index % line_length;
            let row = index / line_length;
            let col = (char_index_in_grid % line_length) % 3;
            let pos = ((char_index_in_grid % line_length) / 3) % num_digits;

            acc[pos][row][col] = char;

            acc
        },
    )
}

fn decypher_digit(array3d: &Vec<[[char; 3]; 3]>, array2d: &[[char; 3]; 3]) -> String {
    match array3d.iter().position(|x| x == array2d) {
        Some(digit) => digit.to_string(),
        None => {
            for row in array2d.iter() {
                for &element in row.iter() {
                    print!("{} ", element);
                }
                println!();
            }
            "".to_string()
        }
    }
}

pub fn parse_bank_account(bank_account: &str) -> u64 {
    let cypher = structure_digits(RAW_DIGITS);
    let account_digits = structure_digits(bank_account);

    account_digits
        .iter()
        .map(|digit| decypher_digit(&cypher, digit))
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
