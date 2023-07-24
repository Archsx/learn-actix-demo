use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;


// 配置 route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

// 配置 handler
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Actix Web Service is running")
}


// 如何启动这个项目?
// 在控制台 actix-quickstart 这个目录下
// cargo run -p webservice --bin server1
// 或者进入webservice这个目录
// cargo run --bin server1


// Actix支持两类并发
// 异步I/O, 给定的OS原生线程在等待I/O时执行其他任务(例如侦听网络连接)
// 多线程并行，默认情况下启动OS原生线程的数量与系统逻辑CPU数量相同


#[actix_rt::main]
async fn main() -> io::Result<()> {
    // 构建 app, 配置 route
    let app = move || App::new().configure(general_routes);

    // 运行HTTP server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}