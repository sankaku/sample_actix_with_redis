use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sample_actix_with_redis::direct;
use sample_actix_with_redis::with_bb8;
use sample_actix_with_redis::with_deadpool;
use sample_actix_with_redis::with_mobc;
use sample_actix_with_redis::with_alt_r2d2;
use sample_actix_with_redis::with_r2d2;

use uuid::Uuid;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/direct")]
async fn set_direct(client: web::Data<direct::DirectClient>) -> impl Responder {
    let id = Uuid::new_v4();
    let key = format!("{}", id);
    let value = "hi";
    let result = direct::set(&client, &key, value).await;
    match result {
        Ok(_) => HttpResponse::Ok().body(key),
        Err(e) => HttpResponse::InternalServerError().body(e.msg),
    }
}

#[get("/direct/{id}")]
async fn get_direct(
    client: web::Data<direct::DirectClient>,
    info: web::Path<(String,)>,
) -> impl Responder {
    let result = direct::get(&client, &info.0).await;
    match result {
        Ok(r) => HttpResponse::Ok().body(r),
        Err(e) => HttpResponse::NotFound().body(e.msg),
    }
}

#[get("/r2d2")]
async fn set_with_r2d2(pool: web::Data<with_r2d2::R2D2Pool>) -> impl Responder {
    let id = Uuid::new_v4();
    let key = format!("{}", id);
    let value = "hi";
    let result = with_r2d2::set(&pool, &key, value);
    match result {
        Ok(_) => HttpResponse::Ok().body(key),
        Err(e) => HttpResponse::InternalServerError().body(e.msg),
    }
}

#[get("/r2d2/{id}")]
async fn get_with_r2d2(
    pool: web::Data<with_r2d2::R2D2Pool>,
    info: web::Path<(String,)>,
) -> impl Responder {
    let result = with_r2d2::get(&pool, &info.0);
    match result {
        Ok(r) => HttpResponse::Ok().body(r),
        Err(e) => HttpResponse::NotFound().body(e.msg),
    }
}

#[get("/alt_r2d2")]
async fn set_with_alt_r2d2(pool: web::Data<with_alt_r2d2::AltR2D2Pool>) -> impl Responder {
    let id = Uuid::new_v4();
    let key = format!("{}", id);
    let value = "hi";
    let result = with_alt_r2d2::set(&pool, &key, value);
    match result {
        Ok(_) => HttpResponse::Ok().body(key),
        Err(e) => HttpResponse::InternalServerError().body(e.msg),
    }
}

#[get("/alt_r2d2/{id}")]
async fn get_with_alt_r2d2(
    pool: web::Data<with_alt_r2d2::AltR2D2Pool>,
    info: web::Path<(String,)>,
) -> impl Responder {
    let result = with_alt_r2d2::get(&pool, &info.0);
    match result {
        Ok(r) => HttpResponse::Ok().body(r),
        Err(e) => HttpResponse::NotFound().body(e.msg),
    }
}

#[get("/bb8")]
async fn set_with_bb8(pool: web::Data<with_bb8::BB8Pool>) -> impl Responder {
    let id = Uuid::new_v4();
    let key = format!("{}", id);
    let value = "hi";
    let result = with_bb8::set(&pool, &key, value).await;
    match result {
        Ok(_) => HttpResponse::Ok().body(key),
        Err(e) => HttpResponse::InternalServerError().body(e.msg),
    }
}

#[get("/bb8/{id}")]
async fn get_with_bb8(
    pool: web::Data<with_bb8::BB8Pool>,
    info: web::Path<(String,)>,
) -> impl Responder {
    let result = with_bb8::get(&pool, &info.0).await;
    match result {
        Ok(r) => HttpResponse::Ok().body(r),
        Err(e) => HttpResponse::NotFound().body(e.msg),
    }
}

#[get("/deadpool")]
async fn set_with_deadpool(pool: web::Data<with_deadpool::DeadpoolPool>) -> impl Responder {
    let id = Uuid::new_v4();
    let key = format!("{}", id);
    let value = "hi";
    let result = with_deadpool::set(&pool, &key, value).await;
    match result {
        Ok(_) => HttpResponse::Ok().body(key),
        Err(e) => HttpResponse::InternalServerError().body(e.msg),
    }
}

#[get("/deadpool/{id}")]
async fn get_with_deadpool(
    pool: web::Data<with_deadpool::DeadpoolPool>,
    info: web::Path<(String,)>,
) -> impl Responder {
    let result = with_deadpool::get(&pool, &info.0).await;
    match result {
        Ok(r) => HttpResponse::Ok().body(r),
        Err(e) => HttpResponse::NotFound().body(e.msg),
    }
}

#[get("/mobc")]
async fn set_with_mobc(pool: web::Data<with_mobc::MobcPool>) -> impl Responder {
    let id = Uuid::new_v4();
    let key = format!("{}", id);
    let value = "hi";
    let result = with_mobc::set(&pool, &key, value).await;
    match result {
        Ok(_) => HttpResponse::Ok().body(key),
        Err(e) => HttpResponse::InternalServerError().body(e.msg),
    }
}

#[get("/mobc/{id}")]
async fn get_with_mobc(
    pool: web::Data<with_mobc::MobcPool>,
    info: web::Path<(String,)>,
) -> impl Responder {
    let result = with_mobc::get(&pool, &info.0).await;
    match result {
        Ok(r) => HttpResponse::Ok().body(r),
        Err(e) => HttpResponse::NotFound().body(e.msg),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "redis://127.0.0.1/";
    let direct_client = direct::create_client(host).unwrap();
    let r2d2_pool = with_r2d2::create_pool(host).unwrap();
    let bb8_pool = with_bb8::create_pool(host).await.unwrap();
    let deadpool_pool = with_deadpool::create_pool(host).unwrap();
    let mobc_pool = with_mobc::create_pool(host);
    let alt_r2d2_pool = with_alt_r2d2::create_pool(host).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(direct_client.clone()))
            .app_data(web::Data::new(r2d2_pool.clone()))
            .app_data(web::Data::new(bb8_pool.clone()))
            .app_data(web::Data::new(deadpool_pool.clone()))
            .app_data(web::Data::new(mobc_pool.clone()))
            .app_data(web::Data::new(alt_r2d2_pool.clone()))
            .service(hello)
            .service(set_direct)
            .service(get_direct)
            .service(set_with_r2d2)
            .service(get_with_r2d2)
            .service(set_with_bb8)
            .service(get_with_bb8)
            .service(set_with_deadpool)
            .service(get_with_deadpool)
            .service(set_with_mobc)
            .service(get_with_mobc)
            .service(set_with_alt_r2d2)
            .service(get_with_alt_r2d2)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
