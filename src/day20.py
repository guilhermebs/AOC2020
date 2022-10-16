import numpy as np
import re


def rotate(sides, n_times=1):
    if n_times > 1:
        sides = rotate(sides, n_times=n_times-1)
    return [
        sides[3][::-1],
        sides[0],
        sides[1],
        sides[2][::-1],
    ]


def solve():
    with open("inputs/day20") as f:
        file_contents = f.read()

    re_title = re.compile(r"Tile (\d+):")
    tiles = {}
    for tile_string in file_contents.split("\n\n"):
        tile_string = tile_string.replace(".", "0").replace("#", "1")
        tile_desc = tile_string.split("\n")
        if (match := re_title.match(tile_desc[0])) is not None:
            tile_nr = int(match.group(1))
        else:
            raise ValueError("invalid tile") 
        tiles[tile_nr] = np.array([list(map(lambda x: int(x), t)) for t in tile_desc[1:]])
        
    tile_ids = np.stack([np.repeat(list(tiles.keys()), 4), np.tile(range(1, 5), len(tiles))]).T
    tile_ids = np.vstack([tile_ids, -tile_ids])
    tile_ids[:, 0] = abs(tile_ids[:, 0])
    sides = np.vstack([np.stack([t[0, :], t[:, -1], t[-1, :], t[:, 0]]) for t in tiles.values()])
    sides = np.vstack([sides, sides[:, ::-1]])
    # find all unique sides
    _, unique_side_indices, unique_side_inverse, unique_side_counts = np.unique(sides, return_index=True, return_inverse=True, return_counts=True, axis=0)

    tiles_and_sides_with_no_match = tile_ids[unique_side_indices[unique_side_counts==1]]
    # removed fliped versions
    tiles_and_sides_with_no_match = tiles_and_sides_with_no_match[tiles_and_sides_with_no_match[:, 1] > 0]
    side_tiles, side_tiles_indices, side_tiles_counts = np.unique(tiles_and_sides_with_no_match[:, 0], return_index=True, return_counts=True)
    part1_sol = np.product(side_tiles[side_tiles_counts==2])
    print(side_tiles[side_tiles_counts==2])
    print("Part 1:", part1_sol)

if __name__ == "__main__":
    solve()