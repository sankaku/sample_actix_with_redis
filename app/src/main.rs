use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sample_actix_with_redis::direct;
use sample_actix_with_redis::with_bb8;
use sample_actix_with_redis::with_deadpool;
use sample_actix_with_redis::with_mobc;
use sample_actix_with_redis::with_r2d2;
use sample_actix_with_redis::with_r2d2_feature;

use uuid::Uuid;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/direct")]
async fn hello_direct_redis() -> impl Responder {
    let id = Uuid::new_v4();
    let key = format!("{}", id);
    let value = "hi";
    let result = direct::set(&key, value).await;
    match result {
        Ok(_) => HttpResponse::Ok().body(key),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/r2d2_feature")]
async fn hello_r2d2_feature_redis(
    pool: web::Data<with_r2d2_feature::R2D2FeaturePool>,
) -> impl Responder {
    let id = Uuid::new_v4();
    let key = format!("{}", id);
    let value = "hi";
    let result = with_r2d2_feature::set(&pool, &key, value);
    match result {
        Ok(_) => HttpResponse::Ok().body(key),
        Err(e) => HttpResponse::InternalServerError().body(e.msg),
    }
}

#[get("/r2d2")]
async fn hello_r2d2_redis(pool: web::Data<with_r2d2::R2D2Pool>) -> impl Responder {
    let id = Uuid::new_v4();
    let key = format!("{}", id);
    let value = "hi";
    let result = with_r2d2::set(&pool, &key, value);
    match result {
        Ok(_) => HttpResponse::Ok().body(key),
        Err(e) => HttpResponse::InternalServerError().body(e.msg),
    }
}

#[get("/bb8")]
async fn hello_bb8_redis(pool: web::Data<with_bb8::BB8Pool>) -> impl Responder {
    let id = Uuid::new_v4();
    let key = format!("{}", id);
    let value = "hi";
    let result = with_bb8::set(&pool, &key, value).await;
    match result {
        Ok(_) => HttpResponse::Ok().body(key),
        Err(e) => HttpResponse::InternalServerError().body(e.msg),
    }
}

#[get("/deadpool")]
async fn hello_deadpool_redis(pool: web::Data<with_deadpool::DeadpoolPool>) -> impl Responder {
    let id = Uuid::new_v4();
    let key = format!("{}", id);
    let value = "hi";
    let result = with_deadpool::set(&pool, &key, value).await;
    match result {
        Ok(_) => HttpResponse::Ok().body(key),
        Err(e) => HttpResponse::InternalServerError().body(e.msg),
    }
}

#[get("/mobc")]
async fn hello_mobc_redis(pool: web::Data<with_mobc::MobcPool>) -> impl Responder {
    let id = Uuid::new_v4();
    let key = format!("{}", id);
    let value = "hi";
    let result = with_mobc::set(&pool, &key, value).await;
    match result {
        Ok(_) => HttpResponse::Ok().body(key),
        Err(e) => HttpResponse::InternalServerError().body(e.msg),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "redis://127.0.0.1/";
    // use this conn every time?
    let direct_connection = direct::create_connection()
        .await
        .expect("failed to create direct connection");
    let r2d2_feature_pool = with_r2d2_feature::create_pool(host).unwrap();
    let r2d2_pool = with_r2d2::create_pool(host).unwrap();
    let bb8_pool = with_bb8::create_pool(host).await.unwrap();
    let deadpool_pool = with_deadpool::create_pool(host).unwrap();
    let mobc_pool = with_mobc::create_pool(host);

    HttpServer::new(move || {
        // TODO: needs `move`?
        App::new()
            .app_data(web::Data::new(r2d2_feature_pool.clone()))
            .app_data(web::Data::new(r2d2_pool.clone()))
            .app_data(web::Data::new(bb8_pool.clone()))
            .app_data(web::Data::new(deadpool_pool.clone()))
            .app_data(web::Data::new(mobc_pool.clone()))
            .service(hello)
            .service(echo)
            .service(hello_direct_redis)
            .service(hello_r2d2_feature_redis)
            .service(hello_r2d2_redis)
            .service(hello_bb8_redis)
            .service(hello_deadpool_redis)
            .service(hello_deadpool_redis)
            .service(hello_mobc_redis)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
