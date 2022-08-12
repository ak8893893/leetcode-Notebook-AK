"""
title : 0013. Roman to Integer
author : AK 賴韋銘
time : 2022/08/10
"""

class Solution:
    def romanToInt(self, s: str) -> int:
        Ans = 0
        Temp = ""
        for x in s:
            if Temp != "I" and Temp != "X" and Temp != "C":
                if x == "I":
                    Ans += 1
                    Temp = x
                elif x == "V":
                    Ans += 5
                    Temp = x
                elif x == "X":
                    Ans += 10
                    Temp = x
                elif x == "L":
                    Ans += 50
                    Temp = x
                elif x == "C":
                    Ans += 100
                    Temp = x
                elif x == "D":
                    Ans += 500
                    Temp = x
                elif x == "M":
                    Ans += 1000
                    Temp = x
            elif Temp == "I":
                if x == "I":
                    Ans += 1
                    Temp = x
                elif x == "V":
                    Ans += 3
                    Temp = x
                elif x == "X":
                    Ans += 8
                    Temp = x
                elif x == "L":
                    Ans += 50
                    Temp = x
                elif x == "C":
                    Ans += 100
                    Temp = x
                elif x == "D":
                    Ans += 500
                    Temp = x
                elif x == "M":
                    Ans += 1000
                    Temp = x
            elif Temp == "X":
                if x == "I":
                    Ans += 1
                    Temp = x
                elif x == "V":
                    Ans += 5
                    Temp = x
                elif x == "X":
                    Ans += 10
                    Temp = x
                elif x == "L":
                    Ans += 30
                    Temp = x
                elif x == "C":
                    Ans += 80
                    Temp = x
                elif x == "D":
                    Ans += 500
                    Temp = x
                elif x == "M":
                    Ans += 1000
                    Temp = x
            elif Temp == "C":
                if x == "I":
                    Ans += 1
                    Temp = x
                elif x == "V":
                    Ans += 5
                    Temp = x
                elif x == "X":
                    Ans += 10
                    Temp = x
                elif x == "L":
                    Ans += 50
                    Temp = x
                elif x == "C":
                    Ans += 100
                    Temp = x
                elif x == "D":
                    Ans += 300
                    Temp = x
                elif x == "M":
                    Ans += 800
                    Temp = x

        return Ans
