"""
Title : 0066. Plus One
Author : AK 賴韋銘
Time : 2022/08/15
"""

class Solution:
    def plusOne(self, digits: List[int]) -> List[int]:

        # let digits transfer to string
        num = ""                 
        for x in digits:
            num += str(x)
        
        # transfer digits to int and plus 1
        num = int(num)+1
        
        # transfer to string again
        num = str(num)
        
        # append the digits to list
        ans = []
        for x in num:
            ans.append(x)
        
        # return the answer
        return ans
