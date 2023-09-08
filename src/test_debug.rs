pub fn test_debug_vec(content: Vec<&str>) {
    println!("{:?}", content);
}

pub fn test_debug_vec_mc(content: Vec<&str>) {
    println!("{:#?}", dbg!(content));
}
