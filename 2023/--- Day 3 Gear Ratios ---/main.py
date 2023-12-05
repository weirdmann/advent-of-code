import re


puzzle = ""
with open("input") as file:
    puzzle = file.read()


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
    if prv:
        if isSym(prv[idx]):
            r = True
        if not first:
            if isSym(prv[idx - 1]):
                r = True
        if not last:
            if isSym(prv[idx + 1]):
                r = True
    if nxt:
        if isSym(nxt[idx]):
            r = True
        if not first:
            if isSym(nxt[idx - 1]):
                r = True
        if not last:
            if isSym(nxt[idx + 1]):
                r = True
    if not first:
        if isSym(cur[idx - 1]):
            r = True
    if not last:
        if isSym(cur[idx + 1]):
            r = True
    return r


found_num = False
current_num = ""
parts = []
for prv, cur, nxt in scanner(puzzle):
    i = 0
    for c in cur:
        # print(
        #     f"{i}:{c}, num:{isNum(c)}, dot:{isDot(c)}, sym:{isSym(c)}, adj:{hasAdjacentSymbol(prv,cur,nxt,i)}"
        # )
        if not isNum(c):
            pass
        if isNum(c) and hasAdjacentSymbol(prv, cur, nxt, c):
            pass
        i += 1
