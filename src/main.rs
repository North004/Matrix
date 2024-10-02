use core::fmt;
use std::{
    f64::consts::PI,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
};

// Number Field
trait Numeric:
    Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Default
{
    fn one() -> Self;
}

macro_rules! impl_numeric {
    ($($t:ty),+) => {
        $(impl Numeric for $t {
            fn one() -> Self {
                1 as $t
            }
        })+
    };
}

impl_numeric!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

#[derive(Clone)]
struct Matrix<T: Numeric> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Numeric> Matrix<T> {
    fn new(rows: usize, cols: usize) -> Matrix<T> {
        Matrix {
            data: vec![T::default(); rows * cols],
            rows,
            cols,
        }
    }
    fn multiplication(&self, matrix: &Matrix<T>) -> Matrix<T> {
        assert_eq!(self.cols, matrix.rows);
        let mut result: Matrix<T> = Matrix::new(self.rows, matrix.cols);
        for i in 0..self.rows {
            for j in 0..matrix.cols {
                for k in 0..self.cols {
                    result[i][j] = result[i][j] + self[i][k] * matrix[k][j];
                }
            }
        }
        result
    }
    fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
    fn addition(&self, matrix: &Matrix<T>) -> Matrix<T> {
        assert_eq!((self.rows, self.cols), (matrix.rows, matrix.cols));
        let mut result: Matrix<T> = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[i][j] = self[i][j] + matrix[i][j];
            }
        }
        result
    }
    fn identity(order: usize) -> Matrix<T> {
        let mut data: Matrix<T> = Matrix::new(order, order);
        for rc in 0..order {
            data[rc][rc] = T::one();
        }
        data
    }
    #[allow(dead_code)]
    fn transpose(&self) -> Matrix<T> {
        let mut result: Matrix<T> = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[j][i] = self[i][j];
            }
        }
        result
    }

    fn inverse(&self) -> Matrix<T> {
        todo!()
    }
}

impl<T: Numeric> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.cols;
        let end = start + self.cols;
        &self.data[start..end]
    }
}
//art
//  -------------------------------------------------------
//  |  11101101 00110101 01111010 10110011   |   01101010 |
//  |  11001010 10100111 10101110 11110101   |   01010101 |
//  |  01010101 11010010 10101010 10101011   |   01010100 |
//  |  11010101 11111110 01101010 00000111   |   01010011 |
//  -------------------------------------------------------
impl<T: Numeric> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.cols;
        let end = start + self.cols;
        &mut self.data[start..end]
    }
}

impl<T: Numeric> Mul for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, other: Matrix<T>) -> Matrix<T> {
        self.multiplication(&other)
    }
}

impl<T: Numeric> Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, other: Matrix<T>) -> Matrix<T> {
        self.addition(&other)
    }
}

impl<T: fmt::Debug + Numeric> fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[")?;
        for i in 0..self.rows {
            f.write_str("[")?;
            for j in 0..self.cols {
                if j > 0 {
                    f.write_str(", ")?;
                }
                write!(f, "{:?}", self.data[i * self.cols + j])?;
            }
            f.write_str("]")?;
            if i < self.rows - 1 {
                f.write_str(", ")?;
            }
        }
        f.write_str("]")?;
        Ok(())
    }
}

macro_rules! matrix {
    ($($row:expr),*) => {{
        let rows = vec![$(Vec::from($row)),*];
        Matrix {
            rows: rows.len(),
            cols: rows[0].len(),
            data: rows.into_iter().flatten().collect(),
        }
    }};
}

#[allow(unused)]
fn main() {
    let transformation: Matrix<f64> = matrix![[PI.cos(), -(PI.sin())], [PI.sin(), PI.cos()]];
    let vector: Matrix<f64> = matrix![[4.0], [2.0]];
    let transformed_vector = transformation.multiplication(&vector);
    println!("{:?}", transformed_vector);
    let identity_matrix: Matrix<i32> = Matrix::identity(3);
    println!("{:?}", identity_matrix.size());
    println!("{:?}", identity_matrix);
    let b: Matrix<f64> = transformed_vector.transpose();
    println!("{:?}", b);
}
