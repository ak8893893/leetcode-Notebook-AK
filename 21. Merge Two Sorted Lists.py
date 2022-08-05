# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def mergeTwoLists(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        """
        :type l1: ListNode
        :type l2: ListNode
        :rtype: ListNode
        """
        if not l1 or not l2: return l1 or l2    # 如果其中一邊為空值 直接回傳另外一個 linkedlist 回去
        head = cur = ListNode(val=0,next=None)  # 創造一個頭，值為0，指向的下一個節點為空值
        while l1 and l2:                        # 進行迴圈直到一邊 linkedList 變成空值為止
            if l1.val > l2.val:                 # 如果 l1 的當前值 > l2 的當前值
                cur.next = l2                   # 把 l2 當成下一個節點連接起來
                l2 = l2.next                    # 更新 l2 把用過的值捨去掉 進入下一個節點
            elif l1.val <= l2.val:              # 如果 l1 的當前值 <= l2 的當前值
                cur.next = l1                   # 把 l1 當成下一個節點連接起來
                l1 = l1.next                    # 更新 l1 把用過的值捨去掉 進入下一個節點
            cur = cur.next                      # 把現在看的節點往後看一個
        cur.next= l1 or l2                      # l1 or l2 其中一個變成空值後離開迴圈 把剩下的節點接在後面就完成了
        return head.next                        # 回傳頭後面的節點鍊
        
        
        
        
        
        
        
        
        
        
        
        # # https://blog.csdn.net/coder_orz/article/details/51529359
        # # 思路三 : 用遞歸的思想
        # if not l1 or not l2:
        #     return l1 or l2
        # if l1.val < l2.val:
        #     l1.next = self.mergeTwoLists(l1.next, l2)
        #     return l1
        # else:
        #     l2.next = self.mergeTwoLists(l1, l2.next)
        #     return l2
    
    
    
    
        # 思路一
        # 合併後的鍊錶仍然是有序的，可以同時遍歷兩個鍊錶，
        # 每次選取兩個鍊錶中較小值的節點，依次連接起來，就能得到最終的鍊錶。
        
        # if not l1 or not l2:
        #     return l1 or l2
        # head = cur = ListNode(0)
        # while l1 and l2:            # 一邊變成 none 就會結束迴圈
        #     if l1.val < l2.val:
        #         cur.next = l1
        #         l1 = l1.next
        #     else:
        #         cur.next = l2
        #         l2 = l2.next
        #     cur = cur.next
        # cur.next = l1 or l2         # 把另外一個還沒變成 None 的放進來
        # return head.next
