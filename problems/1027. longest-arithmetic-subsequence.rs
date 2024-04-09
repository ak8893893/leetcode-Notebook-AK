// 版本1  294ms 5.8MB
// use std::collections::HashMap;

// impl Solution {
//     pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
//         // 使用 HashMap 的向量來存儲每個索引處每個差值的最長算術序列長度
//         let mut dp: Vec<HashMap<i32, i32>> = vec![HashMap::new(); nums.len()];
//         let mut max_length = 1; // 任何單個數字本身構成的算術序列的長度為 1

//         for i in 0..nums.len() {
//             for j in 0..i {
//                 let diff = nums[i] - nums[j]; // 計算兩數之間的差值
//                 // 先計算出這個位置的最長長度值
//                 let current_length = dp[j].get(&diff).unwrap_or(&1) + 1;
//                 // 使用 entry API 來更新 dp[i] 中的值，避免同時借用造成的錯誤
//                 dp[i].entry(diff).and_modify(|e| *e = std::cmp::max(*e, current_length)).or_insert(current_length);
//                 // 更新目前為止發現的最長長度
//                 max_length = std::cmp::max(max_length, current_length);
//             }
//         }

//         max_length
//     }
// }


// 版本2 15ms 6MB
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        // 設定一個範圍常數，用於處理數組中元素差值可能的範圍，並允許差值有負數
        const RANGE: i32 = 500;
        // 初始化一個二維向量map，用於儲存每個元素針對每個差值的最長算術序列長度
        // 這裡將差值範圍偏移，以便使用正數索引
        let mut map = vec![[1; (RANGE * 2) as usize + 1]; nums.len()];
        // 用於記錄到目前為止發現的最長算術序列長度
        let mut ret = i32::MIN;

        // 遍歷每個數字，從第二個元素開始（因為至少需要兩個元素來形成序列）
        for (i, &n) in nums.iter().enumerate().skip(1) {
            // 對於當前元素之前的每個元素進行反向遍歷，計算差值並更新map
            for (j, &m) in nums[..i].iter().enumerate().rev() {
                // 計算差值並加上RANGE偏移，轉化為usize作為map的索引
                let diff = (n - m + RANGE) as usize;
                // 如果這個差值的序列長度已經被更新過，則跳過以避免重複計算
                if map[i][diff] != 1 {
                    continue;
                }
                // 計算當前差值的最長算術序列長度，並更新map及最長長度ret
                let count = map[j][diff] + 1;
                map[i][diff] = count;
                ret = ret.max(count);
            }
        }

        ret
    }
}
