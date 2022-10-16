use std::collections::HashMap;

fn main() {
    // get args
    let mut arguments = std::env::args().skip(1);
    // get arg or inform error message
    let key = arguments.next().expect("inform some arg");
    // get arg or panic!
    let value = arguments.next().unwrap();
    println!("The key is '{}' and the value is {}", key, value);

    let contents = format!("{}\t{}\n", key, value);
    // write file or panic!
    std::fs::write("kv.db", contents).unwrap();

    let database = Database::new().expect("Database::new() panic!");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // // read kv.db
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Err(error);
        //     }
        // };
        // // same as above because ? operator
        let contents = std::fs::read_to_string("kv.db")?;

        // parse the string

        // populate the map
        Ok(Database {
            map: HashMap::new(),
        })
    }
}
