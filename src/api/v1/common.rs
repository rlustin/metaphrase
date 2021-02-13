use crate::errors::MetaphraseError;
use crate::models::*;
use actix_web::HttpRequest;
use time::OffsetDateTime;

pub fn current_user(req: &HttpRequest) -> Result<User, MetaphraseError> {
    let extensions = req.extensions();
    let current_session = extensions.get::<Session>().unwrap();

    current_session.user()
}

pub fn now_str() -> String {
    OffsetDateTime::now_utc().format("%F %T")
}
