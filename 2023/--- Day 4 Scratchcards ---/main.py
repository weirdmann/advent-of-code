puzzle = ""
with open("input") as file:
    puzzle = file.read()
lines = puzzle.splitlines()


class Card:
    def __init__(self, n, winning, scratched):
        self.n = n
        self.winning = winning
        self.scratched = scratched
        self.matched = []
        for n in self.scratched:
            if n in self.winning:
                self.matched.append(n)
        self.n_of_matches = len(self.matched)

        self.points = self.calc_points()

    def calc_points(self):
        if self.n_of_matches == 0:
            return 0
        if self.n_of_matches == 1:
            return 1
        return 2 ** (self.n_of_matches - 1)


cards: list[Card] = []
card_dict = {}
for line in lines:
    card, numbers = line.split(":")
    card = card.replace("  ", " ")
    card = card.replace("  ", " ")
    _, n = card.split(" ")

    numbers = numbers.replace("  ", " ")
    winning, scratched = [n.strip() for n in numbers.split("|")]
    winning = [int(n) for n in winning.split(" ")]
    scratched = [int(n) for n in scratched.split(" ")]
    c = Card(int(n), winning, scratched)
    cards.append(c)
    card_dict[int(n)] = 1


sum1 = 0
for c in cards:
    print(c.__dict__)
    sum1 += c.points

print(f"Part1: {sum1}")


for c in cards:
    i = 0
    for _ in c.matched:
        i = i + 1
        card_dict[c.n + i] += card_dict[c.n]

print(card_dict)


sum2 = 0
for k, v in card_dict.items():
    sum2 += v
print(f"Part2: {sum2}")
