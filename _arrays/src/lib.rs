use std::usize;

pub fn create_array_primitive<T: Copy + Default, const N: usize>() -> [T; N] {
    [T::default(); N]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_array_primitive() -> Result<(), std::io::Error> {
        let arr: [i32; 3] = create_array_primitive();
        assert_eq!([0, 0, 0], arr);
        assert_ne!([0, 0, 1], arr);
        Ok(())
    }
}
