#!/usr/bin/env python
# Input: n = 5
# Output:   1   2   3   4   5
#           16  17  18  19  6
#           15  24  25  20  7
#           14  23  22  21  8
#           13  12  11  10  9

# Input: n = 6
# Output:
#           1  2  3  4  5  6
#           20 21 22 23 24 7
#           19 32 33 34 25 8
#           18 31 36 35 26 9
#           17 30 29 28 27 10
#           16 15 14 13 12 11

# Input: n = 2
# Output: 1 2
#         4 3


def spiral1(n):
    matrix = [[None] * n for _ in range(n)]
    dx, dy = 1, 0
    x, y = 0, 0
    for i in range(1, n**2 + 1):
        matrix[x][y] = i
        nx, ny = x + dx, y + dy
        if 0 <= nx < n and 0 <= ny < n and matrix[nx][ny] == None:
            x, y = nx, ny
        else:
            dx, dy = -dy, dx
            x, y = dx + x, dy + y

    return matrix

# Doesn't work
# def spiral2(n):
#     matrix = [[0] * n for _ in range(n)]
#     row, col = 0, 0
#     # hdir = 1 means going right, hdir = -1 means going left
#     # vdir = 1 means going down, vdir = -1 means going up
#     # 0 for either means not going anywhere
#     vdir, hdir = 0, 1
#     top, bot, left, right = 0, n - 1, 0, n - 1
#     rotations = 0

#     for i in range(1, n**2 + 1):
#         matrix[row][col] = i
#         new_row, new_col = row + vdir, col + hdir
#         if top <= new_row <= bot and left <= new_col <= right:
#             row, col = new_row, new_col
#         else:
#             # need to rotate, we're at the boundary
#             hdir, vdir = -vdir, hdir
#             row, col = row + vdir, col + hdir

#     return matrix


def printspiral(matrix):
    n = range(len(matrix))
    print "n = {}:\n\n".format(len(matrix))
    for y in n:
        for x in n:
            print "%2i" % matrix[x][y],
        print


printspiral(spiral1(5))
print
printspiral(spiral1(2))
print
printspiral(spiral1(6))
