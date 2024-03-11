/*
slice.to_vec()：這部分將字節切片轉換為一個向量（Vec<u8>）。to_vec() 方法創建了一個新的向量，其中包含了切片中的所有元素。
String::from_utf8(...)：這是一個 Rust 函數，它將字節切片轉換為 UTF-8 字串。具體來說：
如果切片中的字節是有效的 UTF-8 字節序列，則 from_utf8() 方法會返回一個 Result<&str, Utf8Error>。這意味著它要麼返回一個有效的字符串切片（&str），要麼返回一個錯誤（Utf8Error）。
如果切片中的字節不是有效的 UTF-8，則 from_utf8() 會返回一個錯誤，指示為什麼該切片不是有效的 UTF-8。
.unwrap_or("".to_string())：這部分是處理錯誤的一種方式。如果 from_utf8() 成功將切片轉換為字符串，則 .unwrap_or(...) 會返回該字符串。如果轉換失敗（例如，切片不是有效的 UTF-8），則它會返回一個空字符串（""）。
目的是將字節切片轉換為字符串。如果切片是有效的 UTF-8，則返回該字符串；否則返回一個空字符串。
*/
impl Solution {
        pub fn longest_palindrome(s: String) -> String {
        let mut window_size = s.len();              // 初始化窗口大小從字串長度開始
        while window_size > 0 {          
             match s.as_bytes()                     // 將字符串轉換為字節切片 
                    .windows(window_size)           // 創造長度為目前窗口大小的滑動窗口  滑動窗口會創造長度為窗口大小的 slice 的數個切片每個起始都向前一個差一
                    .find(|slice| { let iter = slice.iter();   // 把切片變成迭代器
                                        iter.clone().eq(iter.clone().rev()) // 把迭代的切片和反過來的自己做比較
                                }
                        ) {
                             Some(slice) => return String::from_utf8(slice.to_vec()).unwrap_or("".to_string()),   // 如果找到了這個切片  回傳這個切片的字串型態  
                             None => window_size -= 1, // 如果這個大小的窗口長度沒有答案   把窗口大小減小一點再試一次
                        }
        }
        "".to_string() // 如果窗口長度為0表示沒字串  回傳空字串
    }
}
