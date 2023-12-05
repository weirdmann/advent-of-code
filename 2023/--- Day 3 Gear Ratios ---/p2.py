from p1 import *


class Part:
    def __init__(self, symbol):
        self.symbol = symbol
        self.numbers = []


puzzle = ""
with open("input") as file:
    puzzle = file.read()
lines = puzzle.splitlines()


def pix(lines, px, py):
    xmin = 0
    xmax = len(lines[0]) - 1
    ymin = 0
    ymax = len(lines)

    if px < xmin or py < ymin:
        return None
    if px > xmax or py > ymax:
        return None

    return lines[py][px]
