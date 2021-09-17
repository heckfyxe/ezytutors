use crate::models::Course;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_data: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_data.health_check_response;
    let mut visit_count = app_data.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn get_courses_for_tutor(
    _app_state: web::Data<AppState>,
    _params: web::Path<usize>,
) -> HttpResponse {
    HttpResponse::Ok().json("success")
}

pub async fn get_course_details(
    _app_state: web::Data<AppState>,
    _params: web::Path<(usize, usize)>,
) -> HttpResponse {
    HttpResponse::Ok().json("success")
}

pub async fn post_new_course(
    _new_course: web::Json<Course>,
    _app_state: web::Data<AppState>,
) -> HttpResponse {
    HttpResponse::Ok().json("success")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use chrono::NaiveDate;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    async fn get_app_state() -> web::Data<AppState> {
        dotenv::dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool = PgPool::new(&database_url).await.unwrap();
        web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        })
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        let app_state = get_app_state().await;
        let tutor_id = web::Path::from(1);
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_test() {
        let app_state = get_app_state().await;
        let params = web::Path::from((1, 1));
        let resp = get_course_details(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test()]
    async fn post_course_success() {
        let app_state = get_app_state().await;
        let new_course_msg = Course {
            course_id: 1,
            tutor_id: 1,
            course_name: "This is the next course".into(),
            posted_time: Some(NaiveDate::from_ymd(2020, 9, 17).and_hms(14, 01, 11)),
        };
        let course_params = web::Json(new_course_msg);
        let resp = post_new_course(course_params, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
