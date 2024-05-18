use salvo::prelude::*;

#[handler]
async fn hello() -> &'static str {
    "hello world"
}
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let router = Router::new().get(hello);
    let accepter = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(accepter).serve(router).await;
}
