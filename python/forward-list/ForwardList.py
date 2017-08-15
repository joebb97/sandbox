class Node:
    
    def __init__(self, datum=None, next=None):
        self.datum = datum
        self.next = next

    def __str__(self):
        return str(self.datum)

class ForwardList:

    def __init__(self, head=None, tail=None):
        self.head = head
        self.tail = tail

    def __str__(self):
        ret = ""
        marker = self.head
        while marker:
            ret += str(marker.datum) + " "
            marker = marker.next
        ret = ret[:-1]
        return ret

    def append(self, data):
        if not self.head:
            self.head = Node(data)
            self.tail = self.head
        else:
            self.tail.next = Node(data)
            self.tail = self.tail.next
    
    def prepend(self, data):
        new_head = Node(data, self.head)
        self.head = new_head

    def pop_back(self):
        assert self.head
        marker = self.head
        next = marker.next
        if not next:
            self.head = None
            del marker
        else:
            while next.next:
                marker = next
                next = marker.next
            marker.next = None
            del next
            self.tail = marker

    def pop_front(self):
        assert self.head
        next = self.head.next
        del self.head
        self.head = next

    def front(self):
        return self.head.datum

    def back(self):
        return self.tail.datum
