use anyhow::{Ok, Result};
use std::io::Write;
use std::path::Path;

const INPUT_DIR: &str = "input";
const OUTPUT_DIR: &str = "output";
const RESULT_FILE_NAME: &str = "result.csv";
const RESULT_NORMALIZED_FILE_NAME: &str = "result_normalized.csv";
const RESULT_INFO_FILE_NAME: &str = "result_info.csv";

fn main() -> Result<()> {
    let mut created: bool = false;
    // Confirm that the input directory exists.
    // If it doesn't, create it and exit with an error.
    if !Path::new(INPUT_DIR).is_dir() {
        eprintln!("Input directory does not exist.");
        eprintln!("Creating input directory...");
        std::fs::create_dir(INPUT_DIR)?;
        created = true;
    }

    // Confirm that the output directory exists.
    // If it doesn't, create it and exit with an error.
    if !Path::new(OUTPUT_DIR).is_dir() {
        eprintln!("Output directory does not exist.");
        eprintln!("Creating output directory...");
        std::fs::create_dir(OUTPUT_DIR)?;
        created = true;
    }

    // If the input or output directory was created, exit with an error.
    if created {
        eprintln!("Input or output directory created.");
        eprintln!("Please run the program again.");
        std::process::exit(1);
    }

    // Get a list of all the input files.
    let input_files: Vec<String> = std::fs::read_dir(INPUT_DIR)?
        .filter_map(|entry| Some(entry.ok()?.path().to_string_lossy().to_string()))
        .collect();

    // Create Bundle object

    let mut bundle = Bundle::init(&input_files[0])?;

    input_files.iter().for_each(|file_path| {
        bundle.assign_to_column_from_csv(file_path).unwrap();
    });

    bundle.save_data(&format!("{}/{}", OUTPUT_DIR, RESULT_FILE_NAME))?;
    bundle.save_normalized_data(&format!("{}/{}", OUTPUT_DIR, RESULT_NORMALIZED_FILE_NAME))?;
    bundle.save_info(&format!("{}/{}", OUTPUT_DIR, RESULT_INFO_FILE_NAME))?;

    Ok(())
}

type AcquisitionDataList = Vec<String>;
type AcquisitionDataTable = Vec<AcquisitionDataList>;
type AcquisitionInfoList = Vec<String>;
type AcquisitionWaveLengthList = Vec<String>;
type ExposureTimeList = Vec<String>;
type FilenameList = Vec<String>;

struct Bundle {
    data: AcquisitionDataTable,
    info: AcquisitionInfoList,
    wave_length: AcquisitionWaveLengthList,
    exposure_time: ExposureTimeList,
    filename: FilenameList,
}

impl Bundle {
    /// get info and wave_length from csv file
    fn init(ref_path: &str) -> Result<Bundle> {
        // read file
        let raw_data = std::fs::read_to_string(ref_path)?;
        let lines: Vec<&str> = raw_data.trim().lines().collect();

        // Extract only before empty string elements
        let limit = lines.iter().position(|&v| v == "").unwrap_or(lines.len());

        let wave_length: AcquisitionWaveLengthList = lines[..limit]
            .iter()
            .map(|&x| x.split(',').collect::<Vec<&str>>())
            .map(|v| v[0].to_string())
            .collect();

        let info: AcquisitionInfoList = lines[limit..]
            .iter()
            .filter_map(|&v| if v != "" { Some(v.to_string()) } else { None })
            .collect();

        Ok(Self {
            data: vec![vec![]; wave_length.len()],
            info,
            wave_length,
            exposure_time: vec![],
            filename: vec![],
        })
    }

    fn assign_to_column_from_csv(&mut self, path: &str) -> Result<()> {
        // read file
        let raw_data = std::fs::read_to_string(path)?;
        let lines: Vec<&str> = raw_data.trim().lines().collect();

        // Extract only before empty string elements
        let limit = lines.iter().position(|&v| v == "").unwrap_or(lines.len());

        let new_data: Vec<String> = lines[..limit]
            .iter()
            .map(|&x| x.split(',').collect::<Vec<&str>>())
            .map(|v| v[1].to_string())
            .collect();

        for i in 0..self.wave_length.len() {
            self.data[i].push(new_data[i].clone());
        }

        let exposure_time = lines[limit..]
            .iter()
            .find(|&v| v.contains("Exposure Time"))
            .expect("Exposure Time not found")
            .split(":")
            .nth(1)
            .expect("Exposure Time format error")
            .trim()
            .to_string();

        self.exposure_time.push(exposure_time);
        self.filename.push(
            Path::new(path)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );

        Ok(())
    }

    fn save_data(&self, path: &str) -> Result<()> {
        let mut file = std::fs::File::create(path)?;
        let mut content = format!("Wave Length (nm),{}\n", self.filename.join(","));
        content.push_str(&format!(
            "Exposure Time (sec),{}\n",
            self.exposure_time.join(",")
        ));
        for i in 0..self.wave_length.len() {
            let line = format!("{},{}\n", self.wave_length[i], self.data[i].join(","));
            content += &line;
        }
        write!(file, "{}", content)?;
        file.flush()?;
        Ok(())
    }

    fn save_normalized_data(&self, path: &str) -> Result<()> {
        let mut file = std::fs::File::create(path)?;
        let mut content = format!("Wave Length (nm),{}\n", self.filename.join(","));
        for i in 0..self.wave_length.len() {
            let data = self.data[i].clone();
            let data: Vec<String> = data
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let data = v.parse::<f64>().unwrap();
                    let time = self.exposure_time[i].parse::<f64>().unwrap();
                    let normalized_data = data / time;
                    format!("{}", normalized_data)
                })
                .collect();
            let line = format!("{},{}\n", self.wave_length[i], data.join(","));
            content += &line;
        }
        write!(file, "{}", content)?;
        file.flush()?;
        Ok(())
    }

    fn save_info(&self, path: &str) -> Result<()> {
        let mut file = std::fs::File::create(path)?;
        let content = self.info.join("\n");
        writeln!(file, "{}", content)?;
        file.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut bundle = super::Bundle::init("sample/example1.asc").unwrap();
        assert_eq!(bundle.exposure_time.len(), 0);
        assert_eq!(bundle.wave_length.len(), 10);
        bundle
            .assign_to_column_from_csv("sample/example1.asc")
            .unwrap();
        bundle
            .assign_to_column_from_csv("sample/example2.asc")
            .unwrap();
        bundle
            .assign_to_column_from_csv("sample/example3.asc")
            .unwrap();
        assert_eq!(
            bundle.data,
            vec![
                ["12", "8", "4"],
                ["24", "16", "8"],
                ["36", "24", "12"],
                ["48", "32", "16"],
                ["60", "40", "20"],
                ["72", "48", "24"],
                ["84", "56", "28"],
                ["96", "64", "32"],
                ["108", "72", "36"],
                ["120", "80", "40"],
            ]
        );
        bundle.save_data("sample/result.csv").unwrap();
        bundle
            .save_normalized_data("sample/result_normalized.csv")
            .unwrap();
    }
}
