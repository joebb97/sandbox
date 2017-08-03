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


    def test_append(self):
        self.flist.append(34)

if __name__ == '__main__':
    unittest.main(verbosity=2)
