from dataclasses import dataclass
from typing import List
import re
import z3


@dataclass
class Food:
    ingredients: List[str]
    allergens: List[str]
    

def solve():

    re_line = re.compile(r"(.+) \(contains (.+)\)")
    foods: List[Food] = []
    with open("inputs/day21") as f:
        for line in f:
            if (match := re.match(re_line, line)):
                foods.append(
                    Food(
                        match.group(1).split(" "),
                        match.group(2).split(", ")
                    )
                )
    
    # Global indices
    ingredient_number = {}
    allergen_vars = {}
    for f in foods:
        for i in f.ingredients:
            if i not in ingredient_number:
                ingredient_number[i] = len(ingredient_number)
        for a in f.allergens:
            if a not in allergen_vars:
                allergen_vars[a] = z3.Int(a)
    
    s = z3.Solver()
    # Allergen in only found in one ingredient
    s.add(z3.Distinct(*allergen_vars.values()))

    # Add each food
    for f in foods:
        for a in f.allergens:
            s.add(z3.Or([allergen_vars[a] == ingredient_number[i] for i in f.ingredients]))

    # solve
    s.check()
    m = s.model()
    print(m)
    contains_allergen = {m[v].as_long(): str(v) for v in m}

    inv_ingredient_number = list(ingredient_number.keys())
    ingredients_without_allergen = [i for i, i_nr in ingredient_number.items() if i_nr not in contains_allergen]

    part1_sol = sum([len(set(f.ingredients).intersection(ingredients_without_allergen)) for f in foods])
    print("part 1:", part1_sol)

    ingredients_with_allergen = {i: contains_allergen[i_nr] for i, i_nr in ingredient_number.items() if i_nr in contains_allergen}
    part2_sol = ",".join(sorted(ingredients_with_allergen, key= lambda i: ingredients_with_allergen[i]))
    print("part 2:", part2_sol)

if __name__ == "__main__":
    solve()