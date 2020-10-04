use std::ops::{Mul, Index, Neg};
use crate::print_vec;


pub struct Matrix {
    pub values: Vec<f64>,
    pub rows: usize,
    pub columns: usize,
}

impl Matrix {
    pub fn determinant(&self) -> f64 {
        assert_eq!(self.rows, self.columns);
        return if self.columns == 1 {
            self[(0, 0)]
        } else if self.columns == 2 {
            self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
        } else {
            let mut sign = 1f64;
            let mut determinant = 0f64;
            for i in 0..self.columns {
                determinant += sign * self[(i, 0)] * self.get_minor(i, 0).determinant();
                sign *= -1f64;
            }
            determinant
        };
    }


    pub fn transpose(&self) -> Matrix {
        let mut transposed = Matrix::zeros(self.rows, self.columns);
        for i in 0..self.rows {
            for j in 0..self.columns {
                transposed[(i, j)] = self[(j, i)];
            }
        }
        return transposed;
    }

    pub fn get_minor(&self, column: usize, row: usize) -> Matrix {
        let mut sub: Vec<f64> = Vec::with_capacity(self.values.len() - self.rows - self.columns + 1);
        for i in 0..self.columns {
            for j in 0..self.rows {
                if i != column && j != row {
                    sub.push(self[(i, j)]);
                }
            }
        }
        Matrix {
            values: sub,
            rows: self.rows - 1,
            columns: self.columns - 1,
        }
    }

    pub fn sub_matrix(&self, start: [usize; 2], end: [usize; 2]) -> Matrix {
        let columns = end[0] - start[0];
        let rows = end[1] - start[1];
        let mut sub_matrix = Matrix {
            values: Vec::with_capacity(columns * rows),
            rows,
            columns,
        };
        for i in start[0]..end[0] {
            for j in start[1]..end[1] {
                sub_matrix.values.push(self[(i, j)]);
            }
        }
        return sub_matrix;
    }

    pub fn matrix_of_minors(&self) -> (Matrix, f64) {
        assert_eq!(self.rows, self.columns);
        return if self.columns == 1 {
            (self.sub_matrix([0, 0], [1, 1]), self[(0, 0)])
        } else {
            let mut minors = Matrix {
                values: Vec::with_capacity(self.columns * self.rows),
                columns: self.columns,
                rows: self.rows,
            };
            let mut determinant = 0f64;
            let mut sign = 1f64;
            let mut cur_det;
            for i in 0..self.columns {
                cur_det = self.get_minor(i, 0).determinant();
                determinant += sign * self[(i, 0)] * cur_det;
                sign *= -1f64;
                minors.values.push(cur_det);
                for j in 1..self.rows {
                    cur_det = self.get_minor(i, j).determinant();
                    minors.values.push(cur_det);
                }
            }
            (minors, determinant)
        };
    }

    pub fn invert(&self) -> Matrix {
        assert_eq!(self.rows, self.columns);
        let (mut minors, determinant) = self.matrix_of_minors();
        minors.hadamard_product_with(Matrix::checker_board(self.columns, self.rows));
        return minors.transpose() * (1f64 / determinant);
    }

    pub fn hadamard_product_with(&mut self, m0: Matrix) {
        assert_eq!(self.rows, m0.rows);
        assert_eq!(self.columns, m0.columns);
        for i in 0..m0.values.len() {
            self.values[i] *= m0.values[i];
        }
    }


    pub fn hadamard_product(m0: Matrix, m1: Matrix) {
        assert_eq!(m0.rows, m1.rows);
        assert_eq!(m0.columns, m1.columns);
        let mut out = Matrix {
            values: Vec::with_capacity(m0.rows * m0.columns),
            rows: m0.rows,
            columns: m0.columns,
        };
        for i in 0..m0.values.len() {
            out.values.push(m0.values[i] * m1.values[i]);
        }
    }

    pub fn zeros(columns: usize, rows: usize) -> Matrix {
        Matrix {
            values: vec!(0f64; rows * columns),
            rows,
            columns,
        }
    }

    pub fn fill(columns: usize, rows: usize, value: f64) -> Matrix {
        Matrix {
            values: vec!(value; rows * columns),
            rows,
            columns,
        }
    }

    pub fn checker_board(columns: usize, rows: usize) -> Matrix {
        let mut checker_board = Matrix {
            values: Vec::with_capacity(rows * columns),
            rows,
            columns,
        };
        let mut start_sign = 1f64;
        for _i in 0..columns {
            let mut sign = start_sign;
            for _j in 0..rows {
                checker_board.values.push(sign);
                sign *= -1f64;
            }
            start_sign *= -1f64;
        }
        return checker_board;
    }
}

impl std::ops::Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        return &self.values[index.0 + index.1 * self.columns];
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        return &mut self.values[index.0 + index.1 * self.columns];
    }
}

impl std::ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        assert_eq!(self.columns, rhs.rows);
        let mut matrix = Matrix {
            values: Vec::with_capacity(self.rows * rhs.columns),
            columns: self.rows,
            rows: rhs.columns,
        };
        for i in 0..rhs.columns {
            for j in 0..self.rows {
                let mut val = 0f64;
                for k in 0..self.columns {
                    val += self[(k, j)] * rhs[(i, k)];
                }
                matrix.values.push(val);
            }
        }
        return matrix;
    }
}

impl std::ops::Mul<&Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        assert_eq!(self.columns, rhs.rows);
        let mut matrix = Matrix {
            values: Vec::with_capacity(self.rows * rhs.columns),
            columns: self.rows,
            rows: rhs.columns,
        };
        for i in 0..rhs.columns {
            for j in 0..self.rows {
                let mut val = 0f64;
                for k in 0..self.columns {
                    val += self[(k, j)] * rhs[(i, k)];
                }
                matrix.values.push(val);
            }
        }
        return matrix;
    }
}

impl std::ops::Mul<Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        assert_eq!(self.columns, rhs.rows);
        let mut matrix = Matrix {
            values: Vec::with_capacity(self.rows * rhs.columns),
            columns: self.rows,
            rows: rhs.columns,
        };
        for i in 0..rhs.columns {
            for j in 0..self.rows {
                let mut val = 0f64;
                for k in 0..self.columns {
                    val += self[(k, j)] * rhs[(i, k)];
                }
                matrix.values.push(val);
            }
        }
        return matrix;
    }
}

impl std::ops::MulAssign<f64> for Matrix {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..self.columns {
            for j in 0..self.rows {
                self[(i, j)] *= rhs;
            }
        }
    }
}

impl std::ops::Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut matrix = Matrix {
            values: Vec::with_capacity(self.columns * self.rows),
            rows: self.rows,
            columns: self.columns,
        };
        for i in 0..self.columns {
            for j in 0..self.rows {
                matrix.values.push(self[(i, j)] * rhs);
            }
        }
        return matrix;
    }
}

impl std::ops::AddAssign<f64> for Matrix {
    fn add_assign(&mut self, rhs: f64) {
        for i in 0..self.columns {
            for j in 0..self.rows {
                self[(i, j)] += rhs;
            }
        }
    }
}

impl std::ops::Add<f64> for Matrix {
    type Output = Matrix;
    fn add(self, rhs: f64) -> Self::Output {
        let mut matrix = Matrix {
            values: Vec::with_capacity(self.columns * self.rows),
            rows: self.rows,
            columns: self.columns,
        };
        for i in 0..self.rows {
            for j in 0..self.columns {
                matrix[(i, j)] = self[(i, j)] + rhs
            }
        }
        return matrix;
    }
}

impl std::ops::SubAssign<f64> for Matrix {
    fn sub_assign(&mut self, rhs: f64) {
        for i in 0..self.columns {
            for j in 0..self.rows {
                self[(i, j)] -= rhs;
            }
        }
    }
}

impl std::ops::Sub<f64> for Matrix {
    type Output = Matrix;
    fn sub(self, rhs: f64) -> Self::Output {
        let mut matrix = Matrix {
            values: Vec::with_capacity(self.columns * self.rows),
            rows: self.rows,
            columns: self.columns,
        };
        for i in 0..self.rows {
            for j in 0..self.columns {
                matrix[(i, j)] = self[(i, j)] - rhs
            }
        }
        return matrix;
    }
}

impl std::ops::Neg for Matrix {
    type Output = Matrix;

    fn neg(self) -> Self::Output {
        let mut out = Matrix {
            values: Vec::with_capacity(self.values.len()),
            rows: self.rows,
            columns: self.columns,
        };
        for i in self.values {
            out.values.push(i);
        }
        return out;
    }
}