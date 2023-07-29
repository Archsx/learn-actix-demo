// 有点神奇的是，ctrl + 左键点击 web 进去看
// 发现是个 web.rs文件
// 这么写和use actix_web::web::* 有区别吗？
use actix_web::web;

use chrono::NaiveDateTime;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    // 表示id类型都用usize吗？其他语言里面不是long或者Int吗？
    // 后来发现和数据库取数据时的类型不太匹配，所以还是改成i32吧
    pub teacher_id: i32,

    // 使用Option类型的时候，因为新增的时候没有id
    pub id: Option<i32>,

    pub name: String,

    pub time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        // 挺神奇的，虽然course的类型是web::Json<Course>
        // 但是访问它的各字段的时候，就和直接是Course类型一样
        // 直接就是course.xxx
        Course {
            teacher_id: course.teacher_id,
            id: course.id,
            name: course.name.clone(),
            time: course.time,
        }
    }
}
