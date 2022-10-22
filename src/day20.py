import numpy as np
import scipy.signal
import re

def rotate(sides, n_times=1):
    if n_times == 0:
        return sides
    if n_times > 1:
        sides = rotate(sides, n_times=n_times-1)
    return [
        -sides[1],
         sides[2],
        -sides[3],
         sides[0]
    ]

"""
---------
|   2   |
|1     3|
|   4   |
---------

"""

ORIENTATIONS = [
    # original
    rotate([1, 2, 3, 4], n_times=n) for n in range(4)
] + [
    # flip along y
    rotate([3,-2, 1,-4], n_times=n) for n in range(4)
] + [
    # flip along x
    rotate([-1, 4, -3, 2], n_times=n) for n in range(4)
] + [
    # flip along x and y
    rotate([-1,-2,-3,-4], n_times=n) for n in range(4)
]

class OrientationError(IndexError):
    pass

def adjust_tile(side_1, side_2, tile):
    # find the orientation of the tile
    orient = [i for i, o in enumerate(ORIENTATIONS) if (o[0]==side_1 and o[1]==side_2)]
    if len(orient) == 0:
        raise OrientationError("No orientation found!")
    else:
        orient = orient[0]
    # adjust image according to orientation
    if orient//4 == 1: # flip y
        tile = tile[: , ::-1]
    elif orient//4 == 2: # flip x
        tile = tile[::-1, :]
    elif orient//4 == 3: # flip x and y
        tile = tile[::-1, ::-1]
    return orient, np.rot90(tile, orient%4)

MONSTER="""
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   
"""

TEST_IMAGE = """
.#.#..#.##...#.##..#####
###....#.#....#..#......
##.##.###.#.#..######...
###.#####...#.#####.#..#
##.#....#.##.####...#.##
...########.#....#####.#
....#..#...##..#.#.###..
.####...#..#.....#......
#..#.##..#..###.#.##....
#.####..#.####.#.#.###..
###.#.#...#.######.#..##
#.####....##..########.#
##..##.#...#...#.#.#.#..
...#..#..#.#.##..###.###
.#.#....#.##.#...###.##.
###.#...#..#.##.######..
.#.#.###.##.##.#..#.##..
.####.###.#...###.#..#.#
..#.#..#..#.#.#.####.###
#..####...#.#.#.###.###.
#####..#####...###....##
#.##..#..#...#..####...#
.#.###..##..##..####.##.
...###...##...#...#..###
"""

