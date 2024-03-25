impl Solution {
    pub fn decode_string(s: String) -> String {
        // 初始化一個新的字符串 `t` 來存放暫時的結果，
        // 和一個字符串 `n` 來存放數字。
        let mut t = String::new();
        let mut n = String::new();

        // 使用一個向量 `v` 來存放數字和字符串的組合，
        // 這對於追蹤括號內的字符串和重複次數很有幫助。
        let mut v = Vec::new();

        // 遍歷輸入字符串的每個字符。
        for c in s.chars() {
            // 如果字符是數字，將其添加到 `n`。
            if c.is_numeric() {
                n.push(c);
            } else if c.is_alphabetic() { // 如果字符是字母，將其添加到 `t`。
                t.push(c);
            } else if c == '[' { // 遇到 '[' 時，解析 `n` 為數字並將數字和 `t` 保存到 `v`。
                let m = n.parse::<i32>().unwrap();

                v.push((m, t));

                // 重置 `t` 和 `n` 為新的循環。
                t = String::new();
                n.clear();
            } else { // 遇到 ']' 時，從 `v` 中取出最後一個元素，並根據次數重複字符串。
                let mut p = v.pop().unwrap();
                let mut r = String::new();

                // 根據之前保存的數字，重複 `t` 字符串。
                while p.0 > 0 {
                    r.push_str(&t);

                    p.0 -= 1;
                }

                // 將重複後的字符串 `r` 添加到 `p.1`（之前保存的字符串）。
                p.1.push_str(&r);

                // 更新 `t` 為當前組合的字符串。
                t = p.1;
            }
        }
        
        // 返回最終組合的字符串。
        t
    }
}
