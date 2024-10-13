use axum::{
    extract::{ConnectInfo, Extension},
    routing::get,
    Router,
    Json,

};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use serde_json::json;
use std::sync::Arc;
use ip2region::Searcher;


#[tokio::main]
async fn main() {
    // 获取当前目录
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {:?}", current_dir);
    // 构造数据库文件的路径
    let db_path = current_dir.join("ip2region.xdb");
    println!("Database path: {:?}", db_path);

    // 初始化加载xdb文件
    // searcher_init(Some(db_path.to_str().unwrap().to_owned()));

    let searcher = Arc::new(Searcher::new(db_path).expect("Failed to create IP searcher"));

    // Build our application with a route
    let app = Router::new()
        .route("/", get(hello))
        .route("/ip", get(get_ip_info))
        .with_state(searcher);

    // Run it
    // let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // println!("listening on {}", listener.local_addr().unwrap());
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn hello() -> &'static str {
    "Hello, world!"
}

async fn get_ip_info(
    Extension(ConnectInfo(addr)): Extension<ConnectInfo<SocketAddr>>,
    axum::extract::State(searcher): axum::extract::State<Arc<Searcher>>,
) -> Json<serde_json::Value> {
    let ip = addr.ip().to_string();
    // println!("ip: {}", ip);
    match searcher.search(&ip) {
        Ok(location) => {
            // 将返回的字符串分割成各个部分
            let parts: Vec<&str> = location.split('|').collect();
            Json(json!({
                "ip": ip,
                "region": location,
                "country": parts.get(0).unwrap_or(&""),
                "province": parts.get(2).unwrap_or(&""),
                "city": parts.get(3).unwrap_or(&""),
                "isp": parts.get(4).unwrap_or(&""),
            }))
        },
        Err(e) => Json(json!({ "error": format!("Unable to locate IP: {}", e) })),
    }
}