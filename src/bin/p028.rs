fn main() {
    println!("{}", sum_diagonals(generate_spiral(1001)));
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn walk(self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        Some(match self {
            Self::Right => (x + 1, y),
            Self::Down => (x, y + 1),
            Self::Left if x > 0 => (x - 1, y),
            Self::Up if y > 0 => (x, y - 1),
            _ => return None,
        })
    }

    fn left(self) -> Direction {
        match self {
            Self::Right => Self::Up,
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
        }
    }
}

fn generate_spiral(size: usize) -> Vec<Vec<u32>> {
    let max = size as u32 * size as u32 + 1;
    let mut matrix = vec![vec![0; size]; size];

    let (mut x, mut y) = (size - 1, 0); // Start in the top right
    let mut direction = Direction::Left;

    for value in (1..max).rev() {
        matrix[y][x] = value;

        if (x == 0 && direction == Direction::Left)
            || (y == size - 1 && direction == Direction::Down)
            || (x == size - 1 && direction == Direction::Right)
            || (y == 0 && direction == Direction::Up)
        {
            direction = direction.left();
        }

        let (mut nx, mut ny) = match direction.walk((x, y)) {
            None => unreachable!("a direction was not caught!"),
            Some(new) => new,
        };
        if matrix[ny][nx] != 0 {
            direction = direction.left();
            let (nnx, nny) = match direction.walk((x, y)) {
                None => unreachable!("a direction was not caught!"),
                Some(new) => new,
            };
            nx = nnx;
            ny = nny;
        }

        x = nx;
        y = ny;
    }

    matrix
}

fn sum_diagonals(matrix: Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;

    for y in 0..matrix.len() {
        let row = &matrix[y];
        let x = row.len() - 1 - y;
        sum += row[y];
        if x != y {
            sum += row[x];
        }
    }

    sum
}

#[test]
fn test() {
    assert_eq!(
        generate_spiral(5),
        "21 22 23 24 25
20  7  8  9 10
19  6  1  2 11
18  5  4  3 12
17 16 15 14 13"
            .lines()
            .map(|l| l.split_whitespace().flat_map(str::parse).collect())
            .collect::<Vec<Vec<u32>>>(),
    );
    assert_eq!(101, sum_diagonals(generate_spiral(5)));
}
