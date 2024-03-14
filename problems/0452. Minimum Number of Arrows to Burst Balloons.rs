impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();               // 取得有幾顆氣球
        if n == 0 {                         
            return 0;                       // 如果沒有汽球 回傳0
        }
        points.sort_by_key(|p| p[1]);       // 將氣球按照結束位置進行排序
        let mut end = points[0][1];         // 初始化第一支箭可以達到的最遠距離為第一個氣球的結束位置
        let mut res = 1;                    // 初始化所需最少箭頭數為1，因為至少需要一支箭射爆第一個氣球
        for i in 1..n {                     // 遍歷剩下的每個氣球
            if points[i][0] <= end {        // 如果目前這顆汽球的起始沒有超過上一個汽球的結束 進到下一圈
                continue;
            }
            end = points[i][1];             // 起始超過了上一顆的結束了  把這顆的結束當成新的結束
            res += 1;                       // 發射次數+1
        }
        res 
    }
}
