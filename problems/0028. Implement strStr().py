"""
title : 0028. Implement strStr()
author : AK 賴韋銘
time : 2022/08/10
"""

class Solution:
    def strStr(self, haystack: str, needle: str) -> int:
        
        # 如果 needle 是空字串 回傳 0
        if needle == "": return 0

        # 索引值從 0 開始 使用雙迴圈比對 全部都對就回傳當前的索引值  如果有不 match 的部分就結束迴圈進下一個loop
        try:
            i = 0
            for x in haystack:
                j = i
                for y in range(len(needle)):

                    if needle[y] == haystack[j]:
                        j += 1
                        if y == len(needle)-1: return i
                    else:
                        break
                i+=1
            return -1
        
        # 如果跑比對跑到超過索引值表示比對失敗  回傳 -1
        except:
            return -1
