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

    let database = Database::new().expect("Create db panic");
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

        let mut map = HashMap::new();
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");
            map.insert(key.to_owned(), value.to_owned());
        }

        // parse string

        // populate map
        Ok(Database { map: map })
    }
}
