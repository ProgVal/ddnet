#[no_mangle]
pub extern "C" fn rustfunction2() {
    println!("hi");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
