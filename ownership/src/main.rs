fn main() {
    let stringey: String =  String::from("sussy baka");
    let section1 = &stringey[..5];
    let section2 = &stringey[6..];
    println!("{} {}", section1, section2);
}