fn main() {
    let name = String::from("com.123123");
    if name.contains(".") {
        let s = name.replace(".", "/");
        println!("{}", s);
    }
}


