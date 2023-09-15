pub fn _test_debug_vec(content: Vec<&str>) {
    println!("{:?}", content);
}

pub fn _test_debug_vec_mc(content: Vec<&str>) {
    println!("{:#?}", dbg!(content));
}

pub fn _test_debug_string(content: String) {
    println!("{}", content);
}

pub fn _test_debug_string_mc(content: String) {
    println!("{:#?}", dbg!(content));
}
