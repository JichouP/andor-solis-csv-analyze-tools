use anyhow::{Ok, Result};

fn main() {
    println!("Hello, world!");
}

/// read csv file and return as String
fn read_csv_file(path: &str) -> Result<Vec<Vec<String>>> {
    // read file
    let raw_data = std::fs::read_to_string(path)?;
    let data: Vec<&str> = raw_data.trim().lines().collect();

    // Extract only before empty string elements
    let limit = data.iter().position(|&v| v == "").unwrap_or(data.len());
    let data = data[..limit].to_vec();

    let data: Vec<Vec<String>> = data
        .iter()
        .map(|&x| x.split(',').map(|v| v.to_string()).collect())
        .collect();
    Ok(data)
}

struct Bundle {
    data: Vec<Vec<String>>,
}

impl Bundle {
    fn new(data: Vec<Vec<String>>) -> Self {
        Self { data }
    }

    fn assign_to_column(&mut self, new_column: &mut Vec<String>) {
        new_column.reverse();
        for row in &mut self.data {
            row.push(new_column.pop().unwrap_or("".to_string()));
        }
    }

    fn assign_to_column_from_csv(&mut self, path: &str) -> Result<()> {
        let new_column = read_csv_file(path)?;
        let mut new_column: Vec<String> = new_column
            .iter()
            .map(|v| {
                if let Some(v) = v.get(1) {
                    (*v).clone()
                } else {
                    "".to_string()
                }
            })
            .collect();
        self.assign_to_column(&mut new_column);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut bundle = super::Bundle::new(super::read_csv_file("sample/example.asc").unwrap());
        bundle
            .assign_to_column_from_csv("sample/example2.asc")
            .unwrap();
        bundle
            .assign_to_column_from_csv("sample/example3.asc")
            .unwrap();
        assert_eq!(
            bundle.data,
            vec![
                ["0", "0", "0", "0"],
                ["1", "1", "2", "3"],
                ["2", "2", "4", "6"],
                ["3", "3", "6", "9"],
                ["4", "4", "8", "12"],
                ["5", "5", "10", "15"],
                ["6", "6", "12", "18"],
                ["7", "7", "14", "21"],
                ["8", "8", "16", "24"],
                ["9", "9", "18", "27"],
                ["10", "10", "20", "30"]
            ]
        );
    }
}
