/// Solution to Advent of Code Challenge Day 20.
use aoc2020::{get_day_input, print_elapsed_time};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io;
use std::str::FromStr;

const DAYNUM: &'static str = "20";
type ChallengeData = Vec<Tile>;
type ChallengeOut = u64;

// Respresent a row as a bitfield representing on (#) or off (.) pixels in a 10 byte array.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct TileRow(u16);

impl TileRow {
    fn from_vec(v: &Vec<bool>) -> Self {
        let mut bitfield: u16 = 0;
        for (i, p) in v.iter().rev().enumerate() {
            if *p {
                bitfield |= 1 << i;
            }
        }
        Self(bitfield)
    }

    /// Flip the binary represention i.e. 1010101011 -> 1101010101
    fn flip(&self) -> Self {
        let mut new = 0;
        for i in 0..10 {
            if (self.0 & 1 << i) != 0 {
                new |= 1 << (10 - 1 - i)
            }
        }
        Self(new)
    }
}

#[derive(Clone, Debug)]
struct Tile {
    id: u32,
    rows: Vec<Vec<bool>>,

    // Map [top, right, bottom, left] neighbour to [0, 1, 2, 3] key pointing to tile ID
    adjacent: HashMap<usize, u32>,
    // Mark a tile as free to rotate/flip or already fixed into the puzzle board
    fixed: bool,
}

