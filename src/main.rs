use std::collections::HashMap;

fn main() {
    // get args
    let mut arguments = std::env::args().skip(1);
    // get arg or inform error message
    let key = arguments.next().expect("inform some arg");
    // get arg or panic!
    let value = arguments.next().unwrap();
    println!("The key is '{}' and the value is {}", key, value);

    let mut database = Database::new().expect("Create db panic");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    match database.flush() {
        Ok(()) => println!("Flush ok"),
        Err(err) => println!("Flush error '{}'", err),
    }
}

struct Database {
    map: HashMap<String, String>,
    flush: bool,
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
        Ok(Database { map, flush: false })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(mut self) -> std::io::Result<()> {
        self.flush = true;
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _ = do_flush(self); // don't care abour the Result
        }
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
    // flush the data is the last thing that a database does
    // so the contents should not be available anymore after flush
    // this justify why we take ownership here
    let mut contents = String::new();
    for (key, value) in &database.map {
        // create a kvpair below for each interaction with the db is memory unefficient
        // let kvpair = format!("{}\t{}\n", key, value);
        // contents.push_str(&kvpair);
        // this is more eficient
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }
    std::fs::write("kv.db", contents)
}
