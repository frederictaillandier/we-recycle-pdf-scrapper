use chrono::{self, DateTime, Datelike, TimeZone};
use pdf_extract;
use regex::Regex;

fn extract_text_from_pdf(path: &str) -> String {
    match pdf_extract::extract_text(path) {
        Ok(text) => text,
        Err(e) => {
            println!("Error: {}", e);
            String::new()
        }
    }
}

fn extract_dates_from_txt(text: &str) -> Vec<DateTime<chrono::Local>> {
    let mut result = Vec::new();
    let re = Regex::new(r"(\d{1,2}\.\d{1,2}\.)\s+([A-Z]{2})\s*([\d\s\+\-]+(?:\s+\d+\s*-\s*\d+)?(?:\s+\d+\s*-\s*\d+)*)?\s+").unwrap();

    for caps in re.captures_iter(text) {
        let date = &caps[1];

        if (&caps).len() < 4 || caps.get(3).is_none() {
            continue;
        }
        let regions = &caps[3];

        if regions.contains("19") {
            let current_year = chrono::Local::now().date_naive().year();
            let formated_date = format!("07:00:00 {}{}", date, current_year);
            let datetime =
                chrono::NaiveDateTime::parse_from_str(&formated_date, "%H:%M:%S %d.%m.%Y");

            match datetime {
                Ok(dt) => {
                    result.push(dt);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
    result.sort();
    let localized_result = result
        .iter()
        .map(|ndt| match chrono::Local.from_local_datetime(&ndt) {
            chrono::LocalResult::Single(dt) => dt,
            _ => chrono::Local::now(),
        })
        .collect();

    localized_result
}

fn main() {
    // Path to the uploaded PDF file
    let pdf_path = "Abholplan-Oktober-Dezember-2024.pdf";
    let extracted_text = extract_text_from_pdf(pdf_path);
    let extracted_dates = extract_dates_from_txt(&extracted_text);

    println!("{:?}", extracted_dates);
}
