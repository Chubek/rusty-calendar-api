use crate::utils::{hash_str, get_queries};



#[test]
fn test_hash() {
    println!("{}", hash_str(String::from("aaaa")));
}

#[test]
fn test_queries() {
    println!("{:?}", get_queries("/?q=fsfsf&sss=224"));
}
