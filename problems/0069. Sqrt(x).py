"""
Title : 0069. Unique Morse Code Words
Author : AK 賴韋銘
Time : 2022/08/30
"""

class Solution:
    def mySqrt(self, x: int) -> int:

        # solution by AK
        # from 0 to x test every square bigger or less than x and find the answer
        # for test in range(x+1):
        #     A = test * test
        #     if A < x:
        #         B = test
        #     elif A == x:
        #         return test
        #     elif A > x:
        #         return B

        # solution by aerrojuakhil https://leetcode.com/problems/sqrtx/discuss/2495637/35-ms-faster-than-96.32-of-Python3-online-submissions-for-Sqrt(x)
        # start from (1+x)/2 make the number become mid
        # if the mid**2 bigger than x make the mid-1 and do it again and let top = mid-1   
        # if the mid**2 less than x make the mid-1 and do it again and let base = mid-1 
        # while base > top return top
        l, r = 1, x
        while l<=r:
            mid=(l+r)//2
            if (mid*mid)>x:
                r=mid-1
            elif (mid*mid)<x:
                l=mid+1
            else:
                return mid
        return r
