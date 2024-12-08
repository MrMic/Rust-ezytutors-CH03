use sqlx::{postgres::PgPool, types::chrono};

use super::errors::EzyTutorError;
use crate::models::Course;

// ______________________________________________________________________
pub async fn get_courses_for_tutor_db(pool: &PgPool, tutor_id: i32) -> Vec<Course> {
    // * INFO: Prepare SQL Statement
    let course_rows = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c4 WHERE tutor_id = $1",
        tutor_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    // * INFO: Extract result
    course_rows
        .iter()
        .map(|course_row| Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
        })
        .collect()
}

// ______________________________________________________________________
pub async fn get_course_details_db(pool: &PgPool, tutor_id: i32, course_id: i32) -> Course {
    // * INFO: Prepare SQL Statement
    let course_row = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c4 WHERE tutor_id = $1 AND course_id = $2",
        tutor_id,
        course_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    // * INFO: Execute query
    Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
    }
}

// ______________________________________________________________________
pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Course {
    // * INFO: Prepare SQL Statement
    let course_row = sqlx::query!(
        "INSERT INTO ezy_course_c4 (course_id,tutor_id, course_name) VALUES ($1, $2, $3) RETURNING tutor_id, course_id, course_name, posted_time",
        new_course.course_id,
        new_course.tutor_id,
        new_course.course_name
    )
    .fetch_one(pool)
    .await
    .unwrap();

    // * INFO: Retrieve result
    Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
    }
}
