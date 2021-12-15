use crate::{impl_ops_all, tensor::Tensor};
use std::ops::{Add, Mul, Sub};

// 0  1  2
// 3  4  5
// 6  7  8
// 9 10 11
// Matrix Warpper for Tensor
#[derive(Debug, PartialEq, Clone)]
struct Matrix<T>
where
    T: Copy,
{
<<<<<<< HEAD
=======
>>>>>>> b7d17ceff0223c3f4f84a4e439c515bb38842a63
data: Tensor<T>,
    // 0 直排長度 1 橫排長度
    shape: [usize; 2],
}





impl<T> Matrix<T>
where
    T: Copy,
{
    pub fn new(value: T, shape: [usize; 2]) -> Self {
        Matrix {
            data: Tensor::new(value, shape.iter().product()),
            shape,
        }
    }

// transpose Matrix // 把Matrix轉掉
// 1 2 3     1 4 
// 4 5 6 ->  2 5 
//           3 6


    pub fn t(self) -> Transpose<T> {
        Transpose(self)
    }

    // iter by row
    // it is non-consum iterator
    // faster than col iter
    // 0  1  2
    // 3  4  5
    // 6  7  8
    // 9 10 11
    pub fn into_row_iter<'a>(&'a self) -> impl Iterator<Item = &T> + 'a {
        self.data.iter()
    }

    // 從橫排掃過一遍


    // iter by col
    // it is non-consum iterator
    // 0  4  8
    // 1  5  9
    // 2  6 10
    // 3  7 11
    pub fn into_col_iter<'a>(&'a self) -> impl Iterator<Item = &T> + 'a {
        // 第1直排到第N直排的迭代器
        (0..self.shape[1]) 
            // flat_map 用來攤平 ex: [[1,2],[3,4],[5,6]] -> [1,2,3,4,5,6]
            // step_by(self.shape[1]) 跳直的 , take()用來取迭代器，相對的原本迭代器裡的那個iter會被刪掉 , b的值會來自前面的(0..self.shape[1]) , 也就是 第 0,1,2項的迭代器,是flat_map自己抓的
            .flat_map(move |b| (b..).step_by(self.shape[1]).take(self.shape[0]))   
            // map用來取迭代器的值 , map會自己抓上面flat_map跳到的位置n
            .map(move |n| self.data.iter().nth(n).unwrap()) // 
    }
}

<<<<<<< HEAD
impl<T> From<(Tensor<T>, [usize; 2])> for Matrix<T>
where
    T: Copy,
{
    fn from(data: (Tensor<T>, [usize; 2])) -> Self {
        Matrix {
            data: data.0,
            shape: data.1,
        }
    }
}
=======


>>>>>>> b7d17ceff0223c3f4f84a4e439c515bb38842a63

impl<T> AsMut<Vec<T>> for Matrix<T>
where
    T: Copy,
{
    fn as_mut(&mut self) -> &mut Vec<T> {
        self.data.as_mut()
    }
}
// 使data可變
impl<T> AsRef<Vec<T>> for Matrix<T>
where
    T: Copy,
{
    fn as_ref(&self) -> &Vec<T> {
        self.data.as_ref()
    }
}
//使&a與a(借用或不借用)都可以作為參數

// Add Matrix
impl_ops_all!(+[<K, T> where T: Add<K> + Copy,K: Copy,<T as Add<K>>::Output: Copy]
    (left:Matrix<T>, right:Matrix<K>)->Matrix<<T as Add<K>>::Output> {
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(left.shape, right.shape)
        }
        Matrix {
            shape: left.shape.clone(),
            // 這裡可以直接+，來源自macros.rs
            data: &left.data + &right.data, 
        }
    }
);


// Add Matrix & Transpose
impl_ops_all!(+[<K, T> where T: Add<K> + Copy,K: Copy,<T as Add<K>>::Output: Copy]
    (left:Matrix<T>, right:Transpose<K>)->Matrix<<T as Add<K>>::Output> {
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(left.shape, [right.0.shape[1],right.0.shape[0]])
        }
        Matrix {
            shape: left.shape.clone(),
            data: Tensor::from(left.into_row_iter().zip(right.into_row_iter()).map(|(&a,&b)| a + b).collect::<Vec<_>>()),
        }
    }
);

// Sub Matrix
impl_ops_all!(-[<K, T> where T: Sub<K> + Copy,K: Copy,<T as Sub<K>>::Output: Copy]
    (left:Matrix<T>, right:Matrix<K>)->Matrix<<T as Sub<K>>::Output> {
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(left.shape, right.shape)
        }
        Matrix {
            shape: left.shape.clone(),
            data: &left.data - &right.data,
        }
    }
);

