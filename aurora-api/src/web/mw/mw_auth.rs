use crate::{
    ctx::Ctx,
    model::{session::service::_get_session, user::service::_get_user},
    web::{set_token_cookie, AUTH_TOKEN},
};
use aurora_common::{
    core_error::error::{AuroraData, Error},
    core_results::results::Result,
};
// use crate::web::{Error, Result};
use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, Request},
    middleware::Next,
    response::Response,
};
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::info;

#[allow(dead_code)] // For now, until we have the rpc.
pub async fn mw_ctx_require<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    info!("{:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

    let _ctx = ctx?;
    info!("{:<12} - mw_ctx_require - {_ctx:?}", "MIDDLEWARE");

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolve<B>(
    // mm: State<ModelManager>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    info!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

    let ctx_ext_result = _ctx_resolve(&cookies).await;

    if ctx_ext_result.is_err()
        && !matches!(
            ctx_ext_result,
            Err(Error::LoginSessionFailed(AuroraData::Null, None))
        )
    {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    // Store the ctx_ext_result in the request extension
    // (for Ctx extractor).
    req.extensions_mut().insert(ctx_ext_result);

    Ok(next.run(req).await)
}

async fn _ctx_resolve(
    // mm: State<ModelManager>,
    cookies: &Cookies,
) -> CtxExtResult {
    // -- Get Token String
    let session_id = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(Error::LoginSessionFailed(AuroraData::Null, None))?;

    let session = _get_session(session_id).await?;
    let user = _get_user(session.user_id).await?;
    // -- Parse Token
    // let token: Token = token.parse().map_err(|_| Error::LoginSessionFailed)?;

    // -- Get UserForAuth
    // let user: UserForAuth = UserBmc::first_by_username(&Ctx::root_ctx(), &token.ident)
    //     .await
    //     .map_err(|ex| CtxExtError::ModelAccessError(ex.to_string()))?
    //     .ok_or(CtxExtError::UserNotFound)?;

    // -- Validate Token
    // validate_web_token(&token, &user.token_salt.to_string())
    //     .map_err(|_| CtxExtError::FailValidate)?;

    // -- Update Token
    set_token_cookie(cookies, session.id.as_str())
        .map_err(|_| Error::LoginSessionFailed(AuroraData::Null, None))?;

    // -- Create CtxExtResult
    Ctx::new(user.id).map_err(|_ex| Error::LoginSessionFailed(AuroraData::Null, None))
}

// region:    --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        info!("{:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::UserNotExist(AuroraData::Null, None))?
            .clone()
            .map_err(|_| Error::UserNotExist(AuroraData::Null, None))
    }
}
// endregion: --- Ctx Extractor
type CtxExtResult = core::result::Result<Ctx, Error>;
