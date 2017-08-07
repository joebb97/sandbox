class Node:
    
    def __init__(self, datum=None, next=None):
        self.datum = datum
        self.next = next

    def __str__(self):
        return str(self.datum)

class ForwardList:

    def __init__(self, head=None):
        self.head = head

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
        else:
            marker = self.head
            while marker.next:
                marker = marker.next
            marker.next = Node(data)
    
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

    def pop_front(self):
        assert self.head
        next = self.head.next
        del self.head
        self.head = next

    def front():
        return self.head.datum
