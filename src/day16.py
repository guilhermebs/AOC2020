from dataclasses import dataclass
import functools
import itertools
import re
from typing import List, Sequence

import z3

@dataclass
class Interval:
    lb: int
    ub: int

    def validate(self, x: int):
        return self.lb <= x <= self.ub
    
    def __str__(self) -> str:
        return f"{self.lb}-{self.ub}"


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

    def __str__(self):
        return f"{self.name}: " + " ".join(str(i) for i in self.intervals)
    
    def __repr__(self) -> str:
        return str(self)


def solve():
    with open("inputs/day16", 'r') as f:
        contents = [ln.strip() for ln in f.readlines()]

    contents = iter(contents)
    rules: list[TicketField] = []
    for ln in contents:
        if len(ln) == 0:
            break
        rules.append(TicketField.from_string(ln))
    
    tickets: List[List[int]] = []
    next(contents)
    tickets.append([int(s) for s in next(contents).split(",")])

    next(contents)
    next(contents)

    for ln in contents:
        if len(ln) == 0:
            break
        tickets.append([int(s) for s in ln.split(",")])

    prob1_sol = 0
    valid_tickets: List[List[int]] = []
    for ticket in tickets:
        for nr in ticket:
            if all(not r.apply(nr) for r in rules):
                prob1_sol += nr
                break
        else:
            valid_tickets.append(ticket)

    print("part 1:", prob1_sol)

    fields = [z3.Int(f"F{i}") for i, _ in enumerate(rules)]
    s = z3.Solver()
    s.add(z3.Distinct(*fields))
    applicable = []
    for i in range(len(valid_tickets[0])):
        applicable.append([r for r in rules if all(r.apply(t[i]) for t in valid_tickets)])
        s.add(z3.Or([fields[i] == j for j, r in enumerate(rules) if all(r.apply(t[i]) for t in valid_tickets)]))

    s.check()
    m = s.model()
    ordered_rules = [rules[m[f].as_long()] for f in fields]

    departures = [f for f, r in zip(tickets[0], ordered_rules) if r.name.startswith("departure")]
    part2_sol = functools.reduce(lambda x, y: x * y, departures, 1)
    print("part 2:", part2_sol)
       

if __name__ == "__main__":
    solve()