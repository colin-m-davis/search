pub mod api;

#[derive(serde::Serialize, serde::Deserialize)]
struct Course {
    courseId: String,
    title: String,
    description: String,
}

async fn get_course_data(quarter_code: String, enroll_code: String) -> String {
    let client = reqwest::Client::new();
    let response = client
        .get(
            format!("https://api.ucsb.edu/academics/curriculums/v3/classes/{quarter_code}/{enroll_code}?includeClassSections=true")
        )
        .header(
            reqwest::header::HeaderName::from_static("ucsb-api-version"),
            reqwest::header::HeaderValue::from_static("3.0")
        )
        .header(
            reqwest::header::HeaderName::from_static("ucsb-api-key"),
            reqwest::header::HeaderValue::from_static(api::API_KEY)
        )
        .send().await.unwrap()
        .text().await.unwrap();
    response
}

fn extract_info(data: String) -> Course {
    let parsed_course: Course = serde_json::from_str(&data[..])
        .unwrap();
    parsed_course
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let quarter = std::env::args().nth(1).expect("no quarter for the wicked");
    let enroll = std::env::args().nth(2).expect("no enroll code?");

    let data = get_course_data(
        quarter.to_string(),
        enroll.to_string()
    )
        .await;
    let course = extract_info(data);

    println!("{}", course.courseId);
    println!("{}", course.title);
    println!("{}", course.description);

    Ok(())
}