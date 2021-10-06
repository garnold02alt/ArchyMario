use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign},
};

#[derive(Clone, Copy, PartialEq)]
pub struct Vector<const N: usize> {
    pub inner: [f64; N],
}

impl<const N: usize> Vector<N> {
    pub fn from_array(array: [f64; N]) -> Self {
        Self { inner: array }
    }
    pub fn dot(self, b: Self) -> f64 {
        self.inner.iter().zip(b.inner).map(|(a, b)| a * b).sum()
    }
}

impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        Self { inner: [0.0; N] }
    }
}

impl<const N: usize> Debug for Vector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.inner.iter()).finish()
    }
}

impl<const N: usize> Index<usize> for Vector<N> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}
impl<const N: usize> IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}
impl<const N: usize> Mul for Vector<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut o = self;
        for (a, b) in o.inner.iter_mut().zip(rhs.inner) {
            *a *= b;
        }
        o
    }
}
impl<const N: usize> Add for Vector<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut o = self;
        for (a, b) in o.inner.iter_mut().zip(rhs.inner) {
            *a += b;
        }
        o
    }
}
impl<const N: usize> AddAssign for Vector<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
impl<const N: usize> MulAssign for Vector<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}
impl<const N: usize> Mul<f64> for Vector<N> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut o = self;
        for a in o.inner.iter_mut() {
            *a *= rhs;
        }
        o
    }
}
impl<const N: usize> Mul<Vector<N>> for f64 {
    type Output = Vector<N>;

    fn mul(self, rhs: Vector<N>) -> Self::Output {
        rhs * self
    }
}
impl<const N: usize> MulAssign<f64> for Vector<N> {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

pub type Vec3 = Vector<3>;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self::from_array([x, y, z])
    }
    pub fn x(self) -> f64 {
        self[0]
    }
    pub fn y(self) -> f64 {
        self[1]
    }
    pub fn z(self) -> f64 {
        self[2]
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Matrix<const N: usize, const M: usize> {
    pub inner: [Vector<N>; M],
}

impl<const N: usize> Matrix<N, N> {
    pub fn identity() -> Self {
        let mut o: Self = Default::default();
        for i in 0..N {
            o[i][i] = 1.0;
        }
        o
    }
}

impl<const N: usize, const M: usize> Matrix<N, M> {
    pub fn transpose(self) -> Matrix<M, N> {
        let mut o: Matrix<M, N> = Default::default();
        for x in 0..N {
            let vec = &mut o[x];
            for y in 0..M {
                vec[y] = self[y][x];
            }
        }
        o
    }

    pub fn from_vectors(vectors: [Vector<N>; M]) -> Self {
        Self { inner: vectors }
    }
}

impl<const N: usize, const M: usize> Default for Matrix<N, M> {
    fn default() -> Self {
        Self {
            inner: [Default::default(); M],
        }
    }
}

impl<const N: usize, const M: usize> Index<usize> for Matrix<N, M> {
    type Output = Vector<N>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<const N: usize, const M: usize> IndexMut<usize> for Matrix<N, M> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<const N: usize, const M: usize> Debug for Matrix<N, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.inner.iter()).finish()
    }
}

impl<const N: usize, const M: usize> Mul<Vector<M>> for Matrix<N, M> {
    type Output = Vector<N>;

    fn mul(self, rhs: Vector<M>) -> Self::Output {
        let mut o = Vector::from_array([0.0; N]);
        for (scalar, vector) in rhs.inner.iter().zip(self.inner) {
            o += *scalar * vector;
        }
        o
    }
}

impl<const N: usize, const M: usize, const K: usize> Mul<Matrix<N, M>> for Matrix<M, K> {
    type Output = Matrix<N, K>;

    fn mul(self, rhs: Matrix<N, M>) -> Self::Output {
        let mut o: Self::Output = Default::default();
        for (i, vector) in self.inner.iter().enumerate() {
            o[i] = rhs * (*vector);
        }
        o
    }
}

#[macro_export]
macro_rules! vector {
    ( $($x:expr),* ) => {
        $crate::utilities::math::Vector::from_array([$($x, )*])
    };
}
#[macro_export]
macro_rules! matrix {
    ( $($x:expr),* ) => {
        $crate::utilities::math::Matrix::from_vectors([$($x, )*])
    };
}
