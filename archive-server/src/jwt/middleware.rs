use actix_web::FromRequest;

use super::Jwt;

pub struct JWTMiddleware {
    pub jwt: Jwt,
}

impl JWTMiddleware {
    pub fn new(jwt: Jwt) -> Self {
        Self { jwt }
    }
}

impl FromRequest for JWTMiddleware {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let token = match req.cookie("AuthToken") {
            Some(cookie) => cookie.value().to_string(),
            None => {
                return std::future::ready(Err(actix_web::error::ErrorUnauthorized(
                    "Cookie not found",
                )))
            }
        };

        let jwt = match Jwt::decode(&token) {
            Ok(jwt) => jwt,
            Err(error) => {
                eprintln!("Error while decoding jwt\n{:?}", error);
                return std::future::ready(Err(actix_web::error::ErrorUnauthorized(
                    "Invalid cookie",
                )));
            }
        };

        std::future::ready(Ok(JWTMiddleware::new(jwt)))
    }
}
