from p1 import *
import json


class Part:
    def __init__(self, x, y, symbol):
        self.symbol = symbol
        self.numbers = []
        self.x = x
        self.y = y

    def sum(self):
        s = 0
        for n in self.numbers:
            s += int(n)
        return s

    def isValid(self):
        return len(self.numbers) != 0

    def isGear(self):
        return self.symbol == "*" and len(self.numbers) == 2

    def gearRatio(self):
        mult = 1
        for n in self.numbers:
            mult *= int(n)
        return mult


puzzle = ""
with open("input") as file:
    puzzle = file.read()
lines = puzzle.splitlines()


def pix(lines, px, py):
    xmin = 0
    xmax = len(lines[0]) - 1
    ymin = 0
    ymax = len(lines) - 1

    if px < xmin or py < ymin:
        return None
    if px > xmax or py > ymax:
        return None

    return lines[py][px]


def posHash(x, y):
    return f"(x:{x},y:{y})"


def parsePosHash(pos_hash):
    # Remove parentheses and split by commas
    parts = pos_hash[1:-1].split(",")

    # Extract x and y values
    x = int(parts[0].split(":")[1])
    y = int(parts[1].split(":")[1])

    return x, y


def AdjacentSymbols(lines, x, y):
    return {
        # y - 1
        posHash(x - 1, y - 1): pix(lines, x - 1, y - 1),
        posHash(x, y - 1): pix(lines, x, y - 1),
        posHash(x + 1, y - 1): pix(lines, x + 1, y - 1),
        # y
        posHash(x - 1, y): pix(lines, x - 1, y),
        posHash(x + 1, y): pix(lines, x + 1, y),
        # y + 1
        posHash(x - 1, y + 1): pix(lines, x - 1, y + 1),
        posHash(x, y + 1): pix(lines, x, y + 1),
        posHash(x + 1, y + 1): pix(lines, x + 1, y + 1),
    }


parts = {}


found_num = False
current_num = ""
adj_to_current_num = []

for x in range(0, len(lines[0])):
    for y in range(0, len(lines)):
        char = pix(lines, x, y)
        if isSym(char):
            parts[posHash(x, y)] = Part(x, y, char)

for y in range(0, len(lines)):
    for x in range(0, len(lines[0])):
        char = pix(lines, x, y)
        adjacent = AdjacentSymbols(lines, x, y)

        if isNum(pix(lines, x, y)):
            current_num += char
            found_num = True

        if isSym(char):
            ph = posHash(x, y)
            if not ph in parts:
                parts[ph] = Part(x, y, char)
            if found_num:
                if ph not in adj_to_current_num:
                    adj_to_current_num.append(ph)
                    print(f"{current_num} : {adj_to_current_num}")

        if not isNum(char):
            if found_num:
                for p in adj_to_current_num:
                    parts[p].numbers.append(current_num)
                found_num = False
                current_num = ""
                adj_to_current_num = []

        for ph, ch in adjacent.items():
            if ch:
                if isSym(ch):
                    if not ph in parts:
                        chx, chy = parsePosHash(ph)
                        parts[ph] = Part(chx, chy, ch)
                    if found_num:
                        if ph not in adj_to_current_num:
                            adj_to_current_num.append(ph)
                            print(f"{current_num} : {adj_to_current_num}")


sum1 = 0
sum2 = 0
for p in parts.values():
    if p.isValid():
        sum1 += p.sum()
        if p.isGear():
            print(f"gear: {p.__dict__}")
            sum2 += p.gearRatio()
print(f"Part1: {sum1}")
print(f"Part2: {sum2}")
