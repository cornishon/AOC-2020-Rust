use std::{mem, ops::Range};

use itertools::iproduct;

const CYCLES: usize = 6;
const WIDTH: usize = 8;
const HEIGHT: usize = 8;
const INITIAL_STATE: &str = include_str!("input.txt");
const W: usize = WIDTH + 2 * CYCLES + 2;
const H: usize = HEIGHT + 2 * CYCLES + 2;
const D: usize = 1 + 2 * CYCLES + 2;

fn main() {
    simulate3d();
    simulate4d();
}

fn simulate3d() {
    let mut buf1 = [[[false; W]; H]; D];
    let mut buf2 = [[[false; W]; H]; D];
    INITIAL_STATE.lines().enumerate().for_each(|(i, line)| {
        line.bytes().enumerate().for_each(|(j, byte)| {
            buf1[CYCLES + 1][j + CYCLES + 1][i + CYCLES + 1] = match byte {
                b'.' => false,
                b'#' => true,
                _ => unreachable!(),
            }
        })
    });

    for cycle in 0..CYCLES {
        for k in range(cycle, HEIGHT) {
            for j in range(cycle, WIDTH) {
                for i in range(cycle, 1) {
                    let n = count_nbors3d(buf1, i, j, k);
                    buf2[i][j][k] = matches!((buf1[i][j][k], n), (true, 2 | 3) | (false, 3));
                }
            }
        }
        mem::swap(&mut buf1, &mut buf2);
    }

    let n_active = buf1.iter().flatten().flatten().filter(|x| **x).count();
    println!("{n_active}");
}

fn count_nbors3d(space: [[[bool; W]; H]; D], i: usize, j: usize, k: usize) -> usize {
    let cell_is_active = if space[i][j][k] { 1 } else { 0 };

    let active_nbors = iproduct!(i - 1..=i + 1, j - 1..=j + 1, k - 1..=k + 1)
        .filter(|(x, y, z)| space[*x][*y][*z])
        .count();

    if active_nbors == 0 {
        return active_nbors;
    }
    active_nbors - cell_is_active
}

fn simulate4d() {
    let mut buf1 = [[[[false; W]; H]; D]; D];
    let mut buf2 = [[[[false; W]; H]; D]; D];
    INITIAL_STATE.lines().enumerate().for_each(|(i, line)| {
        line.bytes().enumerate().for_each(|(j, byte)| {
            buf1[CYCLES + 1][CYCLES + 1][j + CYCLES + 1][i + CYCLES + 1] = match byte {
                b'.' => false,
                b'#' => true,
                _ => unreachable!(),
            }
        })
    });

    for cycle in 0..CYCLES {
        for l in range(cycle, HEIGHT) {
            for k in range(cycle, WIDTH) {
                for j in range(cycle, 1) {
                    for i in range(cycle, 1) {
                        let n = count_nbors4d(buf1, i, j, k, l);
                        buf2[i][j][k][l] =
                            matches!((buf1[i][j][k][l], n), (true, 2 | 3) | (false, 3));
                    }
                }
            }
        }
        mem::swap(&mut buf1, &mut buf2);
    }

    let n_active = buf1
        .iter()
        .flatten()
        .flatten()
        .flatten()
        .filter(|x| **x)
        .count();
    println!("{n_active}");
}

fn count_nbors4d(space: [[[[bool; W]; H]; D]; D], i: usize, j: usize, k: usize, l: usize) -> usize {
    let cell_is_active = if space[i][j][k][l] { 1 } else { 0 };

    let active_nbors = iproduct!(i - 1..=i + 1, j - 1..=j + 1, k - 1..=k + 1, l - 1..=l + 1)
        .filter(|(x, y, z, w)| space[*x][*y][*z][*w])
        .count();

    if active_nbors == 0 {
        return active_nbors;
    }
    active_nbors - cell_is_active
}

fn range(cycle: usize, size: usize) -> Range<usize> {
    CYCLES - cycle..CYCLES + cycle + size + 2
}
