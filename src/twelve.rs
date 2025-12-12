use std::{fs::File, io::Read};
use std::sync::LazyLock;

use itertools::Itertools;
use regex::Regex;
use colored::Colorize;
use colored::Color;

const S0: [u8; 9] = *b"#.#####.#";
const S1: [u8; 9] = *b"#####..##";
const S2: [u8; 9] = *b"###..####";
const S3: [u8; 9] = *b".##.#####";
const S4: [u8; 9] = *b"..#.####.";
const S5: [u8; 9] = *b"#####.#..";

const SHAPES: [[u8; 9]; 6] = [
    S0, S1, S2, S3, S4, S5
];

// const COLORS: [Color; 14] = [
//     Color::Red,
//     Color::Green,
//     Color::Yellow,
//     Color::Blue,
//     Color::Magenta,
//     Color::Cyan,
//     Color::White,
//
//     Color::BrightRed,
//     Color::BrightGreen,
//     Color::BrightYellow,
//     Color::BrightBlue,
//     Color::BrightMagenta,
//     Color::BrightCyan,
//     Color::BrightWhite,
// ];

static COUNTS: LazyLock<[usize; 6]> = LazyLock::new(|| SHAPES.map(|s| bytecount::count(&s, b'#')));

#[derive(Copy, Clone)]
struct ColoredByte {
    inner: u8,
    color: Color,
}

impl ColoredByte {
    fn print(&self) {
        print!("{}", str::from_utf8(std::slice::from_ref(&self.inner)).unwrap().color(self.color));
    }
}

struct Board {
    width: usize,
    height: usize,
    grid: Box<[ColoredByte]>
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum GetResult {
    Empty,
    Full,
    OutOfBounds
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        let grid = vec![ColoredByte { inner: b'.', color: Color::BrightWhite }; width * height].into_boxed_slice();

        Self {
            width, height, grid
        }
    }

    fn get(&self, x: usize, y: usize) -> GetResult {
        if !(0..self.width).contains(&x) || !(0..self.height).contains(&y) {
            GetResult::OutOfBounds
        } 
        else {
            let byte = self.grid[x + y * self.width];
            if byte.inner == b'.' {
                GetResult::Empty
            } 
            else {
                GetResult::Full
            }
        }
    }

    fn set(&mut self, new_byte: u8, color: Color, x: usize, y: usize) {
        if !(0..self.width).contains(&x) || !(0..self.height).contains(&y) {
            panic!("Board::set out of bounds");
        } 
        else {
            let byte = self.grid[x + y * self.width];
            if byte.inner == b'.' {
                self.grid[x + y * self.width] = ColoredByte { inner: new_byte, color };
            } 
            else {
                panic!("Board::set on occupied space");
            }
        }
    }

    fn can_insert_at(&self, pc: &PackingCost, x: usize, y: usize) -> bool {
        for x in x..x+pc.dims.0 {
            for y in y..y+pc.dims.1 {
                if self.get(x, y) != GetResult::Empty {
                    return false;
                }
            }
        }

        true
    }

    fn insert_at(&mut self, pc: &PackingCost, color: Color, x: usize, y: usize) {
        // dbg!(pc);
        for _y in 0..pc.dims.1 {
            for _x in 0..pc.dims.0 {
                // self.set(pc.debug[dbg!(_x + _y * pc.dims.0)], x + _x, y + _y);
                self.set(pc.debug[_x + _y * pc.dims.0], color, x + _x, y + _y);
            }
        }
    }

    fn attempt_insert(&mut self, pc: &PackingCost, color: Color) -> bool {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.can_insert_at(pc, x, y) {
                    self.insert_at(pc, color, x, y);
                    return true;
                }
            }
        }

        false
    }

    fn assert_correct_counts(&self, shape_counts: &[i32; 6]) {
        for (idx, shape_count) in shape_counts.iter().enumerate() {
            let count = self.grid.iter()
                .filter(|b| b.inner == b'0' + idx as u8)
                .count();
            assert_eq!(
                count, *shape_count as usize * COUNTS[idx],
                "testing count of {idx}"
            );
        }
    }

    fn print(&self) {
        for y in 0..self.height {
            let idx = y * self.width;
            let row = &self.grid[idx..idx + self.width];
            for cb in row { cb.print(); }
            println!();
        }
    }
}

#[derive(Debug)]
struct PackingCost {
    indices: (usize, usize),
    cost: usize,
    dims: (usize, usize),
    debug: Box<[u8]>
}

