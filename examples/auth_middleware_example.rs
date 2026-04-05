/// 本示例展示如何在 Actix Web 应用中注册认证中间件 (AuthMiddleware)
/// 并在路由处理中使用用户信息。
///
/// 运行方式:
/// cargo run --example auth_middleware_example --features "model,common"

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use log::info;

// 注意：需要启用以下 features: model, common
use rust_wheel::common::util::net::auth_middleware::AuthMiddleware;
use rust_wheel::common::util::net::context_util::ContextUtil;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

/// 示例处理程序1: 使用 LoginUserInfo 作为参数注入
/// 该处理程序会自动从上下文中获取用户信息
async fn get_user_profile(user: LoginUserInfo) -> impl Responder {
    info!("获取用户资料: {:?}", user);
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": {
            "appId": user.appId,
            "userId": user.userId,
            "account": user.account,
        }
    }))
}

/// 示例处理程序2: 在处理程序中手动获取用户信息
/// 通过 ContextUtil::current_user() 从异步上下文中获取用户
async fn get_current_user_info() -> impl Responder {
    match ContextUtil::current_user() {
        Ok(user) => {
            info!("获取当前用户: {:?}", user);
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "userId": user.userId,
                "account": user.account,
            }))
        }
        Err(e) => {
            info!("获取用户信息失败: {}", e);
            HttpResponse::Unauthorized().json(serde_json::json!({
                "status": "error",
                "message": "未授权或用户信息未在上下文中"
            }))
        }
    }
}

/// 示例处理程序3: 获取用户账户信息
async fn get_user_account(user: LoginUserInfo) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "userId": user.userId,
        "account": user.account,
    }))
}

/// 健康检查端点（不需要认证）
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let server_address = "127.0.0.1";
    let server_port = 8080;

    info!("启动服务器: {}:{}", server_address, server_port);

    HttpServer::new(|| {
        App::new()
            // ============== 重要: 注册认证中间件 ==============
            // 该中间件必须在路由定义之前注册，以便对所有请求进行处理
            .wrap(AuthMiddleware)
            // ================================================

            // 不需要认证的路由
            .route("/health", web::get().to(health_check))

            // 需要认证的路由
            // 路由1: 使用 LoginUserInfo 参数进行自动注入
            .route("/user/profile", web::get().to(get_user_profile))

            // 路由2: 在处理程序中手动获取用户
            .route(
                "/user/current-info",
                web::get().to(get_current_user_info),
            )

            // 路由3: 获取用户账户
            .route("/user/account", web::get().to(get_user_account))
    })
    .bind(format!("{}:{}", server_address, server_port))?
    .run()
    .await
}

// ============== 使用说明 ==============
//
// 1. 启动服务器后，需要先获取访问令牌:
//    - 通过登录接口获取 JWT token
//    - 保存 token 以便后续使用
//
// 2. 调用受保护的端点:
//
//    方式A: 使用 Authorization header:
//    $ curl -H "Authorization: Bearer YOUR_JWT_TOKEN" \
//           http://localhost:8080/user/profile
//
//    方式B: 使用查询参数 access_token:
//    $ curl http://localhost:8080/user/profile?access_token=YOUR_JWT_TOKEN
//
//    方式C: 使用 Traefik 转发的 X-Forwarded-Uri header:
//    $ curl -H "X-Forwarded-Uri: /user/profile?access_token=YOUR_JWT_TOKEN" \
//           http://localhost:8080/user/profile
//
// 3. 调用健康检查（无需认证）:
//    $ curl http://localhost:8080/health
//
// ======================================
//
// 中间件工作流程:
//
// 1. 请求到达 AuthMiddleware
// 2. 中间件提取 JWT token (从 header、查询参数或 Traefik 转发)
// 3. 验证 token 的有效性和过期时间
// 4. 从 token 解析用户信息并创建 LoginUserInfo 实例
// 5. 使用 ContextUtil::with_user() 将用户信息存入任务本地上下文
// 6. 将请求转发到具体的处理程序
// 7. 处理程序可以通过以下方式访问用户信息:
//    - 作为函数参数: async fn handler(user: LoginUserInfo)
//    - 通过 ContextUtil: ContextUtil::current_user()
//
// ======================================
