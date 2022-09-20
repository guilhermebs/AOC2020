from dataclasses import dataclass
from importlib.resources import contents
from operator import ne
import re
from typing import Sequence

@dataclass
class Interval:
    lb: int
    ub: int

    def validate(self, x: int):
        return self.lb <= x <= self.ub


class TicketField:
    def __init__(self, name: str, intervals: Sequence[Interval]) -> None:
        self.name: str = name
        self.intervals: Sequence[Interval] = intervals
    
    @classmethod
    def from_string(self, string: str):
        if match := re.match(r"([\w\s]+): (\d+)-(\d+) or (\d+)-(\d+)$", string):
            name = match.group(1)
            intervals = [Interval(int(match.group(2)), int(match.group(3))), Interval(int(match.group(4)), int(match.group(5)))]
        else:
            raise ValueError(f"Invalid string!: {string}")

        return TicketField(name, intervals)
    
    def apply(self, nr:int) -> bool:
        return any(interval.validate(nr) for interval in self.intervals)
        


def solve():
    with open("inputs/day16", 'r') as f:
        contents = [ln.strip() for ln in f.readlines()]

    contents = iter(contents)
    rules: list[TicketField] = []
    for ln in contents:
        if len(ln) == 0:
            break
        rules.append(TicketField.from_string(ln))
    
    tickets = []
    next(contents)
    tickets.append([int(s) for s in next(contents).split(",")])

    next(contents)
    next(contents)

    for ln in contents:
        if len(ln) == 0:
            break
        tickets.append([int(s) for s in ln.split(",")])

    prob1_sol = 0
    for ticket in tickets:
        for nr in ticket:
            if not any(r.apply(nr) for r in rules):
                prob1_sol += nr

    print("part 1:", prob1_sol)
       

if __name__ == "__main__":
    solve()