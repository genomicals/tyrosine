mod genome;
mod topo;
mod creature;
mod generation_manager;


pub mod errors {
    /// Centralized error enum for Tyrosine.
    pub enum TyrosineError {
        InvalidGenome,
        EmptyPopulation,
        CouldntCreateFile,
        CouldntWriteFile,
        CouldntReadFile,
        InvalidFileFormat,
        InvalidGenomeFormat,
    }
}


extern {
    fn verify_cuda() -> u32;
    fn maxmul(a: *const f32, b: *const f32, c: *mut f32, size: i32) -> ();
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    //#[test]
    //fn cuda_test0() {
    //    let a: Vec<f32> = vec![-1.0, 2.0, 4.0, 0.0, 5.0, 3.0, 6.0, 2.0, 1.0];
    //    let b: Vec<f32> = vec![3.0, 0.0, 2.0, 3.0, 4.0, 5.0, 4.0, 7.0, 2.0];
    //    let mut c: Vec<f32> = vec![0.0; 9];
    //    let a_point = a.as_ptr() as *const f32;
    //    let b_point = b.as_ptr() as *const f32;
    //    let c_point = c.as_mut_ptr() as *mut f32;
    //    //let expected = vec![19.0, 36.0, 16.0, 27.0, 41.0, 31.0, 28.0, 15.0, 24.0];
    //    let expected = vec![2.3, 36.0, 16.0, 27.0, 41.0, 31.0, 28.0, 15.0, 24.0];

    //    unsafe { maxmul(a_point, b_point, c_point, 3) };
    //    assert_eq!(c, expected);

    //    //println!("{:?}", c);
    //}
    //#[test]
    //fn cuda_test1() {
    //    assert!(unsafe { verify_cuda() } == 1);
    //}
}


