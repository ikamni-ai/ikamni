use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};

/// APIルート設定
pub fn api_routes() -> Scope {
    web::scope("/api")
        .service(web::resource("/demo/request").route(web::post().to(handle_demo_request)))
        .service(web::resource("/agents/list").route(web::get().to(list_agents)))
        .service(web::resource("/contact").route(web::post().to(handle_contact)))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DemoRequest {
    company_email: String,
    company_name: Option<String>,
    requested_agents: Vec<String>,
}

async fn handle_demo_request(request: web::Json<DemoRequest>) -> HttpResponse {
    // デモリクエスト処理ロジック
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "デモリクエストを受け付けました"
    }))
} 
