# 认证中间件 (AuthMiddleware) 注册指南

## 概述

`AuthMiddleware` 是一个 Actix Web 中间件，用于处理 JWT 认证和用户信息管理。它自动：

1. 从请求中提取 JWT token（支持多种方式）
2. 验证 token 的有效性和过期时间
3. 从 token 中解析用户信息
4. 将用户信息设置到异步上下文中，便于其他方法调用

---

## 快速开始

### 步骤 1: 在应用中注册中间件

```rust
use actix_web::{web, App, HttpServer};
use rust_wheel::common::util::net::auth_middleware::AuthMiddleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // 注册认证中间件
            .wrap(AuthMiddleware)
            
            // 然后定义你的路由...
            .route("/user/profile", web::get().to(handle_profile))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### 步骤 2: 在处理程序中使用用户信息

**方式 A: 自动注入（推荐）**

```rust
use rust_wheel::model::user::login_user_info::LoginUserInfo;

async fn handle_profile(user: LoginUserInfo) -> impl Responder {
    // user 自动从上下文中获取
    HttpResponse::Ok().json(serde_json::json!({
        "userId": user.userId,
        "account": user.account,
    }))
}
```

**方式 B: 手动获取**

```rust
use rust_wheel::common::util::net::context_util::ContextUtil;

async fn handle_profile() -> impl Responder {
    match ContextUtil::current_user() {
        Ok(user) => {
            // 获取用户成功
            HttpResponse::Ok().json(user)
        }
        Err(_) => {
            // 用户未认证或信息不在上下文中
            HttpResponse::Unauthorized().finish()
        }
    }
}
```

---

## Token 获取方式

中间件支持以下三种方式获取 JWT token：

### 1. Authorization Header（标准方式）

最推荐的方式，符合 HTTP 标准：

```bash
curl -H "Authorization: Bearer YOUR_JWT_TOKEN" \
     http://localhost:8080/user/profile
```

**代码实现：**
```rust
pub fn get_auth_header(req: &HttpRequest) -> Option<String> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(header_value) = auth_header.to_str() {
            if header_value.starts_with("Bearer ") {
                let token = header_value.trim_start_matches("Bearer ");
                return Some(token.to_string());
            }
        }
    }
    None
}
```

### 2. 查询参数（access_token）

当无法使用 header 时的备选方案：

```bash
curl http://localhost:8080/user/profile?access_token=YOUR_JWT_TOKEN
```

**代码实现：**
```rust
fn get_params_access_token(request: &HttpRequest) -> Option<String> {
    let q_str = request.query_string();
    if q_str.is_empty() {
        return None;
    }
    let params = Query::<HashMap<String, String>>::from_query(request.query_string()).ok()?;
    params.get("access_token").map(|s| s.to_owned())
}
```

### 3. Traefik 转发的 Header（X-Forwarded-Uri）

用于在 Traefik 反向代理后面使用：

```bash
curl -H "X-Forwarded-Uri: /user/profile?access_token=YOUR_JWT_TOKEN" \
     http://localhost:8080/user/profile
```

**代码实现：**
```rust
fn get_forward_params_access_token(request: &HttpRequest) -> Option<String> {
    let x_header = request.headers().get("X-Forwarded-Uri")?;
    let x_header_str = x_header.to_str().ok()?;
    
    let key_value_pairs: Vec<&str> = x_header_str.split('?').collect();
    let pairs = key_value_pairs.get(1)?;
    
    for pair in pairs.split('&') {
        if pair.contains("access_token=") {
            let parts: Vec<&str> = pair.split('=').collect();
            if let Some(token) = parts.get(1) {
                return Some(token.to_string());
            }
        }
    }
    None
}
```

---

## 中间件工作流程

```
┌─────────────────┐
│   HTTP 请求     │
└────────┬────────┘
         │
         ▼
