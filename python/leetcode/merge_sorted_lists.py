# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        if not list1:
            return list2
        
        if not list2:
            return list1
        
        it1 = list1
        it2 = list2
        out = None
        
        while it1 and it2:
            if it2.val <= it1.val:
                new_node = ListNode(it2.val, None)
                it2 = it2.next
            else:
                new_node = ListNode(it1.val)
                it1 = it1.next
                
            if not out:
                out = new_node
                out_it = out
            else:
                out_it.next = new_node
                out_it = out_it.next
                
        while it1:
            new_node = ListNode(it1.val, None)
            it1 = it1.next
            out_it.next = new_node
            out_it = out_it.next
            
        while it2:
            new_node = ListNode(it2.val, None)
            it2 = it2.next
            out_it.next = new_node
            out_it = out_it.next
                
        return out
            
