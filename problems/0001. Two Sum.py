"""
title : 0001. Two Sum
author : AK 賴韋銘
time : 2022/08/10
"""

class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        
        # Only one valid answer exists.
        
        # 方法一 我自己寫的  但執行速度太慢
        # n1 = 0
        # n2 = 0
        # while n1 < nums.__len__():
        #     n2=n1
        #     while n2 < nums.__len__():
        #         if nums[n1] + nums[n2] == target and n2!=n1:
        #             return [n1,n2]
        #         n2 += 1
        #     n1 += 1
        
        # 方法二 參考別人寫的 執行速度很快  
        # 關鍵差在只有一個解所以不用每個都算一次  找目標值減每個數的差後 有沒有對應到裡面的  有就可以回傳了

        d = {}                                 # 創建一個空集合
        for i, n in enumerate(nums):           # 用enumerate函數 會回傳 索引值(或第幾次迭代) 和 當前值
            m = target - n                  
            if m in d:
                return [d[m], i]
            else:
                d[n] = i
