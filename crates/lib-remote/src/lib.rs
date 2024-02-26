pub mod cmd;
pub mod processor;

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     let mut v = vec![1, 2, 3];
    //     v.sort();
    //     (v).sort();

    //     fn factorial(n: usize) -> usize {
    //         (1..n + 1).fold(1, |a, b| a * b)
    //     }
    //     let r = &factorial(6);
    //     assert_eq!(r + 1009, 1729);
    //     let x = 10;
    //     let _r;
    //     {
    //         let y = 20;
    //         {
    //             let s = S { _x: &x, _y: &y };
    //             _r = s.x;
    //         }
    //     }

    //     let mut wave = Vec::new();
    //     let head = vec![0.0, 1.0];
    //     let tail = [0.0, -1.0];
    //     extend(&mut wave, &head); // 用另一个向量扩展wave
    //     extend(&mut wave, &tail); // 用另一个向量扩展wave
    //     assert_eq!(wave, [0.0, 1.0, 0.0, -1.0]); // 检查wave是否正确
    //                                              // extend(&mut wave, &wave);
    //                                              // assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0]);
    // }
    // struct S<'a, 'b> {
    //     _x: &'a i32,
    //     _y: &'b i32,
    // }
    // fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    //     for elt in slice {
    //         vec.push(*elt);
    //     }
    // }
}
