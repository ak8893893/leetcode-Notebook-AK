"""
Title : 0067. Add Binary
Author : AK 賴韋銘
Time : 2022/08/15
"""

class Solution:
    def addBinary(self, a: str, b: str) -> str:
        # let two binary number transfer to decimal and add them toghether and use f"{:b}" transfer to binary
        return f"{int(a,2)+int(b,2):b}"
        
