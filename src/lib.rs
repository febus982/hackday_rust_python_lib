#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    println!("OMG! We are now executing some cool function in a SYSTEM LIBRARY written using Rust");
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
}