def find_monster(image):
    pattern = np.array([[int(c) for c in row] for row in MONSTER.replace(" ", "0").replace("#", "1").split("\n") if len(row) > 0])
    number_monsters = []
    for image_flip in (image, image[::-1, :], image[:, ::-1], image[::-1, ::-1]):
        for n_rot in range(4):
            convolved = scipy.signal.convolve2d(np.rot90(image_flip, n_rot), pattern, 'valid') 
            number_monsters.append(
                np.sum(convolved == np.sum(pattern)))
    return number_monsters

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

    tile_numbers = np.array(list(tiles.keys()))
    tiles = list(tiles.values())
    tile_ids = np.stack([np.repeat(range(len(tiles)), 4), np.tile(range(1, 5), len(tiles))]).T
    tile_ids = np.vstack([tile_ids, -tile_ids])
    tile_ids[:, 0] = abs(tile_ids[:, 0])
    sides = np.vstack([np.stack([t[:, 0], t[0, :], t[:, -1], t[-1, :]]) for t in tiles])
    sides = np.vstack([sides, sides[:, ::-1]])
    # find all unique sides
    _, unique_side_indices, unique_side_inverse, unique_side_counts = np.unique(sides, return_index=True, return_inverse=True, return_counts=True, axis=0)

    tiles_and_sides_with_no_match = tile_ids[unique_side_indices[unique_side_counts==1]]
    # removed fliped versions
    tiles_and_sides_with_no_match = tiles_and_sides_with_no_match[tiles_and_sides_with_no_match[:, 1] > 0]
    side_tiles, side_tiles_indices, side_tiles_counts = np.unique(tiles_and_sides_with_no_match[:, 0], return_index=True, return_counts=True)

    corner_tiles = tile_numbers[side_tiles[side_tiles_counts==2]]
    part1_sol = np.product(corner_tiles)
    print("Part 1:", part1_sol)

    # image composition with (tile, orientaion)
    image = np.zeros((12, 12, 2), dtype=int)
    image_tiles = [[[] for i in range(12)] for j in range(12)]
    # Begin with top-right corner
    tile_id = side_tiles[side_tiles_counts==2][0]
    image[0, 0, 0] =  tile_id
    corner_sides = tiles_and_sides_with_no_match[tiles_and_sides_with_no_match[:, 0] == tile_id, 1]
    image[0, 0, 1], image_tiles[0][0] =  adjust_tile(corner_sides[0], -corner_sides[1], tiles[tile_id])
    print(side_tiles[side_tiles_counts==2])
    # fill the first row
    for i in range(1, 12):
        candidates = [id_ for id_, s in zip(tile_ids, sides) if np.all(image_tiles[i-1][0][:, -1] == s) and id_[0] != image[i-1, 0, 0]]
        if len(candidates) == 0:
            raise ValueError("No candidates!")
        if len(candidates) > 1:
            raise ValueError("Too many candidates!")
        tile_id, side1 = candidates[0]
        image[i, 0, 0] =  tile_id
        for side2 in tiles_and_sides_with_no_match[tiles_and_sides_with_no_match[:, 0] == tile_id, 1]:
            for sign in [1, -1]: 
                try:
                    image[i, 0, 1], image_tiles[i][0] = adjust_tile(side1, sign * side2, tiles[tile_id])
                except OrientationError:
                    pass
                else:
                    break
    # Fill the first column
    for j in range(1, 12):
        candidates = [id_ for id_, s in zip(tile_ids, sides) if np.all(image_tiles[0][j-1][-1, :] == s) and id_[0] != image[0, j-1, 0]]
        if len(candidates) == 0:
            raise ValueError("No candidates!")
        if len(candidates) > 1:
            raise ValueError("Too many candidates!")
        tile_id, side2 = candidates[0]
        image[0, j, 0] =  tile_id
        for side1 in tiles_and_sides_with_no_match[tiles_and_sides_with_no_match[:, 0] == tile_id, 1]:
            for sign in [1, -1]: 
                try:
                    image[0, j, 1], image_tiles[0][j] = adjust_tile(sign * side1, side2, tiles[tile_id])
                except OrientationError:
                    pass
                else:
                    break
    # Fill out all the rest
    for i in range(1, 12):
        for j in range(1, 12):
            candidates = [id_ for id_, s in zip(tile_ids, sides) if np.all(image_tiles[i-1][j][:, -1] == s) and id_[0] != image[i-1, j, 0]]
            if len(candidates) == 0:
                raise ValueError("No candidates!")
            if len(candidates) > 1:
                raise ValueError("Too many candidates!")
            tile_id, side1 = candidates[0]
            candidates = [id_ for id_, s in zip(tile_ids, sides) if np.all(image_tiles[i][j-1][-1, :] == s) and id_[0] != image[i, j-1, 0]]
            if len(candidates) == 0:
                raise ValueError("No candidates!")
            if len(candidates) > 1:
                raise ValueError("Too many candidates!")
            tile_id2, side2 = candidates[0]
            assert tile_id == tile_id2
            image[i, j, 0] =  tile_id
            image[i, j, 1], image_tiles[i][j] = adjust_tile(side1, side2, tiles[tile_id])

    for i in range(1, 11):
        for j in range(1, 11):
            assert np.all(image_tiles[i-1][j][:, -1] == image_tiles[i][j][:, 0])
            assert np.all(image_tiles[i][j-1][-1, :] == image_tiles[i][j][0, :])
            assert np.all(image_tiles[i+1][j][:, 0] == image_tiles[i][j][:, -1])
            assert np.all(image_tiles[i][j+1][0, :] == image_tiles[i][j][-1, :])
    # Remove borders from each tile to form image 
    formed_image = np.block([[img[1:-1, 1:-1] for img in row] for row in image_tiles])
    print(image[0, 0, 0], image[0, -1, 0], image[-1, 0, 0], image[-1, -1, 0])

    test_image = np.array([[int(c) for c in row] for row in TEST_IMAGE.replace(".", "0").replace("#", "1").split("\n") if len(row) > 0])
    print(find_monster(test_image))

    part2_sol = find_monster(formed_image)
    print("Part 2:", part2_sol)

if __name__ == "__main__":
    solve()