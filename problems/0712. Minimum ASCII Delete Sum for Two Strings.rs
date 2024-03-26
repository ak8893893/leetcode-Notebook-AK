impl Solution {
    // 定義函數 minimum_delete_sum，接受兩個字符串 s1 和 s2，返回一個 i32 整數作為答案
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        // 將字符串 s1 和 s2 轉換成字節數組以便進行比較
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();

        // 初始化動態規劃表 dp，用於存儲中間結果。dp[i][j] 表示 s1 的前 i 個字符和 s2 的前 j 個字符達到相等所需的最小 ASCII 刪除和
        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];

        // 通過雙層循環遍歷 s1 和 s2 的每個字符
        for i in 0..s1.len() {
            for j in 0..s2.len() {
                // 如果當前字符相等，則 dp[i + 1][j + 1] 等於左上角的值加上當前字符的 ASCII 值；
                // 否則，取左邊和上邊的最大值，表示刪除一個字符後的最小刪除和
                dp[i + 1][j + 1] = if s1[i] == s2[j] {
                    dp[i][j] + s1[i] as i32
                } else {
                    dp[i][j + 1].max(dp[i + 1][j])
                };
            }
        }
        // 最後計算答案，將 s1 和 s2 的 ASCII 總和減去兩倍的 dp[s1.len()][s2.len()]，
        // 因為 dp[s1.len()][s2.len()] 包含了重複計算的部分
        s1.iter().map(|&b| b as i32).sum::<i32>() +
        s2.iter().map(|&b| b as i32).sum::<i32>() - 
        dp[s1.len()][s2.len()] * 2
    }
}
