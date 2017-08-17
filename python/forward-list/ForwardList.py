class Node:
    
    def __init__(self, datum=None, next=None):
        self.datum = datum
        self.next = next

    def __str__(self):
        return str(self.datum)

class ForwardList:

    def __init__(self):
        self.__head = None
        self.__tail = None
        self.__size = 0

    def __str__(self):
        ret = ""
        marker = self.__head
        while marker:
            ret += str(marker.datum) + " "
            marker = marker.next
        ret = ret[:-1]
        return ret

    def get_size(self):
        return self.__size

    def append(self, data):
        if not self.__head:
            self.__head = Node(data)
            self.__tail = self.__head
        else:
            self.__tail.next = Node(data)
            self.__tail = self.__tail.next
        self.__size += 1

    def prepend(self, data):
        new_head = Node(data, self.__head)
        self.__head = new_head
        self.__size += 1

    def pop_back(self):
        assert self.__head
        marker = self.__head
        next = marker.next
        if not next:
            self.__head = None
            self.__tail = None
            del marker
        else:
            while next.next:
                marker = next
                next = marker.next
            marker.next = None
            del next
            self.__tail = marker
        self.__size -= 1

    def pop_front(self):
        assert self.__head
        next = self.__head.next
        del self.__head
        self.__head = next
        self.__size -= 1

    def front(self):
        return self.__head.datum

    def back(self):
        return self.__tail.datum

    def reverse(self):
        if self.__size <= 1:
            return
        self.__tail = self.__head
        front = None
        middle = self.__head
        back = middle.next
        while middle:
            middle.next = front
            front = middle
            middle = back
            if back:
                back = middle.next
        self.__head = front

    
    def clear(self):
        while self.__head:
            next = self.__head.next
            del self.__head
            self.__head = next

        self.__head = None
        self.__tail = None
        self.__size = 0

