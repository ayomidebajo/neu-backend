use actix_web::{HttpResponse, ResponseError};

// TODO: Implemt error handling well

use postgres::Error;
    use derive_more::Display;
    // use tokio_pg_mapper::Error as PGMError;
    use postgres::error::Error as PGError;

    #[derive(Display, Debug)]
    pub enum MyError {
        NotFound,
        PGError(PGError),
        // PGMError(PGMError),
        PoolError(Error),
    }
    impl std::error::Error for MyError {}

    impl ResponseError for MyError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                MyError::NotFound => HttpResponse::NotFound().finish(),
                MyError::PoolError(ref err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }