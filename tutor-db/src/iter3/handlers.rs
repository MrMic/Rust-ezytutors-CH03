use actix_web::{web, HttpResponse};

use crate::{db_access::*, models::Course, state};

// ______________________________________________________________________
pub async fn health_check_handler(app_state: web::Data<state::AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {}", health_check_response, visit_count);

    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

// ______________________________________________________________________
pub async fn get_courses_for_tutor(
    app_state: web::Data<state::AppState>,
    params: web::Path<(i32,)>,
) -> HttpResponse {
    let tutor_id = params.0;
    let courses = get_courses_for_tutor_db(&app_state.db, tutor_id).await;

    HttpResponse::Ok().json(courses)
}

// ______________________________________________________________________
pub async fn get_course_details(
    app_state: web::Data<state::AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    let tutor_id = params.0;
    let course_id = params.1;
    let course = get_course_details_db(&app_state.db, tutor_id, course_id).await;

    HttpResponse::Ok().json(course)
}

// ______________________________________________________________________
pub async fn post_new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<state::AppState>,
) -> HttpResponse {
    let course = post_new_course_db(&app_state.db, new_course.into()).await;

    HttpResponse::Ok().json(course)
}
