use foo::multiply;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_and_multiply(left: usize, right: usize) -> (usize, usize) {
  (add(left,right), multiply(left,right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
