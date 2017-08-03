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

