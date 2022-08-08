class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        
        # # 如果是只有1個長度的列表
        if len(nums) == 1: return 1

        # 從索引值0 開始比對 有偵測當前索引值和後面一個值不同時 把它放到索引值[i]的位置 再把i+1  全部跑完之後回傳i值
        i = 1
        for j in range(nums.__len__()-1):
            if nums[j] != nums[j+1]:
                nums[i] = nums[j+1]
                i+=1
        return i
    
    
        # 參考的答案
        # i=1
        # for j in range(1,len(nums)):
        #     if nums[j] != nums[j-1]:
        #         nums[i] = nums[j]
        #         i+=1
        # return i
