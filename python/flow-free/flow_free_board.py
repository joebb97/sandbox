""" Flow free board class representation """
from colorama import init
from termcolor import colored

class BoardEntry(object):

    def __init__(self, in_color=None, end_point=0):
        self.color = in_color
        self.end_point = end_point



class FlowFreeBoard(object):

    def __init__(self, size, num_colors):
        self.grid = [[BoardEntry()] * size] * size


if __name__ == '__main__':
    BOARD = FlowFreeBoard(10, 5)
