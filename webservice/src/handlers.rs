// cargo check --bin teacher-service

use super::db_access::*;
use super::state::AppState;
use actix_web::{web, HttpResponse};
use chrono::NaiveDateTime;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;

    let mut visit_count = app_state.visit_count.lock().unwrap(); // 这里的lock方法很重要,是为了防止其他线程同时更新这个值

    let response = format!("{} {} times", health_check_response, visit_count);

    *visit_count += 1;

    HttpResponse::Ok().json(&response)

    // 执行完这个handler以后，这个锁就会释放
}

use super::models::Course;
use chrono::Utc;

pub async fn new_course(
    app_state: web::Data<AppState>,
    new_course: web::Json<Course>,
) -> HttpResponse {
    println!("Received new course");

    // let course_count = app_state
    //     .courses
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .filter(|course| course.teacher_id == new_course.teacher_id)
    //     .collect::<Vec<Course>>()
    //     .len();
    //
    // let new_course = Course {
    //     teacher_id: new_course.teacher_id,
    //     // 这里实际上应该是使用了teacher_id和id这两个字段来保证唯一性
    //     // (意思是单纯只看id的话，是有可能重复的)
    //     id: Some(course_count + 1),
    //     name: new_course.name.clone(),
    //     time: Some(Utc::now().naive_utc()),
    // };

    // // 离谱，这里怎么还要lock啊？
    // app_state.courses.lock().unwrap().push(new_course);

    // HttpResponse::Ok().json(course)

    let course = post_new_course_db(&app_state.db, new_course.into()).await;

    HttpResponse::Ok().json(course)
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(usize)>, // params里面其实是一个元组，元组里面就一个元素，是usize类型,注意，这里的逗号不能省略，加了才表示元组
) -> HttpResponse {
    // let teacher_id: usize = params.0;
    //
    // let filtered_course = app_state
    //     .courses
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .filter(|course| course.teacher_id == teacher_id)
    //     .collect::<Vec<Course>>();
    //
    // if filtered_course.len() > 0 {
    //     HttpResponse::Ok().json(filtered_course)
    // } else {
    //     HttpResponse::Ok().json("No courses found for teacher".to_string())
    // }

    let tid = params.into_inner();
    let teacher_id = i32::try_from(tid).unwrap();
    let courses = get_courses_for_teacher_db(&app_state.db, teacher_id).await;

    HttpResponse::Ok().json(courses)
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    // // let params = params.0;
    // // let teacher_id = params.0;
    // // let course_id = params.1;
    //
    // let (teacher_id, course_id) = params.0;
    //
    // let filtered_course = app_state
    //     .courses
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .find(|course| course.teacher_id == teacher_id && course.id.unwrap() == course_id)
    //     .ok_or("Course not found"); // 搞不懂为什么这里需要转换成Result形式,直接使用Option不好吗？
    //
    // if let Ok(course) = filtered_course {
    //     HttpResponse::Ok().json(course)
    // } else {
    //     HttpResponse::Ok().json("Course not found".to_string())
    // }

    // // match filtered_course {
    // //     Some(course) => HttpResponse::Ok().json(course),
    // //     None => HttpResponse::Ok().json("Sorry, the course you are trying to search doesn't exist"),
    // // }

    let (teacher_id, course_id) = params.into_inner();

    let course = get_course_details_db(&app_state.db, teacher_id, course_id).await;

    HttpResponse::Ok().json(course)
}

// cargo test --bin teacher-service
#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::http::StatusCode;

    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_course_test() {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let course = web::Json(Course {
            teacher_id: 1,
            name: "Test Course".into(),
            id: None,
            time: None,
        });

        let resp = new_course(app_state, course).await; // 对于异步的疑惑，在上面的new_course函数中，除了async关键字，是在不知道哪里是异步的

        assert_eq!(resp.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let teacher_id: web::Path<(usize)> = web::Path::from((1));

        let resp = get_courses_for_teacher(app_state, teacher_id).await;

        // 其实这里的测试我觉得差点意思
        // 为什么只能比较状态码啊
        // 不能把数据拿出来对比吗？
        assert_eq!(resp.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn get_one_course_success() {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let resp = get_course_detail(app_state, params).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
