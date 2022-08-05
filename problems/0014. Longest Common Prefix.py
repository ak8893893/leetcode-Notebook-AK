class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        # my Answer
        Temp = ""
        Temp2 = ""

        if len(strs)>1:
            for x in range(1,strs.__len__()):
                if x == 1:
                    for y,z in zip(strs[x-1],strs[x]):
                        if y == z:
                            Temp += y
                        elif y != z:
                            break
                else:
                    for y,z in zip(Temp,strs[x]):
                        if y == z:
                            Temp2 += y
                        elif y != z:
                            break
                    Temp = Temp2
                    Temp2 = ""
        else:
            Temp = strs[0]
        return Temp
                
        
        
        # another solution with set
        # out = ""
        # for i in range(201):
        #     cs = set()
        #     for s in strs:
        #         if i >= len(s): return out
        #         cs.add(s[i])
        #         if len(cs) > 1: return out
        #     out += list(cs)[0]
        # return out
