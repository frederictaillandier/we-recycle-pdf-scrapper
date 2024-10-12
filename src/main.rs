use pdf_extract;
use regex::Regex;
use std::fs::File;
use std::io::Write;
fn extract_text_from_pdf(path: &str) -> String {
    let text = pdf_extract::extract_text(path).unwrap();
    println!("{}", text);
    text
}

fn extract_dates_for_region_19(text: &str) -> Vec<String> {
    let mut region_19_dates = Vec::new();
    let regex = Regex::new(r"(\d{1,2}\.\d{1,2}\.\d{4})").unwrap(); // Match date pattern

    // Search for "Region 19" in the extracted text
    if let Some(region_pos) = text.find("Region 19") {
        // Extract a relevant section of text near "Region 19"
        let region_text = &text[region_pos..];

        // Use regex to find all dates in the section
        for caps in regex.captures_iter(region_text) {
            let date = caps.get(0).unwrap().as_str().to_string();
            region_19_dates.push(date);
        }
    }

    region_19_dates
}

fn main() {
    // Path to the uploaded PDF file
    let pdf_path = "Abholplan-Oktober-Dezember-2024.pdf";

    // Extract text from the PDF
    let extracted_text = extract_text_from_pdf(pdf_path);

    // Extract dates for Region 19
    let region_19_dates = extract_dates_for_region_19(&extracted_text);

    // Output the extracted dates for Region 19
    println!("Dates for Region 19:");
    /*for date in region_19_dates {
        println!("{}", date);
    }*/

    // Optionally, save the dates to a file
    let mut file = File::create("region_19_dates.txt").unwrap();
    for date in region_19_dates {
        writeln!(file, "{}", date).unwrap();
    }
}