impl FromStr for Tile {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, grid) = s.split(":\n").next_tuple().unwrap();
        Ok(Self {
            id: label.trim_start_matches("Tile ").parse().unwrap(),
            rows: grid
                .lines()
                .map(|s| {
                    s.chars()
                        .map(|ch| match ch {
                            '#' => true,
                            '.' => false,
                            _ => panic!("Bad character in tile map"),
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
            adjacent: HashMap::new(),
            fixed: false,
        })
    }
}

impl Tile {
    /// Return the edges of the tile in [top, right, bottom, left] order for its current
    /// orientation.
    fn edges(&self) -> [TileRow; 4] {
        let top = TileRow::from_vec(&self.rows[0]);
        // Convention for the bottom edge is if the tile were rotated 180 degrees, it would have the
        // same value as the bottom edge, i.e. convention is left to right
        let bottom = TileRow::from_vec(&self.rows[self.rows.len() - 1]).flip();
        // Convention for right edge is bottom to top
        let mut right = TileRow(0);
        // Convention for right edge is top to bottom
        let mut left = TileRow(0);
        for (i, row) in self.rows.iter().enumerate() {
            // Pixel is "on" on LHS
            if row[0] {
                left.0 |= 1 << i;
            }
            // Pixel is "on" on RHS
            if row[self.rows.len() - 1] {
                right.0 |= 1 << (self.rows.len() - 1 - i);
            }
        }
        [top, right, bottom, left]
    }

    /// Rotate the tile 90 degrees clockwise (if it free to move).
    fn rotate(&mut self) {
        assert!(!self.fixed);
        rotate_pixels(&mut self.rows);
    }

    /// Flip the tile about the vertical axis (if it is free to move).
    fn flip(&mut self) {
        assert!(!self.fixed);
        flip_pixels(&mut self.rows);
    }
}

/// Flip a set of pixels as a grid about the vertical axis.
fn flip_pixels(grid: &mut Vec<Vec<bool>>) {
    for row in grid.iter_mut() {
        row.reverse();
    }
}

/// Rotate a set of pixels as a grid 90 degrees clockwise.
fn rotate_pixels(grid: &mut Vec<Vec<bool>>) {
    let mut new_grid = Vec::new();
    let len = grid[0].len();
    for x in 0..len {
        let mut new_row: Vec<bool> = Vec::new();
        for y in 1..=len {
            new_row.push(grid[len - y][x]);
        }
        new_grid.push(new_row);
    }
    *grid = new_grid;
}

/// Find all of the tiles neighbours, flipping and rotating the tile as appropriate.
///
/// Start with a random tile. This is marked as fixed, and build the rest of the puzzle
/// around this tile. Its sides will match sides from another tile (which isn't in the done pile).
///
/// When a tile is found that matches, remove it from the stack of to be processed and add it to the
/// top of the stack with the correct orientation (unless it now has 4 neighbours, then add it to
/// the done pile). When a tile has had all 4 sides processed, add it to the done pile anyway.
///
/// Then pop the stack and process the next tile until there are no tiles left: the puzzle pieces
/// should all be connected together and oriented correctly.
fn match_puzzle(tiles: &mut ChallengeData) {
    let mut processing_stack: Vec<Tile> = tiles.drain(..).collect();
    let mut done_stack: Vec<Tile> = Vec::new();

    while let Some(mut processing_tile) = processing_stack.pop() {
        processing_tile.fixed = true;
        // Go over the current tile in [top, right, bottom, left] order.
        for (i, edge) in processing_tile.edges().iter().map(|e| e.flip()).enumerate() {
            // Seek the tile which contains this edge (or this edge flipped).
            let edge_idx = processing_stack.iter().position(|t| {
                t.edges().contains(&edge) || (t.edges().contains(&edge.flip()) && !t.fixed)
            });
            if edge_idx.is_none() {
                continue;
            }
            let mut edge_match = processing_stack.remove(edge_idx.unwrap());
            if !edge_match.fixed {
                if !edge_match.edges().contains(&edge) {
                    // The tile once flipped will contain an edge which can be rotated into
                    // position.
                    edge_match.flip();
                }
                while edge_match.edges()[(i + 2) % 4] != edge {
                    // Rotate until that edge matches our edge if the tile were rotated 180
                    // degrees.
                    edge_match.rotate();
                }
                edge_match.fixed = true;
            }

            // Check the piece is in the correct orientation.
            if edge_match.edges()[(i + 2) % 4] != edge {
                panic!("Found a fixed puzzle piece that wasn't already oriented to fit this piece");
            }

            processing_tile.adjacent.insert(i, edge_match.id);
            edge_match.adjacent.insert((i + 2) % 4, processing_tile.id);

            if edge_match.adjacent.len() > 3 {
                // Piece is surrounded and so is complete.
                done_stack.push(edge_match);
            } else {
                // Piece isn't surrounded yet so put it back: however, put it back on the top of the
                // stack to try and "work outward" from the original random seed piece.
                processing_stack.push(edge_match);
            }
        }
        // Checked all 4 edges, if we haven't got 4 neighbours we're a corner or edge piece.
        done_stack.push(processing_tile);
    }

    *tiles = done_stack;
}

fn strip_border(grid: &mut Vec<Vec<bool>>) {
    grid.remove(0);
    grid.pop();
    for row in grid.iter_mut() {
        row.remove(0);
        row.pop();
    }
}

fn arrange_tiles(tiles: &[Tile]) -> Vec<Vec<&Tile>> {
    let mut grid: Vec<Vec<&Tile>> = Vec::new();
    let id_map: HashMap<u32, &Tile> = tiles.iter().map(|tile| (tile.id, tile)).collect();
    // Find the top left corner: using [top, right, bottom, left] it will have no neighbour in "0"
    // position nor in "3" position.
    let mut corner = tiles
        .iter()
        .find(|tile| {
            tile.adjacent.len() == 2
                && !tile.adjacent.contains_key(&0)
                && !tile.adjacent.contains_key(&3)
        })
        .unwrap();
    // While the corner of the current row has a pointer to the next row ("down" == 2).
    loop {
        let mut new_row: Vec<&Tile> = Vec::new();
        let mut curr_tile = corner;
        // While there is a tile to the right.
        loop {
            new_row.push(curr_tile);
            if !curr_tile.adjacent.contains_key(&1) {
                break;
            }
            curr_tile = id_map.get(curr_tile.adjacent.get(&1).unwrap()).unwrap();
        }
        grid.push(new_row);
        if !corner.adjacent.contains_key(&2) {
            break;
        }
        corner = id_map.get(corner.adjacent.get(&2).unwrap()).unwrap();
    }
    grid
}

/// Strip borders from each tile and then form them into a singular image of "pixels", using their
/// calculated adjacent neighbours.
fn form_image(tiles: &Vec<Tile>) -> Vec<Vec<bool>> {
    let mut tiles: Vec<Tile> = tiles.to_owned();
    let mut image: Vec<Vec<bool>> = Vec::new();
    for tile in tiles.iter_mut() {
        strip_border(&mut tile.rows);
    }
    let puzzle: Vec<Vec<&Tile>> = arrange_tiles(&tiles);
    for row in puzzle {
        let len = row[0].rows.len();
        let mut new_rows = vec![Vec::<bool>::new(); len];
        for tile in row {
            for (i, row) in tile.rows.iter().enumerate() {
                new_rows[i].extend(row);
            }
        }
        image.extend(new_rows);
    }
    image
}

/// Find the number of sea monsters in an image.
///
/// Sea monsters have the following form:
///     |                  # |
///     |#    ##    ##    ###|
///     | #  #  #  #  #  #   |
/// They must be contiguous and are assumed to not overlap (share pixels).
fn find_number_sea_monsters(image: &Vec<Vec<bool>>) -> u64 {
    let sea_monster = [
        (0, 18),
        (1, 0),
        (1, 5),
        (1, 6),
        (1, 11),
        (1, 12),
        (1, 17),
        (1, 18),
        (1, 19),
        (2, 1),
        (2, 4),
        (2, 7),
        (2, 10),
        (2, 13),
        (2, 16),
    ];
    let mut monsters_found = 0;
    // Sea monster is three rows high, so last row to check should be 3 from the bottom
    for x in 0..=image.len() - 3 {
        // Monster is 20 columns long, so last col to check should be 20 from the right (note: image
        // is square)
        for y in 0..=image.len() - 20 {
            monsters_found += sea_monster.iter().all(|(dx, dy)| image[x + dx][y + dy]) as u64;
        }
    }
    monsters_found
}

/// Seek the image for sea monsters, then get the water roughness of the orientation of the image which
/// contains them.
fn get_water_roughness(image: &Vec<Vec<bool>>) -> u64 {
    let mut image = image.clone();
    let mut num_monsters = find_number_sea_monsters(&image);
    while num_monsters == 0 {
        rotate_pixels(&mut image);
        num_monsters = find_number_sea_monsters(&image);
        if num_monsters == 0 {
            // Try with this rotation flipped
            flip_pixels(&mut image);
            num_monsters = find_number_sea_monsters(&image);
        }
    }
    // There are 15 pixels in a sea monster, so subtract the number of sea monsters times 15 from
    // the number of filled pixels in the image
    image.iter().flatten().map(|p| *p as u64).sum::<u64>() - (num_monsters * 15)
}

/// Solution to part one.
fn part_one(data: &ChallengeData) -> Option<ChallengeOut> {
    let all_edges: HashMap<u32, [TileRow; 4]> =
        data.iter().map(|tile| (tile.id, tile.edges())).collect();
    let mut corners: Vec<u32> = Vec::new();
    // Corners are defined as tiles which have two sides which no matter how they are flipped are
    // not the same as any other edge (or its mirror).
    for (id, edges) in all_edges.iter() {
        let mut all_other_edges: HashSet<TileRow> = HashSet::new();
        let mut non_fitting_edges: u32 = 0;
        for (id2, edges2) in all_edges.iter() {
            if id2 == id {
                continue;
            }
            all_other_edges.extend(edges2);
            all_other_edges.extend(edges2.iter().map(|e| e.flip()));
        }
        for edge in edges {
            if !all_other_edges.contains(edge) && !all_other_edges.contains(&edge.flip()) {
                non_fitting_edges += 1;
            }
        }
        if non_fitting_edges > 1 {
            corners.push(*id);
        }
    }
    Some(corners.iter().map(|num| *num as u64).product())
}

/// Solution to part two.
fn part_two(data: &ChallengeData) -> Option<ChallengeOut> {
    let mut tiles = data.clone();
    match_puzzle(&mut tiles);
    let image = form_image(&tiles);
    // Check that the image is square
    assert!(image.iter().all(|row| row.len() == image.len()));
    Some(get_water_roughness(&image))
}

fn get_data(input: String) -> Result<ChallengeData, io::Error> {
    input.split("\n\n").map(|s| s.parse()).collect()
}

fn main() -> Result<(), io::Error> {
    println!("Day {}:", DAYNUM);
    println!("==========");
    println!("Getting data...");
    let data = print_elapsed_time(|| get_data(get_day_input(DAYNUM)))?;
    println!("==========");
    println!("Solving part one...");
    let ans1 = print_elapsed_time(|| part_one(&data)).expect("No solution found for part one");
    println!("Answer: {}", ans1);
    println!("==========");
    println!("Solving part two...");
    let ans2 = print_elapsed_time(|| part_two(&data)).expect("No solution found for part two");
    println!("Answer: {}", ans2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."
            .to_string();

        let data = get_data(input.to_string()).expect("Couldn't convert test input");

        // Assert get the right number.
        assert_eq!(part_one(&data), Some(1951 * 3079 * 2971 * 1171));
        assert_eq!(part_two(&data), Some(273));
    }
}
