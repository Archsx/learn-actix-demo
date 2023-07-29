use crate::models::Course;
use sqlx::PgPool;

pub async fn get_courses_for_teacher_db(pool: &PgPool, teacher_id: i32) -> Vec<Course> {
    let rows = sqlx::query!(
        r#"SELECT id,teacher_id,name,time FROM course where id = $1"#,
        teacher_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    rows.iter()
        .map(|r| Course {
            id: Some(r.id),
            teacher_id: r.teacher_id,
            name: r.name.clone(),
            time: r.time,
        })
        .collect()
}

pub async fn get_course_details_db(pool: &PgPool, teacher_id: i32, course_id: i32) -> Course {
    let row = sqlx::query!(
        r#"SELECT id, teacher_id, name, time FROM course WHERE teacher_id = $1 and id = $2"#,
        teacher_id,
        course_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    Course {
        teacher_id: row.teacher_id,
        id: Some(row.id),
        name: row.name,
        time: row.time,
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Course {
    let row = sqlx::query!(
        r#"INSERT INTO course (teacher_id,name) VALUES ($1,$2) RETURNING id,teacher_id,name,time"#,
        new_course.teacher_id,
        new_course.name
    )
    .fetch_one(pool)
    .await
    .unwrap();


    Course {
        id: Some(row.id),
        name: row.name,
        teacher_id: row.teacher_id,
        time: row.time,
    }
}
