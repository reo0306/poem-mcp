use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route, Server};
use poem_mcpserver::{sse::sse_endpoint, tool::Text, McpServer, Tools};

struct Counter {
    count: i32,
}

#[Tools]
impl Counter {
    async fn increment(&mut self) -> Text<i32> {
        self.count += 1;
        Text(self.count)
    }

    async fn decrement(&mut self) -> Text<i32> {
        self.count -= 1;
        Text(self.count)
    }

    async fn get_value(&self) -> Text<i32> {
        Text(self.count)
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000");
    let app = Route::new()
            .at(
                "/sse",
                sse_endpoint(|_| McpServer::new().tools(Counter { count: 0 })),
            )
            .with(Cors::new());
        Server::new(listener).run(app).await
}
