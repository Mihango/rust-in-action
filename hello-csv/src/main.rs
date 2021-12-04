fn main() {
    // \ below escapes the trailing line character
    let penguin_data = "\
    common name, length(cm)
    Little penguin, 33
    Yellow-eyed penguin, 65
    Fiordland penguin, 60
    Invalid, data";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 { // skip header row and lines with only whitespaces
            continue;
        }

        let fields: Vec<_> = record
        .split(',')
        .map(|field| field.trim())
        .collect();

        if cfg!(debug_assertions) { // check configuration at compile time
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}
