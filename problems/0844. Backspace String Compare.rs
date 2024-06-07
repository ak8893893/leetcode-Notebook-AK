impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        // 將字符串 s 和 t 轉換為字符向量
        let mut s_chars: Vec<char> = s.chars().collect();
        let mut t_chars: Vec<char> = t.chars().collect();
        
        // 處理字符串，移除 # 並返回新字符串的長度
        let k = Self::process_string(&mut s_chars);
        let p = Self::process_string(&mut t_chars);

        // 如果處理後的長度不一樣，直接返回 false
        if k != p {
            return false;
        }

        // 逐字符比較處理後的字符串
        for i in 0..k {
            if s_chars[i] != t_chars[i] {
                return false;
            }
        }

        // 如果所有字符都相同，返回 true
        true
    }

    // 處理字符串的方法
    fn process_string(chars: &mut Vec<char>) -> usize {
        // 克隆原始字符向量
        let chars_copy = chars.clone();
        let mut k = 0;

        // 逐字符處理
        for &c in chars_copy.iter() {
            if c != '#' {
                // 如果不是 #，將字符放到 chars[k] 位置，並增加 k
                chars[k] = c;
                k += 1;
            } else if k > 0 {
                // 如果是 # 並且 k > 0，表示退格，減少 k
                k -= 1;
            }
        }

        // 返回處理後的新字符串長度
        k
    }
}
