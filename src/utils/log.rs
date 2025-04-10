use std::io::BufRead;

use chrono::Local;

use crate::models::alert::Alert;

#[derive(Clone)]
pub struct Logger {
    file_path: String,
}

impl Logger {
    pub fn new(file_path: &str) -> Result<Logger, std::io::Error> {
        if !std::path::Path::new(file_path).exists() {
            std::fs::File::create(file_path)?;
        }
        Ok(Logger {
            file_path: file_path.to_string(),
        })
    }

    pub fn log(&mut self, alert: Alert) -> Result<(), std::io::Error> {
        let time = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;
        use std::io::Write;
        writeln!(file, "{}: {}", time, alert)?;
        Ok(())
    }

    pub fn get(&self, n: usize) -> Result<Vec<String>, std::io::Error> {
        let file = std::fs::File::open(&self.file_path)?;
        let reader = std::io::BufReader::new(file);
        let mut lines = Vec::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                lines.push(line);
            }
        }
        if n == 0 {
            return Ok(lines);
        }

        let start = if n > lines.len() { 0 } else { lines.len() - n };
        let end = lines.len();
        Ok(lines[start..end].to_vec())
    }
}
