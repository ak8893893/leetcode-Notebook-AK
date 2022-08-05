class Solution:
    def isValid(self, s: str) -> bool:
        # # 參考答案
        # res = []
        # if s[0] == ")" or s[0] == "]" or s[0] == "}": return False
        # for i in range(s.__len__()):
        #     if s[i] == "(" or s[i] == "[" or s[i] == "{":
        #         res.append(s[i])
        #     elif res.__len__()>0:
        #         val = res.pop()                        # 重點在這邊 消消
        #         if (s[i] == ")" and val != "(" or
        #             s[i] == "]" and val != "[" or
        #             s[i] == "}" and val != "{" ):return False
        #     else: return False
        # return True if len(res) == 0 else False
    
        # 依照邏輯自己寫一次 核心邏輯為消消  最後要歸零 第一個不可以是右括號
        dataList = []
        if s[0] == ")" or s[0] == "]" or s[0] == "}": return False
        for i in range(len(s)):
            if s[i] == "(" or s[i] == "[" or s[i] == "{":
                dataList.append(s[i])
            elif dataList.__len__() > 0:
                dataOut = dataList.pop()
                if (s[i]==")" and dataOut !="(" or
                    s[i]=="]" and dataOut !="[" or
                    s[i]=="}" and dataOut !="{" ): return False
            else:return False
        if len(dataList) ==0: return True
        else: return False
