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
//     // 定義一個公共函數，接受一個數字字符串，返回一個包含所有可能字母組合的向量。
//     pub fn letter_combinations(digits: String) -> Vec<String> {
//         // 如果輸入的字符串是空的，則直接返回一個空的向量。
//         if digits.is_empty() {
//             return Vec::new();
//         }

//         // 定義一個向量來存儲電話按鍵上的字母，索引2到9對應每個按鍵上的字母。
//         let phone_map: Vec<&str> = vec![
//             "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
//         ];

//         // 將輸入的數字字符串轉換成一個usize類型的向量，以便後續處理。
//         let digits: Vec<usize> = digits.chars()
//             .map(|d| d.to_digit(10).unwrap() as usize)
//             .collect();

//         // 初始化一個空的字符串向量來存儲最終的結果。
//         let mut ans = Vec::new();
//         // 調用backtrack函數開始進行回溯搜索。
//         Self::backtrack(&mut ans, &phone_map, &digits, 0, &mut String::new());
//         ans
//     }

//     // 定義一個回溯函數，用於找到所有可能的字母組合。
//     fn backtrack(ans: &mut Vec<String>, phone_map: &[&str], digits: &[usize], index: usize, current: &mut String) {
//         // 如果當前索引等於數字串的長度，說明已經處理完所有數字，將當前組合加入到結果中。
//         if index == digits.len() {
//             ans.push(current.clone());
//             return;
//         }

//         // 根據當前數字，獲取對應的所有可能字母。
//         let letters = phone_map[digits[index]].chars();
//         for letter in letters {
//             // 對於每一個可能的字母，將它加到當前組合中。
//             current.push(letter);
//             // 然後遞歸調用回溯函數，處理下一個數字。
//             Self::backtrack(ans, phone_map, digits, index + 1, current);
//             // 回溯，移除最後一個字母，以嘗試下一個可能的字母組合。
//             current.pop();
//         }
//     }
// }



