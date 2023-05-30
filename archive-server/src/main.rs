use actix_web::{
    cookie::{time::Duration, CookieBuilder},
    get, post, App, HttpResponse, HttpServer, Responder,
};
use cache::Cache;
use serde_derive::{Deserialize, Serialize};
use jwt::{Jwt, JWTMiddleware};

#[derive(Serialize, Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String
}

mod jwt;
mod cache;

fn users_cache() -> Cache<User> {
    Cache::new("users".to_string())
}

#[post("/login")]
async fn login(body: actix_web::web::Json<LoginData>) -> impl Responder {
    let users = users_cache().get();
    let user = users.iter().find(|user| {
        user.username == body.username && user.password == body.password
    });

    let user = match user {
        None => return HttpResponse::Unauthorized().finish(),
        Some(user) => user
    };

    let token = Jwt::new(user.username.clone());
    let token = match token.encode() {
        Ok(token) => token,
        Err(error) => {
            eprintln!("Error while encoding jwt\n{:?}", error);
            return HttpResponse::InternalServerError().finish()
        }
    };

    HttpResponse::Ok()
        .cookie(
            CookieBuilder::new("AuthToken", token)
                .max_age(Duration::new(60, 0))
                .http_only(true)
                .finish(),
        )
        .finish()
}

#[post("/register")]
async fn register(body: actix_web::web::Json<LoginData>) -> impl Responder {
    let mut users = users_cache().get();
    let user = users.iter().find(|user| {
        user.username == body.username
    });

    if user.is_some() {
        return HttpResponse::Conflict().finish();
    }

    users.push(User {
        username: body.username.clone(),
        password: body.password.clone()
    });

    users_cache().save(users);

    HttpResponse::Ok().finish()
}



#[get("/secret")]
async fn secret(middleware: JWTMiddleware) -> impl Responder {
    HttpResponse::Ok().body(format!("Secretly welcome {}", middleware.jwt.username))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Cache::init();
    HttpServer::new(|| App::new().service(login).service(register).service(secret))
        .bind(("127.0.0.1", 4000))?
        .run()
        .await
}
