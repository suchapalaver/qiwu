#[derive(Debug, PartialEq)]
enum Move {
    Right,
    Down,
    Left,
    Up,
}

impl Movement {
    fn make_move(&mut self, rows: &usize, row_len: &usize) {
        match self.mov {
            Move::Right => {
                if self.position.y == row_len - 1 - self.iteration {
                    self.mov = Move::Down;
                    self.position.x += 1;
                } else {
                    self.position.y += 1;
                }
            }
            Move::Left => {
                if self.position.y == self.iteration {
                    self.mov = Move::Up;
                    self.position.x = self.position.y.saturating_sub(1);
                } else {
                    self.position.y = self.position.y.saturating_sub(1);
                }
            }
            Move::Down => {
                if self.position.x == rows - 1 - self.iteration {
                    self.mov = Move::Left;
                    self.position.y = self.position.y.saturating_sub(1);
                } else {
                    self.position.x += 1;
                }
            }
            Move::Up => {
                if self.position.x == self.iteration + 1 {
                    self.iteration += 1;
                    self.mov = Move::Right;
                    self.position.y += 1;
                } else {
                    self.position.x = self.position.x.saturating_sub(1);
                }
            }
        }
    }
}

pub struct Movement {
    position: Position,
    iteration: usize,
    mov: Move,
}

#[derive(Debug, Default)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            position: Position::default(),
            iteration: 0,
            mov: Move::Right,
        }
    }
}

/// [Spiral Matrix](https://leetcode.com/problems/spiral-matrix/)
pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut m = Movement::default();

        let rows = matrix.len();
        let row_len = matrix[0].len();
        let mut v = Vec::new();

        while v.len() < rows * row_len {
            v.push(matrix[m.position.x][m.position.y]);

            m.make_move(&rows, &row_len);
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use crate::math_and_geometry::spiral_matrix::Solution;

    #[test]
    fn matrix() {
        let x = Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        assert_eq!(x, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
        let x = Solution::spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ]);
        assert_eq!(x, vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
        let x = Solution::spiral_order(vec![vec![1], vec![2]]);
        assert_eq!(x, vec![1, 2]);
    }
}
