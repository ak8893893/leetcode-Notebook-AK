"""
Title : 0091. Decode Ways
Author : AK 賴韋銘
Time : 2022/10/2
refer :
https://www.youtube.com/watch?v=VCJsUYeuqaY
https://zh.wikipedia.org/wiki/%E6%96%90%E6%B3%A2%E9%82%A3%E5%A5%91%E6%95%B0
https://leetcode.com/problems/decode-ways/discuss/2645738/Python-Dp-solutoin-or-98-Fast-or-Space-Optimised
"""

class Solution:
    def numDecodings(self, s: str) -> int:

        # make a dp list with len(s)+1 of 0
        dp=[0 for _ in range(len(s)+1)]

        # first 0 is not valid so if s[0] > 0 let dp[0] and dp[1] = 1
        if int(s[0])>0:
            dp[0]=1
            dp[1]=1

        # i start from 2 to len(s)
        for i in range(2,len(s)+1):
            # if current is 0 then it alone can't be mapped
            if int(s[i-1])>0:
                dp[i]=dp[i-1]
            # combining previous 2
            x=s[i-2:i]
            if 10<=int(x)<=26: # if this is b/w bounds then it can be mapped
                dp[i]+=dp[i-2]

        return dp[len(s)]
print(Solution.numDecodings(self="",s="1202"))