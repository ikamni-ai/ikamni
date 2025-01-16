use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// APIルート計画
pub fn keikaku_routes() -> Scope {
    web::scope("/api")
        .service(web::resource("/shiyou/moushikomi").route(web::post().to(handle_shiyou_moushikomi)))
        .service(web::resource("/agent/ichiran").route(web::get().to(list_agents)))
        .service(web::resource("/toiawase").route(web::post().to(handle_toiawase)))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShiyouMoushikomi {
    kaisha_mail: String,
    kaisha_mei: Option<String>,
    kibou_agent: Vec<String>,
    moushikomi_timestamp: chrono::DateTime<chrono::Utc>,
}

async fn handle_shiyou_moushikomi(moushikomi: web::Json<ShiyouMoushikomi>) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "試用申し込みを受け付けました。"
    }))
} 