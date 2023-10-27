#![allow(unused)]
pub(crate) mod bean;
pub(crate) mod mw;
pub(crate) mod routes_user;

// region:    --- Modules

use aurora_common::core_results::results::Result;
use tower_cookies::{Cookie, Cookies};

// endregion: --- Modules

pub const AUTH_TOKEN: &str = "sessionId";

fn set_token_cookie(cookies: &Cookies, session: &str) -> Result<()> {
    // let token = generate_web_token(user, salt)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, session.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");

    cookies.add(cookie);

    Ok(())
}

fn remove_token_cookie(cookies: &Cookies) -> Result<()> {
    let mut cookie = Cookie::named(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);

    Ok(())
}
