"""
Title : 0058. Length of Last Word
Author : AK 賴韋銘
Time : 2022/08/14
"""

class Solution:
    def lengthOfLastWord(self, s: str) -> int:
        # word length start form 0 
        length = 0
        
        # caculate the word length from first word of string, 
        # if there is a space between two word let length var become 0
        try:
            for x in range(len(s)):
                if s[x] != " ":
                    length += 1

                elif s[x] == " " and s[x+1] != " ": 
                    length = 0 
            return length
                    
        # if there is an error appearing, 
        # that means the index is out of the string so return the length of the last word
        except:
            return length
