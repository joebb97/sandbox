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
        

if __name__ == '__main__':
    unittest.main(verbosity=2)
