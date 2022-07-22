pub mod api;

use reqwest::header::{ HeaderName, HeaderValue };

async fn get_course_data(quarter_code: String, enroll_code: String) -> String {
    let client = reqwest::Client::new();
    let response = client
        .get(
            format!("https://api.ucsb.edu/academics/curriculums/v3/classes/{quarter_code}/{enroll_code}?includeClassSections=true")
        )
        .header(
            HeaderName::from_static("ucsb-api-version"),
            HeaderValue::from_static("3.0")
        )
        .header(
            HeaderName::from_static("ucsb-api-key"),
            HeaderValue::from_static(api::API_KEY)
        )
        .send().await.unwrap()
        .text().await.unwrap();
    response
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let text = get_course_data("20222".to_string(), "31864".to_string()).await;
    println!("{}", text);
    Ok(())
}