┌──────────────────────────┐
│   AuthMiddleware         │
│  --------------------   │
│  1. 提取 JWT token       │
│  2. 验证 token 有效性    │
│  3. 解析用户信息         │
│  4. 设置到上下文         │
└────────┬─────────────────┘
         │
         ▼
┌──────────────────────────┐
│   路由处理程序           │
│  --------------------   │
│  - 自动注入 LoginUserInfo│
│  - 或手动获取用户信息    │
└────────┬─────────────────┘
         │
         ▼
┌─────────────────────────┐
│      HTTP 响应          │
└─────────────────────────┘
```

---

## 错误处理

当认证失败时，中间件返回 401 Unauthorized：

```json
{
    "status": "fail",
    "message": "the user belonging to this token no longer exists"
}
```

### 常见错误场景

| 错误 | 原因 | 解决方法 |
|-----|------|--------|
| Token 格式错误 | Authorization header 不是 "Bearer ..." 格式 | 检查 header 格式 |
| Token 过期 | JWT 中的 exp 时间已过 | 重新获取新 token |
| Token 签名无效 | JWT_SECRET 不匹配 | 检查环境变量 JWT_SECRET |
| 用户不存在 | Token 中的用户在系统中已被删除 | 重新登录获取新 token |

---

## 环境变量配置

确保设置以下环境变量：

```bash
# .env 文件
JWT_SECRET=your_secret_key_here
```

在应用启动时，从环境变量读取：

```rust
pub fn create_access_token(jwt_payload: &WebJwtPayload) -> String {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = &EncodingKey::from_secret(jwt_secret.as_ref());
    let token = encode(&Header::default(), &jwt_payload, key);
    token.unwrap()
}
```

---

## 完整示例

查看 `examples/auth_middleware_example.rs` 获取完整的工作示例：

```bash
cargo run --example auth_middleware_example --features "model,common"
```

---

## 高级用法

### 在其他异步方法中获取用户信息

中间件使用 Tokio 的任务本地存储（task-local storage），用户信息在整个请求的异步执行链中可用：

```rust
async fn some_async_helper() -> Result<String> {
    // 在任何异步函数中都可以获取用户信息
    let user = ContextUtil::current_user()?;
    
    // 使用用户信息进行业务逻辑处理
    Ok(format!("Hello, {}", user.userId))
}

async fn handler(user: LoginUserInfo) -> impl Responder {
    // 调用异步助手函数
    let greeting = some_async_helper().await.unwrap_or_default();
    
    HttpResponse::Ok().body(greeting)
}
```

### 自定义路由权限

可以在处理程序中检查特定的用户属性：

```rust
async fn handle_vip_only(user: LoginUserInfo) -> impl Responder {
    if !user.isVip {
        return HttpResponse::Forbidden().finish();
    }
    
    // VIP 用户可以访问的逻辑...
    HttpResponse::Ok().finish()
}
```

---

## 常见问题

**Q: 如何跳过某些路由的认证？**

A: 中间件会处理所有请求。如果需要跳过某些路由，可以：

1. 在 AuthMiddleware 中添加白名单逻辑
2. 或者使用可选的 LoginUserInfo 参数

**Q: 支持刷新 token 吗？**

A: 当前实现中，token 验证只检查过期时间。可以通过调用登录接口重新获取新 token。建议添加刷新 token 端点以改善用户体验。

**Q: 如何处理并发请求中的用户隔离？**

A: 中间件使用 Tokio 的任务本地存储，每个异步任务（包括并发请求）有自己独立的上下文，自动实现了隔离。

---

## 总结

| 功能 | 代码 |
|------|------|
| 注册中间件 | `.wrap(AuthMiddleware)` |
| 自动注入用户 | `async fn handler(user: LoginUserInfo)` |
| 手动获取用户 | `ContextUtil::current_user()` |
| 创建 token | `create_access_token(&payload)` |
| 验证 token | `verify_jwt_token(&token)` |
