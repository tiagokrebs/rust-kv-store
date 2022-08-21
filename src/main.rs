fn main() {
    // get args
    let mut arguments = std::env::args().skip(1);
    // get arg or inform error message
    let key = arguments.next().expect("inform some arg");
    // get arg or panic!
    let value = arguments.next().unwrap();
    println!("The key is '{}' and the value is {}", key, value);

    //file writing
    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents);
}
