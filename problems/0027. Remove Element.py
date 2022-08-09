class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        # 原地演算法 必須要用本來列表中的資料來進行置換 已減少額外的記憶體空間
        # 如果是空的列表 回傳0
        if not nums: return 0
        
        
        # 有多少元素  從 0 個開始
        i = 0
        for x in range(len(nums)):
            if nums[x] == val:
                pass
            elif nums[x] != val:
                nums[i] = nums[x]
                i+=1
        return i
