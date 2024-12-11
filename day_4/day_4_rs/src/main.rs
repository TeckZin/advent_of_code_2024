use std::fs;

fn main() {
    let content = match read_file() {
        Ok(content) => content,
        Err(err) => {
            println!("err {}", err);
            String::new()
        }
    };
    part_1(content);
}

fn read_file() -> Result<String, std::io::Error> {
    return fs::read_to_string("src/sample.txt");
}

fn part_1(content: String) {
    println!("{}", content);
    let content_rows: Vec<&str> = content.lines().collect();
    // also works
    // println!("{:?}", content_rows);
    println!("{:#?}", content_rows);
    for row in &content_rows {
        println!("row {}", row);

        // let index: i32 = match row.find('X') {
        //     Some(index) => index as i32,
        //     None => -1,
        // };
        // if index >= 0 {
        //     if let Some(char) = row.chars().nth(index as usize) {
        //         println!("here {}", char);
        //     }
        // }
        //
        //
        // if let Some(index) = row.find('X') {
        //     // let char = row.chars().nth(index).unwrap();
        //     // println!("{}", char);
        //     let slice = &row[index..index + 4];
        //     println!("{}", slice);
        // }

        for (index, _) in row.match_indices("X") {
            let slice = &row[index..index + 4];
            if slice == "XMAS" {
                println!("{}", slice);
                println!("next\n");
            }
        }
    }
}
