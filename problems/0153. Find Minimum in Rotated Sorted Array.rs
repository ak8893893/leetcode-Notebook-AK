// solution O(n)
// impl Solution {
//     pub fn find_min(nums: Vec<i32>) -> i32 {
//         let mut pre_num:i32 = nums[0];
//         for x in nums.iter(){
//             if &pre_num > x{
//                 return *x as i32;
//             }else{
//                 pre_num = *x as i32;
//             }
//         }
//         nums[0]
//     }
// }

// solution O(log n)
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 1{
            return nums[0];
        }

        let mut start = 0;
        let mut mid = (nums.len()/2) -1;
        let mut end = nums.len()-1;

        loop{
            if nums[mid] > nums[end]{
                start = mid +1;
                mid = ((start + end)/2);
                end = end;
                if start == mid {
                    if nums[end]>nums[start]{
                        return nums[start];
                    }else{
                        return nums[end];
                    }
                }
            }
            if nums[mid] < nums[end]{
                start = start;
                end = mid;
                mid = ((start + end)/2);
                if mid == end{
                    if nums[end]>nums[start]{
                        return nums[start];
                    }else{
                        return nums[end];
                    }
                }
            }
        }
    }
}
