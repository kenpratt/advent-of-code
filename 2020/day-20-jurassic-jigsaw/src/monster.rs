use indoc::indoc;

use crate::tile::*;

static SEA_MONSTER: &str = indoc! {"
    Tile 0:
    ..................#.
    #....##....##....###
    .#..#..#..#..#..#...
"};

static ORIENTATIONS: &[(Direction, usize); 8] = &[
    (Direction::Clockwise, 0),
    (Direction::Clockwise, 1),
    (Direction::Clockwise, 2),
    (Direction::Clockwise, 3),
    (Direction::Counterclockwise, 0),
    (Direction::Counterclockwise, 1),
    (Direction::Counterclockwise, 2),
    (Direction::Counterclockwise, 3),
];

pub fn count_monster_pixels(tile: &Tile) -> usize {
    let monster_tile = Tile::parse(SEA_MONSTER);
    let num = count_monsters(tile, &monster_tile);
    num * monster_tile.count_active_pixels()
}

fn count_monsters(tile: &Tile, monster: &Tile) -> usize {
    let monster_line_values = Tile::line_values(&monster.pixels);
    ORIENTATIONS.iter().map(|(direction, rotation)| {
        count_monsters_for_orientation(tile, monster, &monster_line_values, direction, rotation)
    }).max().unwrap()
}

fn count_monsters_for_orientation(tile: &Tile, monster: &Tile, monster_line_values: &Vec<TileValue>, direction: &Direction, rotation: &usize) -> usize {
    let tile_pixels = tile.calculate_pixels(direction, rotation, false);
    let tile_line_values = Tile::line_values(&tile_pixels);

    let mut monster_count = 0;
    for y in 0..=(tile.height - monster.height) {
        for x in 0..=(tile.width - monster.width) {
            if is_monster_at_location(&tile_line_values, &monster_line_values, x, y) {
                monster_count += 1;
            }
        }
    }
    monster_count
}

fn is_monster_at_location(tile_line_values: &Vec<TileValue>, monster_line_values: &Vec<TileValue>, x_offset: usize, y_offset: usize) -> bool {
    monster_line_values.iter().enumerate().all(|(i, base_monster_value)| {
        let y = y_offset + i;
        let tile_value = tile_line_values[y];
        let monster_value = base_monster_value << x_offset;
        tile_value & monster_value == monster_value
    })
}