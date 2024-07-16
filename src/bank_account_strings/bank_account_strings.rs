// https://www.codewars.com/kata/597eeb0136f4ae84f9000001/train/rust

pub fn _parse_bank_account(bank_account: &str) -> u64 {
    // I want an array of [[3][3] [9]
    // I want to match on the items in the second array and decypher numbers based on symbols
    let mut a: [[[char; 3]; 3]; 9] = [[['.'; 3]; 3]; 9]; 
    bank_account.chars().enumerate().for_each(|(index, char)| {
        let row = index / 27;
        let mut col = index;
        if row == 1 {
            col = index - 27;
        }
        if row == 2 {
            col = index - 54;
        }
        let num = col / 9;
        a[row][col][num] = char;
    });

    todo!();
}

