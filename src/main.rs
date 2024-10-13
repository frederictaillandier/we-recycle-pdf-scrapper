use chrono::{self, DateTime, Datelike, TimeZone};
use regex::Regex;
use reqwest::blocking::Client;

fn regex_caps_to_datetime(caps: &regex::Captures) -> Option<DateTime<chrono::Local>> {
    let date = &caps[1];

    if let Some(regions) = caps.get(3) {
        if regions.as_str().contains("19") {
            let current_year = chrono::Local::now().date_naive().year();
            let naive_date =
                chrono::NaiveDate::parse_from_str(&format!("{}{}", date, current_year), "%d.%m.%Y")
                    .ok()?;
            let naive_time = chrono::NaiveTime::from_hms_opt(7, 0, 0)?;
            let datetime = chrono::NaiveDateTime::new(naive_date, naive_time);
            return chrono::Local.from_local_datetime(&datetime).single();
        }
    }

    None
}

fn extract_dates_from_txt(
    text: String,
) -> Result<Vec<DateTime<chrono::Local>>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();

    let date_pattern = r"(\d{1,2}\.\d{1,2}\.)";
    let weekday_pattern = r"([A-Z]{2})";
    let regions_pattern = r"([\d\s\+\-]+(?:\s+\d+\s*-\s*\d+)?(?:\s+\d+\s*-\s*\d+)*)?";
    let regex = format!(
        "{}\\s+{}\\s*{}?\\s+",
        date_pattern, weekday_pattern, regions_pattern
    );
    let re = Regex::new(&regex)?;

    for caps in re.captures_iter(&text) {
        if let Some(datetime) = regex_caps_to_datetime(&caps) {
            result.push(datetime);
        }
    }
    result.sort();
    Ok(result)
}

fn download_pdf() -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://www.werecycle.ch/en/abholdaten/";
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;
    let response = client.get(url).send()?;

    let body = response.text()?;
    let re = Regex::new(r#"href="([^"]+\.pdf)""#)?;
    let caps = re
        .captures(&body)
        .ok_or("pdf url could not be found in we-recyle page")?;

    let caps = caps.get(1).ok_or("pdf url corrupted")?;
    let pdf_url = caps.as_str();
    let pdf_response = client.get(pdf_url).send()?;

    let pdf_bytes = pdf_response.bytes()?;
    let pdf_text = pdf_extract::extract_text_from_mem(&pdf_bytes)?;
    return Ok(pdf_text);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let we_recycle_schedule_text = download_pdf()?;
    let extracted_dates = extract_dates_from_txt(we_recycle_schedule_text)?;
    println!("{:?}", extracted_dates);
    Ok(())
}