impl PackingCost {
    fn print_debug(&self) {
        assert!(self.dims.0 * self.dims.1 == self.debug.len());
        for y in 0..self.dims.1 {
            let idx = y * self.dims.0;
            println!("{}", str::from_utf8(&self.debug[idx..idx + self.dims.0]).unwrap());
        } 
    }
}

#[derive(Copy, Clone)]
struct Shape {
    data: [u8; 9],
    id: u8
}

impl Shape {
    const fn new(data: [u8; 9], id: u8) -> Self {
        Self { data, id }
    }

    const fn rotated(&self) -> Self {
        let data = [
            self.data[2], self.data[5], self.data[8],
            self.data[1], self.data[4], self.data[7],
            self.data[0], self.data[3], self.data[6],
        ];

        Self { data, id: self.id }
    }

    fn get(&self, x: i32, y: i32) -> bool {
        if !(0..3).contains(&x) || !(0..3).contains(&y) {
            false
        } else {
            self.data[y as usize * 3 + x as usize] == b'#'
        }
    }

    fn print(&self) {
        println!("{}", str::from_utf8(&self.data[0..3]).unwrap());
        println!("{}", str::from_utf8(&self.data[3..6]).unwrap());
        println!("{}", str::from_utf8(&self.data[6..9]).unwrap());
    }

    fn overlaps(&self, other: &Shape, relative_pos: (i32, i32)) -> bool {
        // dbg!(self.id - b'0', other.id - b'0', relative_pos);
        for x in 0..3 {
            for y in 0..3 {
                if self.get(x, y) && other.get(x - relative_pos.0, y - relative_pos.1) {
                    return true;
                }
            }
        }

        false
    }

    fn generate_debug(&self, other: &Shape, mut relative_pos: (i32, i32)) -> Box<[u8]> {
        let dims = ((relative_pos.0.abs() + 3) as usize, (relative_pos.1.abs() + 3) as usize);
        let mut ret = vec![b'_'; dims.0 * dims.1].into_boxed_slice();
        
        let _other = if relative_pos.0 < 0 || relative_pos.1 < 0 {
            self
        } else {
            other
        };
        let origin = if relative_pos.0 < 0 || relative_pos.1 < 0 {
            relative_pos.0 = relative_pos.0.abs();
            relative_pos.1 = relative_pos.1.abs();

            other
        } else {
            self
        };

        for x in 0..dims.0 {
            for y in 0..dims.1 {
                let idx = y * dims.0 + x;

                if origin.get(x as i32, y as i32) {
                    ret[idx] = origin.id;
                }
                else if _other.get(x as i32 - relative_pos.0, y as i32 - relative_pos.1) {
                    ret[idx] = _other.id;
                }
            }
        }

        // dbg!(self.id, other.id, dims, &ret);
        ret
    }

    fn find_packing_area(&self, other: &Shape) -> (usize, (usize, usize), Box<[u8]>) {
        let shapes = [
            *other,
            other.rotated(),
            other.rotated().rotated(),
            other.rotated().rotated().rotated()
        ];

        let mut dims = (0, 0);
        let mut debug = Box::default();
        let mut ret = usize::MAX;
        for x in -3..=3 {
            for y in -3..=3 {
                for shape in &shapes {
                    if !self.overlaps(shape, (x, y)) {
                        let area = ((x.abs() + 3) * (y.abs() + 3)) as usize;
                        if area < ret {
                            ret = area;
                            dims = ((x.abs() + 3) as usize, (y.abs() + 3) as usize);
                            // dbg!(ret, dims, (x, y));
                            debug = self.generate_debug(shape, (x, y))
                        }
                    }
                }
            }
        }

        (ret, dims, debug)
    }
}

