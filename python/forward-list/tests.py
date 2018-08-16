#!/usr/bin/env python
import unittest
from ForwardList import ForwardList

class TestForwardList(unittest.TestCase):

    def setUp(self):
        self.flist = ForwardList()

    def test_print(self):
        self.assertEqual(str(self.flist), "")
        self.flist.append(11)
        self.flist.append(31)
        self.assertEqual(str(self.flist), "11 31")
        self.flist.append(9)
        self.assertEqual(str(self.flist), "11 31 9")

    def test_size(self):
        self.flist.append(11)
        self.assertEqual(self.flist.get_size(), 1)
        self.flist.pop_back() 
        self.assertEqual(self.flist.get_size(), 0)
        self.flist.prepend(30)
        self.assertEqual(self.flist.get_size(), 1)
        self.flist.pop_front() 
        self.assertEqual(self.flist.get_size(), 0)


    def test_append(self):
        self.flist.append(34)
    
    def test_prepend(self):
        self.flist.append(34)
        self.flist.prepend(11)
        self.assertEqual(str(self.flist), "11 34")
        self.flist.prepend(7)
        self.assertEqual(str(self.flist), "7 11 34")

    def test_pop_back(self):
        self.flist.append(22)
        self.flist.prepend(10)
        self.flist.prepend(8)
        self.flist.pop_back()
        self.assertEqual(str(self.flist), "8 10")
        self.flist.pop_back()
        self.assertEqual(str(self.flist), "8")
        self.flist.pop_back()
        self.assertEqual(str(self.flist), "")
        
    def test_pop_front(self):
        self.flist.append(22)
        self.flist.prepend(10)
        self.flist.prepend(8)
        self.flist.pop_front()
        self.assertEqual(str(self.flist), "10 22")
        self.flist.pop_front()
        self.assertEqual(str(self.flist), "22")
        self.flist.pop_front()
        self.assertEqual(str(self.flist), "")

    def test_front_back(self):
        self.flist.append(1)
        self.assertEqual(self.flist.front(), 1)
        self.assertEqual(self.flist.back(), 1)
        self.flist.prepend(10)
        self.assertEqual(self.flist.front(), 10)
        self.assertEqual(self.flist.back(), 1)
        self.flist.append(125)
        self.assertEqual(self.flist.front(), 10)
        self.assertEqual(self.flist.back(), 125)
    
    def test_clear(self):
        for a in range(3):
            self.flist.append(a)
        self.flist.clear()
        self.assertEqual(str(self.flist), "")
        
    def test_reverse(self):
        self.flist.append(1)
        self.assertEqual(str(self.flist), "1")
        self.flist.append(2)
        self.flist.reverse()
        self.assertEqual(str(self.flist), "2 1")
        self.flist.clear()
        for a in range(3):
            self.flist.append(a)
        self.flist.reverse()
        self.assertEqual(str(self.flist), "2 1 0")

if __name__ == '__main__':
    unittest.main(verbosity=2)
