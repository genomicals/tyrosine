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


/// Converts a slice of bytes into a readable list of 1's and 0's.
pub fn bytes_to_unicode_bits(bytes: &[u8]) -> Vec<u8> {
    let mut ret = Vec::with_capacity(bytes.len() * 8);

    for byte in bytes {
        for offset in 0..8 {
            if byte & (1 << offset) == 0 {
                ret.push(b'0');
            } else {
                ret.push(b'1');
            }
        }
    }

    ret
}


/// Converts a slice of bytes into a readable list of 1's and 0's.
pub fn unicode_bits_to_bytes(bits: &[u8]) -> Option<Vec<u8>> {
    let chunks: Vec<&[u8]> = bits.chunks_exact(8).collect();
    if chunks.len() != bits.len() / 8 {
        return None;
    }
    let mut ret = Vec::with_capacity(chunks.len());
    //println!("chunks: {:?}", chunks);

    for chunk in chunks {
        let mut byte: u8 = 0;
        for i in 0..8 {
            match chunk[i] {
                b'0' => continue, //0 means no alteration
                b'1' => byte |= 1 << i, //1 means we must update the byte
                _ => return None, //was not a 1 or 0
            }
        }
        ret.push(byte);
    }
    Some(ret)
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
    #[test]
    fn bits_bytes() {
        let bytes: Vec<u8> = vec![2, 1, 4, 2, 255, 3, 124];
        let bits = bytes_to_unicode_bits(&bytes);
        let new_bytes = unicode_bits_to_bytes(&bits);
        match new_bytes {
            Some(x) => assert_eq!(bytes, x),
            None => assert!(false),
        }
    }
    #[test]
    fn bits_bytes1() {
        let bytes: Vec<u8> = (3.42 as f32).to_le_bytes().into();
        let bits = bytes_to_unicode_bits(&bytes);
        let new_bytes = unicode_bits_to_bytes(&bits);
        match new_bytes {
            Some(x) => assert_eq!(bytes, x),
            None => assert!(false),
        }
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


