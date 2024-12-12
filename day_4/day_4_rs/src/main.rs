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

    // dia

    let content_rows: Vec<&str> = content.lines().collect();
    // also works
    // println!("{:?}", content_rows);
    println!("{:#?}", content_rows);
    let mut content_columns: Vec<String> = vec![String::new(); content_rows.len()];
    for line in content.lines() {
        for (i, c) in line.chars().enumerate() {
            content_columns[i].push(c);
        }
    }

    for col in &content_columns {
        println!("col {}", col);
        for (index, _) in col.match_indices("X") {
            if index + 4 < col.len() {
                let slice = &col[index..index + 4];
                if slice == "XMAS" {
                    println!("{}", slice);
                    println!("next\n");
                }
            }
        }

        let rev_col: String = col.chars().rev().collect();

        for (index, _) in rev_col.match_indices("X") {
            if index + 4 < col.len() {
                let slice = &rev_col[index..index + 4];
                if slice == "XMAS" {
                    println!("{}", slice);
                    println!("next\n");
                }
            }
        }
    }

    // println!("{:#?}", content_columns);
    for row in &content_rows {
        // println!("row {}", row);

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
        //
        //

        // HORIZONTAL

        for (index, _) in row.match_indices("X") {
            if index + 4 < row.len() {
                let slice = &row[index..index + 4];
                if slice == "XMAS" {
                    // println!("{}", slice);
                    // println!("next\n");
                }
            }
        }

        let rev_row: String = row.chars().rev().collect();

        for (index, _) in rev_row.match_indices("X") {
            if index + 4 < row.len() {
                let slice = &rev_row[index..index + 4];
                if slice == "XMAS" {
                    // println!("{}", slice);
                    // println!("next\n");
                }
            }
        }
    }
}