// Add Matrix & Transpose
// impl<K, T> Add<Transpose<K>> for Matrix<T>
// where
//     T: Add<K> + Copy,
//     K: Copy,
//     <T as Add<K>>::Output: Copy,
// {
//     type Output = Matrix<<T as Add<K>>::Output>;
//
//     fn add(self, rhs: Transpose<K>) -> Self::Output {
//         if cfg!(debug_assertions) {
//             assert_eq!(self.shape, [rhs.0.shape[1],rhs.0.shape[0]])
//         }
//         Matrix {
//             shape: rhs.shape.clone(),
//             data: &self.data + &rhs.0.data,
//         }
//     }
// }

// Mul Matrix
impl_ops_all!(*[<K,T> where T: Mul<K> + Copy,K: Copy,<T as Mul<K>>::Output: Copy + Default + Add<Output = <T as Mul<K>>::Output>]
    (left:Matrix<T>,right:Matrix<K>)->Matrix<<T as Mul<K>>::Output>{
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(left.shape[1], right.shape[0])
        }
        let mut data = Vec::new();
        for r in (0..).step_by(left.shape[1]).take(left.shape[0]) {
            for c in (0..).step_by(right.shape[0]).take(right.shape[1]) {
                data.push(
                    left.into_row_iter()
                        .skip(r)
                        .take(left.shape[1])
                        .zip(right.into_col_iter().skip(c))
                        .fold(<T as Mul<K>>::Output::default(), |v, (&a, &b)| v + a * b),
                );
            }
        }
        Matrix {
            shape: [left.shape[0], right.shape[1]],
            data: Tensor::from(data),
        }
    }
);

// Transpose Matrix
#[derive(Debug, PartialEq, Clone)]
struct Transpose<T>(Matrix<T>)
where
    T: Copy;

impl<T> Transpose<T>
where
    T: Copy + Clone,
{
    // transpose transposed Matrix (unwarp Transpose)
    pub fn t(self) -> Matrix<T> {
        self.0
    }

    // iter by row
    // it is non-consum iterator
    // 0  1  2
    // 3  4  5
    // 6  7  8
    // 9 10 11
    pub fn into_row_iter<'a>(&'a self) -> impl Iterator<Item = &T> + 'a {
        (0..self.0.shape[1])
            .flat_map(move |b| (b..).step_by(self.0.shape[1]).take(self.0.shape[0]))
            .map(move |n| self.0.data.iter().nth(n).unwrap())
    }

    // iter by col
    // it is non-consum iterator
    // faster than row iter
    // 0  4  8
    // 1  5  9
    // 2  6 10
    // 3  7 11
    pub fn into_col_iter<'a>(&'a self) -> impl Iterator<Item = &T> + 'a {
        self.0.data.iter()
    }
}

impl<T> AsMut<Vec<T>> for Transpose<T>
where
    T: Copy,
{
    fn as_mut(&mut self) -> &mut Vec<T> {
        self.0.as_mut()
    }
}

impl<T> AsRef<Vec<T>> for Transpose<T>
where
    T: Copy,
{
    fn as_ref(&self) -> &Vec<T> {
        self.0.as_ref()
    }
}

// Add Transpose
impl_ops_all!(+[<K, T> where T: Add<K> + Copy,K: Copy,<T as Add<K>>::Output: Copy]
    (left:Transpose<T>, right:Transpose<K>)->Transpose<<T as Add<K>>::Output>{
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(left.0.shape, right.0.shape)
        }
        Transpose(
            Matrix {
                shape: left.0.shape.clone(),
                data: &left.0.data + &right.0.data,
            }
        )
    }
);

// Add Transpose & Matrix
impl_ops_all!(+[<K, T> where T: Add<K> + Copy,K: Copy,<T as Add<K>>::Output: Copy]
    (left:Transpose<T>, right:Matrix<K>)->Matrix<<T as Add<K>>::Output> {
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(left.0.shape, [right.shape[1],right.shape[0]])
        }
        Matrix {
            shape: right.shape.clone(),
            data: Tensor::from(left.into_row_iter().zip(right.into_row_iter()).map(|(&a,&b)| a + b).collect::<Vec<_>>()),
        }
    }
);

// Sub Transpose
impl_ops_all!(-[<K, T> where T: Sub<K> + Copy,K: Copy,<T as Sub<K>>::Output: Copy]
    (left:Transpose<T>, right:Transpose<K>)->Transpose<<T as Sub<K>>::Output> {
        // !for debug
        if cfg!(debug_assertions) {
            assert_eq!(left.0.shape, right.0.shape)
        }
        Transpose(
            Matrix {
                shape: left.0.shape.clone(),
                data: &left.0.data - &right.0.data,
            }
        )
    }
);

