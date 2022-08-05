class Solution:
    def isPalindrome(self, x: int) -> bool:
        stringX = str(x)
        listX = []
        for s in stringX:
            listX.append(s)
        listY = listX.copy()

        listX.reverse()
        if listY == listX:
            # print("true")
            return True
        else:
            # print("false")
            return False