/// # Panics
/// This function panics on file read error.
pub fn solution(f: &mut File, part_two: bool) {
    let shapes = SHAPES.into_iter().enumerate().map(|(idx, data)| Shape::new(data, b'0' + idx as u8)).collect_vec();

    let mut packing_costs = Vec::with_capacity(shapes.len().pow(2));

    for ((idx, shape), (odx, other)) in shapes.iter().enumerate().cartesian_product(shapes.iter().enumerate()) {
        let (cost, dims, debug) = shape.find_packing_area(other);
        let pc = PackingCost {
            indices: (idx, odx),
            cost, dims, debug
        };

        if idx == odx {
            assert_eq!(
                bytecount::count(&pc.debug, shape.id), COUNTS[idx] * 2,
                "double checking PackingCost count for {idx}"
            );
        }
        else {
            assert_eq!(
                bytecount::count(&pc.debug, shape.id), COUNTS[idx],
                "double checking PackingCost count for {idx}"
            );

            assert_eq!(
                bytecount::count(&pc.debug, other.id), COUNTS[odx],
                "double checking PackingCost count for {odx}"
            );
        }

        // if pc.cost != 18 {
        //     dbg!(idx, odx);
        //     pc.print_debug();
        //     let _ = std::io::stdin().lines().next();
        // }

        packing_costs.push(pc);
        // println!("{idx} {odx} {}", shape.find_packing_area(other));
    }

    packing_costs.sort_by_key(|p| p.cost);

    let mut ret = 0;
    let re = Regex::new(r"\d+").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let mut empty_lines = 0;
    for line in buf.lines() {
        if line.is_empty() {
            empty_lines += 1;
            continue;
        }

        if empty_lines != 6 {
            continue;
        }

        let v = re.find_iter(line).map(|s| s.as_str().parse::<i32>()).collect::<Result<Vec<_>, _>>().unwrap();
        // let mut dims_left = (v[0], v[1]);
        // let mut area_left = v[0..2].iter().copied().product::<i32>();
        let mut shapes_left = v[2..].iter().copied().collect_array::<6>().unwrap();
        let shapes_left_copied = shapes_left;
        // dbg!(line, &shapes_left);

        let mut board = Board::new(v[0] as usize, v[1] as usize);
        let non_zero = |x: &i32| *x != 0;
        let mut successful = true;

        while shapes_left.iter().any(non_zero) {
            let color = Color::TrueColor { 
                r: fastrand::u8(100..255), 
                g: fastrand::u8(100..255), 
                b: fastrand::u8(100..255), 
            };

            board.assert_correct_counts(
                &shapes_left_copied.iter().zip(shapes_left.iter())
                    .map(|(&copy, &left)| copy - left)
                    .collect_array::<6>().unwrap()
            );

            if part_two {
                println!("{}", shapes_left.iter().map(i32::to_string).join(" "));
                board.print();
                let _ = std::io::stdin().lines().next();
            }

            if shapes_left.iter().copied().filter(non_zero).count() == 1 {
                let last_shape = shapes_left.iter().copied().find_position(non_zero).unwrap().0;
                let byte = b'0' + last_shape as u8;
                let single_pc = PackingCost {
                    indices: (0, 0), // WARN: uninitialized!
                    cost: 9,
                    dims: (3, 3),
                    debug: Box::new(SHAPES[last_shape].map(|b| 
                        if b == b'.' { b'_' }
                        else if b == b'#' { byte }
                        else { unreachable!() }
                    ))
                };

                successful = board.attempt_insert(&single_pc, color);
                if successful {
                    shapes_left[last_shape] -= 1;
                } else {
                    break;
                }
            } else {
                'label: {
                    for p in &packing_costs {
                        if p.indices.0 == p.indices.1 {
                            if shapes_left[p.indices.0] > 1 && board.attempt_insert(p, color) {
                                shapes_left[p.indices.0] -= 2;
                                break 'label;
                            }
                        }

                        else if shapes_left[p.indices.0] > 0 && shapes_left[p.indices.1] > 0 && board.attempt_insert(p, color) {
                            shapes_left[p.indices.0] -= 1;
                            shapes_left[p.indices.1] -= 1;
                            break 'label;
                        }
                    }

                    successful = false;
                }

                if !successful { break; }
            }
        }


        if part_two {
            println!("{}", shapes_left.iter().map(i32::to_string).join(" "));
            board.print();
            dbg!(successful);
            let _ = std::io::stdin().lines().next();
        }

        if successful {
            board.assert_correct_counts(
                &shapes_left_copied.iter().zip(shapes_left.iter())
                    .map(|(&copy, &left)| copy - left)
                    .collect_array::<6>().unwrap()
            );

            // let counts = [
            //     7, 7, 7, 7, 5, 6
            // ];
            // for (idx, count) in shapes_left_copied.iter().enumerate() {
            //     let bytes_counted = board.grid.iter().filter(|&&b| b == b'0' + idx as u8).count();
            //     dbg!(idx, count, count * counts[idx], bytes_counted);
            //     assert_eq!(
            //         bytes_counted, (*count * counts[idx]) as usize,
            //         "testing count of {idx}"
            //     );
            // }

            ret += 1;
        }

        // board.print();
        // dbg!(successful);
        // let _ = std::io::stdin().lines().next();

    }

    dbg!(ret);
}
