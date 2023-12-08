puzzle = ""
with open("input") as file:
    puzzle = file.read()
lines = puzzle.splitlines()


class Map:
    def __init__(self, title: str):
        self.title: str = title
        self.lines: list[str] = []
        self.ranges: list[Range] = []

    def generate_ranges(self):
        for line in self.lines:
            ds, ss, ll = line.split()
            self.ranges.append(Range(ds, ss, ll))

    def calculate(self, number):
        for r in self.ranges:
            if r.fits(number):
                return r.calculate(number)
        return number


class Range:
    def __init__(self, dst_start, src_start, length):
        self.dst_start = int(dst_start)
        self.src_start = int(src_start)
        self.length = int(length)

    def fits(self, number):
        # print(
        #     f"""{self.__dict__} : {number} : {int(number) >= self.src_start
        #     and int(number) <= self.src_start + self.length} """
        # )
        return (
            int(number) >= self.src_start
            and int(number) <= self.src_start + self.length
        )
        # return int(number) in range(self.src_start, self.src_start + self.length + 1)

    def calculate(self, number):
        if not self.fits(number):
            return int(number)
        else:
            return int(number) - self.src_start + self.dst_start


if __name__ == "__main__":
    seeds = []
    seed_line = True

    current_map = None
    maps = []

    for line in lines:
        if seed_line:
            seed_line = False
            _, s = line.split(":")
            s = s.strip()
            seeds = s.split()

            continue

        if line == "":
            if current_map:
                current_map.generate_ranges()
                maps.append(current_map)
                current_map = None
            continue

        if line[0].isnumeric():
            if current_map:
                current_map.lines.append(line)
        else:
            current_map = Map(line)
    if current_map:
        current_map.generate_ranges()
        maps.append(current_map)
        current_map = None

    # seed : location
    locations = {}

    nseeds = []
    start = True
    seed_start = 0
    seed_range = 0
    for s in seeds:
        if start:
            seed_start = int(s)
            start = False
        else:
            seed_range = int(s)
            start = True
            for i in range(seed_start, seed_range + 1):
                nseeds.append(i)

    for s in nseeds:
        location = s
        for m in maps:
            new_loc = m.calculate(location)
            # print(f"Seed {s}: {m.title} {location} -> {new_loc}")
            location = new_loc
        locations[s] = int(location)

    # print(locations)
    # #print([m.__dict__ for m in maps])

    print(f"Part1: {min(locations.values())}")
