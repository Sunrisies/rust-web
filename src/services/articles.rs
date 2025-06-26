use actix_web::HttpResponse;
// 示例接口
pub async fn get_article() -> HttpResponse {
    HttpResponse::Ok().body("文章列表")
}

pub async fn create_article() -> HttpResponse {
    HttpResponse::Ok().body("创建文章")
}
