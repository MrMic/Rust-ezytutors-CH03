use actix_web::{web, HttpResponse};
use chrono::Utc;

use crate::{models::Course, state::AppState};

// ______________________________________________________________________
pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;

    HttpResponse::Ok().json(&response)
}

// ______________________________________________________________________
pub async fn new_course(
    // INFO: ++ Extract Data Payload from HTTP Request +----------------+
    new_course: web::Json<Course>,
    // INFO: ++ Extract App State from HTTP Request +-------------------+
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Received new course");
    let course_count_for_user = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == new_course.tutor_id)
        .count();

    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some((course_count_for_user + 1) as i32),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };

    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("Added new course")
}

// * INFO: ▲       ▲
// * INFO: █ TESTS █
// * INFO: ▼       ▼

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_course_test() {
        let course = web::Json(Course {
            tutor_id: 1,
            course_name: "Test Course".into(),
            course_id: None,
            posted_time: None,
        });

        let app_state = web::Data::new(AppState {
            health_check_response: "I'm good. You've already asked me 'I'm good!'".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        let resp = new_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
