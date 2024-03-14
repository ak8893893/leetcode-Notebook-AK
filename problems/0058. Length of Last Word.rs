impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut count:i32 = 0;
        let mut ans:i32 = 0;

        for x in s.chars(){
            if x == ' '{
                if count != 0 {
                    ans = count.clone();  
                    count = 0;
                }
            }else{
                count+=1;
                ans = count;
            }
        }
        ans
    }
}
