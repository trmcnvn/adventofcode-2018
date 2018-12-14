use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;

#[derive(Clone, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, PartialEq)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Clone, PartialEq)]
struct Cart {
    x: usize,
    y: usize,
    direction: Direction,
    next_turn: Turn,
}

impl Cart {
    fn new(x: usize, y: usize, direction: Direction, next_turn: Turn) -> Self {
        Self {
            x,
            y,
            direction,
            next_turn,
        }
    }
}

type TileMap = HashMap<usize, HashMap<usize, u8>>;
type CartVec = Vec<Cart>;

#[aoc_generator(day13)]
fn input_generator(_input: &str) -> (CartVec, TileMap) {
    let input = include_str!("../input/2018/day13.txt"); // cargo-aoc trims input
    let mut carts: CartVec = Vec::new();
    let mut tiles: TileMap = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, byte) in line.as_bytes().iter().enumerate() {
            let tile = match byte {
                b'^' => {
                    carts.push(Cart::new(x, y, Direction::North, Turn::Left));
                    b'|'
                }
                b'<' => {
                    carts.push(Cart::new(x, y, Direction::West, Turn::Left));
                    b'-'
                }
                b'v' => {
                    carts.push(Cart::new(x, y, Direction::South, Turn::Left));
                    b'|'
                }
                b'>' => {
                    carts.push(Cart::new(x, y, Direction::East, Turn::Left));
                    b'-'
                }
                b' ' => continue,
                _ => *byte,
            };
            let entry = tiles.entry(x).or_default();
            entry.insert(y, tile);
        }
    }
    (carts, tiles)
}

#[aoc(day13, part1)]
fn solve_part1((carts, tiles): &(CartVec, TileMap)) -> String {
    solve(carts, tiles, false)
}

#[aoc(day13, part2)]
fn solve_part2((carts, tiles): &(CartVec, TileMap)) -> String {
    solve(carts, tiles, true)
}

fn get_direction(direction: &Direction, byte: u8) -> Direction {
    match byte {
        b'/' => match direction {
            Direction::North => Direction::East,
            Direction::East => Direction::North,
            Direction::South => Direction::West,
            Direction::West => Direction::South,
        },
        b'\\' => match direction {
            Direction::North => Direction::West,
            Direction::East => Direction::South,
            Direction::South => Direction::East,
            Direction::West => Direction::North,
        },
        b'<' => match direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        },
        b'>' => match direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        },
        _ => unreachable!(),
    }
}

fn solve(carts: &[Cart], tiles: &TileMap, should_remove: bool) -> String {
    let mut carts = carts.to_owned();
    loop {
        carts.sort_by_key(|cart| cart.y);
        for (index, cart) in carts.clone().into_iter().enumerate() {
            if carts[index].x == std::usize::MAX {
                continue;
            }

            let (mut x, y) = match cart.direction {
                Direction::North => (cart.x, cart.y - 1),
                Direction::East => (cart.x + 1, cart.y),
                Direction::South => (cart.x, cart.y + 1),
                Direction::West => (cart.x - 1, cart.y),
            };

            let (direction, next_turn) = match tiles[&x][&y] {
                b'/' => (get_direction(&cart.direction, b'/'), cart.next_turn.clone()),
                b'\\' => (
                    get_direction(&cart.direction, b'\\'),
                    cart.next_turn.clone(),
                ),
                b'+' => match cart.next_turn {
                    Turn::Left => (get_direction(&cart.direction, b'<'), Turn::Straight),
                    Turn::Straight => (cart.direction.clone(), Turn::Right),
                    Turn::Right => (get_direction(&cart.direction, b'>'), Turn::Left),
                },
                _ => (cart.direction.clone(), cart.next_turn.clone()),
            };

            for (other_index, other_cart) in carts.iter_mut().enumerate() {
                if index != other_index && other_cart.x == x && other_cart.y == y {
                    if should_remove {
                        other_cart.x = std::usize::MAX;
                        x = std::usize::MAX;
                        break;
                    } else {
                        return format!("{},{}", x, y);
                    }
                }
            }

            carts[index] = Cart::new(x, y, direction, next_turn);
        }

        carts = carts
            .into_iter()
            .filter(|cart| cart.x != std::usize::MAX)
            .collect();

        if should_remove && carts.len() == 1 {
            return format!("{},{}", carts[0].x, carts[0].y);
        }
    }
}
