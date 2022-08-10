"""
標題 : 0035. Search Insert Position
作者 : 賴韋銘 AK
時間 : 2022/08/10
"""


class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        
        # 先從第 0 筆資料開始比對
        try:
            i = 0
            if target < nums[0]: return 0               # 如果比第 0 筆資料小 就是擺第 0 筆 回傳 0
            for x in range(len(nums)):                  # 從第 0 筆資料開始比對
                if target == nums[x]: return x          # 如果跟某一筆資料相等 就放到那個位置 回傳當筆資料索引值
                elif target > nums [x] and target < nums[x+1]: return x+1   # 如果比當前資料大 但又比下一筆小 放當前資料索引值+1的位置  回傳當前索引值+1
        
        except:
            return len(nums)                            # 跑出資料長度了  表示是最後一筆  回傳整個資料長度作為索引給target使用
