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

            // Decode token to get user info
            let parts: Vec<&str> = token.split('.').collect();
            debug!("AuthMiddleware: token parts count={}", parts.len());
            if parts.len() != 3 {
                error!("AuthMiddleware: invalid token format, parts.len()={}", parts.len());
                return service.call(req).await;
            }
            let payload_base64 = parts[1];
            let payload_bytes = match base64::decode(payload_base64) {
                Ok(b) => b,
                Err(e) => {
                    error!("AuthMiddleware: base64 decode failed, err={}", e);
                    return service.call(req).await;
                }
            };
            let payload_str = match String::from_utf8(payload_bytes) {
                Ok(s) => s,
                Err(e) => {
                    error!("AuthMiddleware: utf8 decode failed, err={}", e);
                    return service.call(req).await;
                }
            };
            debug!("AuthMiddleware: payload_str={}", payload_str);
            let payload_json: serde_json::Value = match serde_json::from_str(&payload_str) {
                Ok(v) => v,
                Err(e) => {
                    error!("AuthMiddleware: json parse failed, err={}", e);
                    return service.call(req).await;
                }
            };
            let payload_claims = match payload_json.as_object() {
                Some(o) => o,
                None => {
                    error!("AuthMiddleware: payload is not object");
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