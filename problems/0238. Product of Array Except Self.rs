impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut clone_nums = nums.clone();

        if nums.contains(&0){                                   // 有零的情況下
            let mut ans:i32 = 1;                            
            let mut i:usize = 0;
            for x in nums.iter(){
                if x == &0{                                       
                    clone_nums[i] = 1;                          // 發現第一個零 把當前數字變成1
                    if clone_nums.contains(&0){                 // 如果還有第二個 0  回傳全部都是零的向量回去
                        return vec![0;nums.len()];
                    }else{                                      // 如果沒有第二個 0  計算答案放到 是零的那個位置
                        for x in clone_nums.iter(){
                            ans *= x;
                        }
                        let mut ans_vec = vec![0;nums.len()];
                        ans_vec[i] = ans;
                        return ans_vec;
                    }
                }
                i+=1;
            } 
        }

        // 沒有零的狀況下
        let mut l2r = vec![i32::MAX;nums.len()];                    // 存放左到右的乘積的向量
        let mut r2l = vec![i32::MAX;nums.len()];                    // 存放右到左的乘積的向量
        let mut av = vec![i32::MAX;nums.len()];                     // 放答案的向量
        let long:usize = nums.len() as usize;

        l2r[0] = clone_nums[0];                                     // 頭尾固定
        r2l[long-1] = clone_nums[nums.len()-1];                     // 頭尾固定

        let mut u:usize = 1;                                        // 從 index 開始算答案
        while u<long{
            l2r[u]= l2r[u-1]*clone_nums[u];                         // 左到右的乘積累計
            r2l[long - 1 - u] = r2l[long - u] * clone_nums[long - 1 - u];   // 右到左的乘積累計
            u+=1;
        }
        av[0]=r2l[1];                                               // 答案頭的答案固定
        av[long-1]=l2r[long-2];                                     // 答案尾的答案固定
        for x in 1..long-1{
            av[x]= l2r[x-1] * r2l[x+1];                             // 不含自己  左邊的左到右乘積*右邊的右到左乘積 就是此格答案
        }
        return av;
    }
}
