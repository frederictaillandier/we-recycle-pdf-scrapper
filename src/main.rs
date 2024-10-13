use chrono::{self, DateTime, Datelike, TimeZone};
use pdf_extract;
use regex::Regex;
use reqwest::blocking::Client;

fn extract_dates_from_txt(
    text: String,
) -> Result<Vec<DateTime<chrono::Local>>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    let regex = r"(\d{1,2}\.\d{1,2}\.)\s+([A-Z]{2})\s*([\d\s\+\-]+(?:\s+\d+\s*-\s*\d+)?(?:\s+\d+\s*-\s*\d+)*)?\s+";
    let re = Regex::new(regex)?;

    for caps in re.captures_iter(&text) {
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
                    println!("Error parsing date: e {} datetime {:?}", e, datetime);
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

    Ok(localized_result)
}

fn download_pdf() -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://www.werecycle.ch/en/abholdaten/";
    let client = Client::new();
    let response = client.get(url).send()?;

    let body = response.text()?;
    let re = Regex::new(r#"href="([^"]+\.pdf)""#)?;
    let caps = re
        .captures(&body)
        .unwrap_or_else(|| panic!("no pdf link found on we-recycle page"));
    let pdf_url = caps
        .get(1)
        .unwrap_or_else(|| panic!("pdf url somehow corrupted"))
        .as_str();
    let pdf_response = client.get(pdf_url).send()?;

    let pdf_bytes = pdf_response.bytes().unwrap_or_else(|e| {
        panic!(
            "Error while casting the we-recycle pdf to text {}: {}",
            pdf_url, e
        )
    });
    let pdf_text = pdf_extract::extract_text_from_mem(&pdf_bytes).unwrap_or_else(|e| {
        panic!(
            "Error while extracting text from the we-recycle pdf {}: {}",
            pdf_url, e
        )
    });
    return Ok(pdf_text);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let we_recycle_schedule_text = download_pdf()?;
    let extracted_dates = extract_dates_from_txt(we_recycle_schedule_text);
    println!("{:?}", extracted_dates);
    Ok(())
}
