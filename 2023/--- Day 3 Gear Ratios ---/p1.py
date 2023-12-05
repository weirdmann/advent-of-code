import re


def scanner(text):
    lines = text.splitlines()
    previous_line = None
    current_line = None
    next_line = None

    # yield three lines at a time, yielding None if first or last
    for line in lines:
        previous_line = current_line
        current_line = next_line
        next_line = line

        if current_line is None:
            continue

        yield previous_line, current_line, next_line
    yield current_line, next_line, None


def isNum(c: str):
    return c.isnumeric()


def isDot(c: str):
    return c == "."


def isSym(c: str):
    return not isNum(c) and not isDot(c)


def hasAdjacentSymbol(prv: str, cur: str, nxt: str, idx: int):
    r = False
    first = idx == 0
    last = idx == len(cur) - 1
    n_of_symbols = 0
    if prv:
        if isSym(prv[idx]):
            r = True
            n_of_symbols += 1
        if not first:
            if isSym(prv[idx - 1]):
                r = True
                n_of_symbols += 1
        if not last:
            if isSym(prv[idx + 1]):
                r = True
                n_of_symbols += 1
    if nxt:
        if isSym(nxt[idx]):
            r = True
            n_of_symbols += 1
        if not first:
            if isSym(nxt[idx - 1]):
                r = True
                n_of_symbols += 1
        if not last:
            if isSym(nxt[idx + 1]):
                r = True
                n_of_symbols += 1
    if not first:
        if isSym(cur[idx - 1]):
            r = True
            n_of_symbols += 1
    if not last:
        if isSym(cur[idx + 1]):
            r = True
            n_of_symbols += 1
    return r, n_of_symbols


if __name__ == "__main__":
    puzzle = ""
    with open("input") as file:
        puzzle = file.read()

    found_num = False
    current_num = ""
    current_num_is_a_part = False
    current_num_n_of_s = 0
    parts = []
    gears = []
    for prv, cur, nxt in scanner(puzzle):
        i = 0
        for c in cur:
            # print(
            #     f"{i}:{c}, num:{isNum(c)}, dot:{isDot(c)}, sym:{isSym(c)}, adj:{hasAdjacentSymbol(prv,cur,nxt,i)}"
            # )
            has, n_of_s = hasAdjacentSymbol(prv, cur, nxt, i)
            if not isNum(c):
                if found_num:
                    if current_num_is_a_part:
                        parts.append(current_num)
                    found_num = False
                    current_num = ""
                    current_num_is_a_part = False
                    current_num_n_of_s = 0
            if isNum(c):
                current_num += c
                found_num = True
                if has:
                    current_num_is_a_part = True
                    current_num_n_of_s += n_of_s
                pass
            i += 1

    print(parts)
    sum = 0
    for p in parts:
        sum += int(p)
    print(sum)
