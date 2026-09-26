use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::LocalBoxFuture;
use log::{debug, error};
use std::{
    future::{ready, Ready},
    rc::Rc,
};

use crate::common::util::net::context_util::ContextUtil;
use crate::model::user::{jwt_auth, login_user_info::LoginUserInfo};

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // Extract token
            let token = jwt_auth::get_auth_token(req.request());
            debug!("AuthMiddleware: extracted token, is_empty={}", token.is_empty());
            if token.is_empty() {
                // If no token, proceed without setting user
                debug!("AuthMiddleware: token is empty, proceeding without user");
                return service.call(req).await;
            }

            // The token decides who the caller is and, through the `et` claim,
            // how many projects they may create, so it has to be a token we
            // signed that is still valid. Decoding the payload on its own would
            // let anyone hand-craft a token for any user with any expiry.
            let payload_json = match jwt_auth::verify_and_decode_jwt_token(&token) {
                Ok(claims) => claims,
                Err(err) => {
                    // A rejected token is not fatal here: the request just
                    // carries no user, and `LoginUserInfo` turns that into a 401
                    // so the client can refresh its token and retry.
                    debug!("AuthMiddleware: token rejected, err={:?}", err);
                    return service.call(req).await;
                }
            };
            let payload_claims = match payload_json.as_object() {
                Some(claims) => claims,
                None => {
                    error!("AuthMiddleware: token payload is not a json object");
                    return service.call(req).await;
                }
            };

            let user_id = payload_claims.get("userId").and_then(|v| v.as_i64());
            let app_id = payload_claims.get("appId").and_then(|v| v.as_str());
            let device_id = payload_claims.get("deviceId").and_then(|v| v.as_str());
            let vip_expire_time = payload_claims.get("et").and_then(|v| v.as_i64()).unwrap_or_default();

            debug!("AuthMiddleware: extracted fields - user_id={:?}, app_id={:?}, device_id={:?}, vip_expire_time={}", user_id, app_id, device_id, vip_expire_time);

            if user_id.is_none() || app_id.is_none() || device_id.is_none() {
                error!("AuthMiddleware: required fields missing - user_id.is_none()={}, app_id.is_none()={}, device_id.is_none()={}", user_id.is_none(), app_id.is_none(), device_id.is_none());
                return service.call(req).await;
            }

            let x_request_id = req
                .headers()
                .get("x-request-id")
                .and_then(|h| h.to_str().ok())
                .unwrap_or(&uuid::Uuid::new_v4().to_string())
                .to_string();

            let login_user_info = LoginUserInfo {
                token: token.to_string(),
                userId: user_id.unwrap(),
                appId: app_id.unwrap().to_string(),
                xRequestId: x_request_id.clone(),
                deviceId: device_id.unwrap().to_string(),
                vipExpireTime: vip_expire_time,
            };

            debug!("AuthMiddleware: LoginUserInfo created - userId={}, appId={}, xRequestId={}, deviceId={}, vipExpireTime={}", 
                login_user_info.userId, login_user_info.appId, login_user_info.xRequestId, login_user_info.deviceId, login_user_info.vipExpireTime);

            // Set user in context and call service
            ContextUtil::with_user(login_user_info, service.call(req)).await
        })
    }
}