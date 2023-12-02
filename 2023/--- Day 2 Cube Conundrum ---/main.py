import re


puzzle = ""
with open("input") as file:
    puzzle = file.read()


class Pick:
    def __init__(self, text):
        self.red = 0
        self.green = 0
        self.blue = 0

        if re.search("red", text):
            self.red = int(re.search("(\d+) red", text).group(1))

        if re.search("green", text):
            self.green = int(re.search("(\d+) green", text).group(1))

        if re.search("blue", text):
            self.blue = int(re.search("(\d+) blue", text).group(1))


class Game:
    def __init__(self, line):
        n, picks = line.split(":")
        self.n = int(re.search("\d+", n).group(0))
        self.picks = [Pick(p) for p in picks.split(";")]

    def is_possible(self, max_red, max_green, max_blue):
        for p in self.picks:
            if p.red > max_red:
                return False
            if p.green > max_green:
                return False
            if p.blue > max_blue:
                return False
        return True

    def get_power(self):
        min_red = 0
        min_green = 0
        min_blue = 0

        for p in self.picks:
            min_red = max(min_red, p.red)
            min_green = max(min_green, p.green)
            min_blue = max(min_blue, p.blue)

        return min_red * min_green * min_blue


lines = puzzle.splitlines()
games = []
result1 = 0
result2 = 0
for line in lines:
    g = Game(line)
    if g.is_possible(12, 13, 14):
        result1 += g.n
    result2 += g.get_power()
    games.append(g)
print(f"Answer 1: {result1}")
print(f"Answer 2: {result2}")
