impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        // 沒有按按鈕的狀況
        if digits.len() == 0{
            return Vec::<std::string::String>::from([]);
        }

        let mut letter_vec:Vec<Vec<&str>> = Vec::new();
        let mut ans:Vec<String> = Vec::new();

        // 按鈕列表
        let two = vec!["a", "b", "c"];
        let three = vec!["d", "e", "f"];
        let four = vec!["g", "h", "i"];
        let five = vec!["j", "k", "l"];
        let six = vec!["m", "n", "o"];
        let seven = vec!["p", "q", "r", "s"];
        let eight = vec!["t", "u", "v"];
        let nine = vec!["w", "x", "y", "z"];

        for x in digits.chars() {
            match x {
                '2' => letter_vec.push(two.clone()),
                '3' => letter_vec.push(three.clone()),
                '4' => letter_vec.push(four.clone()),
                '5' => letter_vec.push(five.clone()),
                '6' => letter_vec.push(six.clone()),
                '7' => letter_vec.push(seven.clone()),
                '8' => letter_vec.push(eight.clone()),
                '9' => letter_vec.push(nine.clone()),
                _ => {}
            }
        }

        // 依照各種狀況組字串出來
        match letter_vec.len() {
            1 => {
                for x in &letter_vec[0] {
                    ans.push(x.to_string());
                }
            }
            2 => {
                for x in &letter_vec[0] {
                    for y in &letter_vec[1] {
                        ans.push(format!("{}{}", x, y));
                    }
                }
            }
            3 => {
                for x in &letter_vec[0] {
                    for y in &letter_vec[1] {
                        for z in &letter_vec[2] {
                            ans.push(format!("{}{}{}", x, y, z));
                        }
                    }
                }
            }
            4 => {
                for x in &letter_vec[0] {
                    for y in &letter_vec[1] {
                        for z in &letter_vec[2] {
                            for a in &letter_vec[3] {
                                ans.push(format!("{}{}{}{}", x, y, z, a));
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        ans
    }
}

// 優化後的寫法
// impl Solution {
//     pub fn letter_combinations(digits: String) -> Vec<String> {
//         if digits.is_empty() {
//             return Vec::new();
//         }

//         let phone_map: Vec<&str> = vec![
//             "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
//         ];

//         let digits: Vec<usize> = digits.chars()
//             .map(|d| d.to_digit(10).unwrap() as usize)
//             .collect();

//         let mut ans = Vec::new();
//         Self::backtrack(&mut ans, &phone_map, &digits, 0, &mut String::new());
//         ans
//     }

//     fn backtrack(ans: &mut Vec<String>, phone_map: &[&str], digits: &[usize], index: usize, current: &mut String) {
//         if index == digits.len() {
//             ans.push(current.clone());
//             return;
//         }

//         let letters = phone_map[digits[index]].chars();
//         for letter in letters {
//             current.push(letter);
//             Self::backtrack(ans, phone_map, digits, index + 1, current);
//             current.pop();
//         }
//     }
// }


