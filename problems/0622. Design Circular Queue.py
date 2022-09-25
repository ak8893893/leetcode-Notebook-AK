"""
Title : 0622. Design Circular Queue
Author : AK
Time : 2022/09/25
refer：https://blog.csdn.net/fuxuemingzhu/article/details/81027583
"""

class MyCircularQueue:    
    def __init__(self, k: int):
        self.size = k
        self.queue = []
        self.front = 0
        self.rear = 0

    def enQueue(self, value: int) -> bool:
        if self.rear - self.front < self.size:
            self.queue.append(value)
            self.rear += 1
            return True
        
        else : return False

    def deQueue(self) -> bool:
        if self.rear - self.front > 0:
            self.front += 1
            return True
        
        else : return False

    def Front(self) -> int:
        if self.isEmpty() : return -1
        else : return self.queue[self.front]

    def Rear(self) -> int:
        if self.isEmpty() : return -1
        
        else : return self.queue[self.rear-1]

    def isEmpty(self) -> bool:
        return self.front == self.rear

    def isFull(self) -> bool:
        return self.rear - self.front == self.size


# Your MyCircularQueue object will be instantiated and called as such:
# obj = MyCircularQueue(k)
# param_1 = obj.enQueue(value)
# param_2 = obj.deQueue()
# param_3 = obj.Front()
# param_4 = obj.Rear()
# param_5 = obj.isEmpty()
# param_6 = obj.isFull()



