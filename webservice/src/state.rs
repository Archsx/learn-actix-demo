// 本来这类数据应该放在数据库中，只是目前先写成在内存中
// 使用Actix框架，这个ApplicationState可以注入到请求的handler里面
// 这样handler就可以通过方法签名的参数来访问到AppState

use sqlx::PgPool;
use std::sync::Mutex;

use super::models::Course;

pub struct AppState {
    // 初始化以后为不可变
    pub health_check_response: String,

    // 是一个可变的数值类型,
    // Mutex是Rust标准库提供的一个机制，线程在修改这个数据之前，需要取得数据的控制权
    pub visit_count: Mutex<u32>,

    // pub courses: Mutex<Vec<Course>>,

    pub db: PgPool,
}
