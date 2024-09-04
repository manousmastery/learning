fn main() {
    assert_eq!(calc_age(65), 23725);
    assert_eq!(calc_age(0), 0);
    assert_eq!(calc_age(20), 7300);
    println!("success");
}

pub fn calc_age(age: i32) -> i32{
    return 365 * age;
}