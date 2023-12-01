puzzle = ""
with open("input") as file:
    puzzle = file.read().lower()


words = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}


def possibleWords(l):
    d = {}
    for k, v in words.items():
        if len(k) <= l:
            d[k] = v
    return d


def replace(line):
    for k, v in words.items():
        line = line.replace(k, str(v))
    return line


def getDigit(line, pos: int):
    l = line[int(pos) :]
    if l[0].isdigit():
        return l[0]
    for i in range(0, len(l) + 1):
        if i > 5:
            break
        li = l[:i]
        for k, v in possibleWords(i).items():
            if li == k:
                return v
    return None


save = ""
sum = 0
for line in puzzle.splitlines():
    first = ""
    last = ""
    for idx, char in enumerate(line):
        d = getDigit(line, idx)
        save += f"{line}, {idx}, {char}, {d if d else ''}\n"
        if d:
            if first == "":
                first = d
            last = d
    save += f"{first}{last}\n"
    sum += int(f"{first}{last}")
print(sum)

with open("save", "w+") as file:
    file.write(save)