// Add Transpose & Matrix
//          3*2
//   2*3   |1 4|
// |1 2 3| |2 5|
// |4 5 6| |3 6|
// impl<K, T> Add<Matrix<K>> for Transpose<T>
// where
//     T: Add<K> + Copy,
//     K: Copy,
//     <T as Add<K>>::Output: Copy + Default,
//     <T as Add<K>>::Output: Add<Output = <T as Add<K>>::Output>,
// {
//     type Output = Matrix<<T as Add<K>>::Output>;

//     fn add(self, rhs: Matrix<K>) -> Self::Output {
//         if cfg!(debug_assertions) {
//             assert_eq!([self.0.shape[1], self.0.shape[0]], rhs.shape)
//         }
//         let mut data = Vec::new();
//         let (mut rit, cit) = (self.into_row_iter(), rhs.into_col_iter());
//         for r in 0..self.0.shape[0] {
//             let cycle = rit.by_ref().take(self.0.shape[1]);
//             for c in 0..rhs.shape[1] {
//                 data.push(
//                     cit.by_ref()
//                         .zip(cycle)
//                         .fold(<T as Add<K>>::Output::default(), |v, (a, b)| (b + a) + v),
//                 );
//             }
//         }
// (0..self.0.shape[0])
//     .flat_map(|r| {
//         (0..self.0.shape[1]).map(move |c| {
//             rhs.into_col_iter()
//                 .skip(c * self.0.shape[1])
//                 .take(self.0.shape[1])
//                 .zip((&self).into_row_iter().skip(r * self.0.shape[1]))
//                 .map(|(a, b)| b + a)
//                 .sum::<<T as Add<K>>::Output>()
//         })
//     })
//     .collect::<<T as Add<K>>::Output>();
// self.into_row_iter().take(self.0.shape[1]).zip(rhs.into_col_iter());
// Matrix {
//     shape: rhs.shape.clone(),
//     data: todo!(),
// }
//         todo!()
//     }
// }
// impl_ops_all!(+[<K, T> where T: Add<K> + Copy,K: Copy,<T as Add<K>>::Output: Copy]
//     (left:Transpose<T>, right:Matrix<K>)->Matrix<<T as Add<K>>::Output>{
//         // !for debug
//         if cfg!(debug_assertions) {
//             assert_eq!(left.0.shape, right.shape)
//         }

//         // Matrix {
//         //     shape: left.0.shape.clone(),
//         //     data: left.into_col_iter().into_iter().
//         // }
//     }
// );

#[cfg(test)]
mod tests {
    use crate::{mat, test_ops, test_ops_all};

    use super::*;

    // 1 1 1
    // 1 1 1
    // Mul
    // 2 2 2
    // 2 2 2
    // 2 2 2
    // Eq
    // 6 6 6
    // 6 6 6
    #[test]
    fn matrix_mul() {
        test_ops_all!(=,*,Matrix::new(1, [2, 3]),Matrix::new(2, [3, 3]),Matrix::new(6, [2, 3]));
    }

    #[test]
    fn matrix_add() {
        test_ops_all!(=,+,Matrix::new(1, [2, 3]),Matrix::new(2, [2, 3]),Matrix::new(3, [2, 3]));
    }

    #[test]
    fn matrix_sub() {
        test_ops_all!(=,-,Matrix::new(5, [2, 3]),Matrix::new(2, [2, 3]),Matrix::new(3, [2, 3]));
    }

    #[test]
    fn matrix_transpose_add() {
        test_ops_all!(=,+,Matrix::new(1, [2, 3]),Matrix::new(2, [3, 2]).t(),Matrix::new(3, [2, 3]));
    }

    #[test]
    fn transpose_add() {
        test_ops_all!(=,+,Matrix::new(1, [2, 3]).t(),Matrix::new(2, [2, 3]).t(),Matrix::new(3, [2, 3]).t());
    }

    #[test]
    fn transpose_matrix_add() {
        test_ops_all!(=,+,Matrix::new(1, [2, 3]).t(),Matrix::new(2, [3, 2]),Matrix::new(3, [3, 2]));
    }

    #[test]
    fn transpose_sub() {
        test_ops_all!(=,-,Matrix::new(3, [2, 3]).t(),Matrix::new(2, [2, 3]).t(),Matrix::new(1, [2, 3]).t());
    }
    #[test]
    fn macro_with_size() {
        assert_eq!(mat![1;5,6], Matrix::new(1, [5, 6]))
    }

    #[test]
    fn macro_with_vec() {
        assert_eq!(
            mat![
                1,2,3,
                4,5,6,
            ;2,3],
            Matrix::from((Tensor::from(vec![1, 2, 3, 4, 5, 6]), [2, 3]))
        )
    }
}