"""
Title : 0002. Add Two Numbers
Author : AK 賴韋銘
Time : 2022/08/13
"""

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        # 先取得 l1 和 l2 的數字
        tot1 = ""
        tot2 = ""
        while True:
            tot1 += str(l1.val)
            if l1.next == None: break
            l1 = l1.next
        
        while True:
            tot2 += str(l2.val)
            if l2.next == None: break
            l2 = l2.next        
        
        # 題目有說要反過來成為數字  所以多這一段讓字串反過來
        tot1 = tot1[::-1]
        tot2 = tot2[::-1]
        
        
        # l1+l2後 轉成字串 反轉一次 
        total = int(tot1)+int(tot2)
        total = str(total)[::-1]

        # 把算好的答案轉成 linked list 
        ans = temp = ListNode()
        for x in range(len(total)):
            temp.next = ListNode(total[x])
            if x == len(total)-1: break
            temp = temp.next

        # 回傳linked list
        return ans.next
