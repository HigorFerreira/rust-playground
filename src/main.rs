use std::iter::Enumerate;

fn main() {

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    {
        println!("Inside scope of v");        
        let v = vec![100, 32, 57];

        for i in &v {
            println!("V elment: {i}");
        }
    }
    println!("Outside scope of v");
    // println!("{v:?}", v); // This will not compile, as v is out of scope here
}