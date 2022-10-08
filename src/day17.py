import itertools
import numpy as np


def evolve(active_blocks):
    sizes = [(c.max() + 3) for c in active_blocks]
    neighbours = np.zeros(sizes, int)
    for offset in itertools.product([-1, 0, 1], repeat=len(active_blocks)):
        neighbours[tuple(c + o + 1 for c, o in zip(active_blocks, offset))] += 1


    active = np.zeros(sizes, bool)
    active[tuple(c + 1 for c in active_blocks)] = True
    neighbours[active] -= 1
    deactivate = active * ~((neighbours == 2) + (neighbours == 3))
    activate = ~active * (neighbours == 3)
    active[deactivate] = False
    active[activate] = True
    return np.nonzero(active)


def solve():
    with open("inputs/day17") as f:
        file_contents = f.readlines()

    block_map = np.array([[[c == "#"] for c in line.strip()] for line in file_contents])
    active_blocks = np.nonzero(block_map)
    for _ in range(6):
        active_blocks = evolve(active_blocks)
    print("Part 1 solution: ", len(active_blocks[0]))

    block_map = np.array([[[[c == "#"]] for c in line.strip()] for line in file_contents])
    active_blocks = np.nonzero(block_map)
    for _ in range(6):
        active_blocks = evolve(active_blocks)
    print("Part 2 solution: ", len(active_blocks[0]))


if __name__ == "__main__":
    solve()
