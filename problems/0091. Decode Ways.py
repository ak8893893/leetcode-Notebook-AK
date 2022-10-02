"""
Title : 0804. Unique Morse Code Words
Author : AK 賴韋銘
Time : 2022/08/17
"""

class Solution:
    def uniqueMorseRepresentations(self, words: List[str]) -> int:

        # 建立英文字列表
        alphabet = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"]
        
        # 建立摩斯密碼表
        wordMorse = [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."]
        
        # 建立空的集合 存放取得的摩斯密碼型態
        finalSet = set()
        
        # 把輸入列表中的每個字逐一比對轉換成摩斯密碼後放入集合
        for word in words:
            transformation = ""
            for w in word:
                i = 0
                for z in alphabet:
                    if z == w: break
                    i += 1
                transformation += wordMorse[i]
            finalSet.add(transformation)
        
        # 回傳集合長度(代表取得多少種不同的形態)
        return len(finalSet)
