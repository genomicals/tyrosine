mod genome;
mod topo;
mod creature;
mod generation_manager;


extern {
    fn verify_cuda() -> u32;
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

    #[test]
    fn cuda_test() {
        assert!(unsafe { verify_cuda() } == 1);
    }
}


