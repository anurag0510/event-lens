mod graphql;

use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    Router,
    response::{self, IntoResponse},
    routing::get,
};
use elasticsearch::{
    Elasticsearch,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
};
use graphql::QueryRoot;
use std::sync::Arc;
use url::Url;

pub type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

struct AppState {
    schema: MySchema,
}

#[tokio::main]
async fn main() {
    // 1. Initialize Elasticsearch Transport
    let url = Url::parse("https://elastic:anurag0510@localhost:9200").unwrap();
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool)
        .disable_proxy()
        .cert_validation(elasticsearch::cert::CertificateValidation::None)
        .build()
        .unwrap();
    let es_client = Elasticsearch::new(transport);

    // 2. Health check on startup
    match es_client
        .cluster()
        .health(elasticsearch::cluster::ClusterHealthParts::None)
        .send()
        .await
    {
        Ok(response) => {
            println!("✅ ES health check status: {}", response.status_code());
            if let Ok(body) = response.json::<serde_json::Value>().await {
                println!("Cluster health: {}", body);
            }
        }
        Err(e) => {
            eprintln!("❌ ES health check failed: {}", e);
            eprintln!("⚠️  Continuing anyway, but queries may fail...");
        }
    }

    // 3. Build the GraphQL Schema with ES client in context
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(es_client)
        .finish();

    let app_state = Arc::new(AppState { schema: schema.clone() });

    // 4. Setup routes - use the GraphQL service from async-graphql-axum
    let app = Router::new()
        .route("/", get(graphql_playground))
        .route("/graphql", get(graphql_playground).post_service(GraphQL::new(schema)))
        .route("/health", get(health_check))
        .with_state(app_state);

    println!("🚀 GraphQL Query Service running:");
    println!("   GraphQL Playground: http://localhost:4000/");
    println!("   GraphQL Endpoint:   http://localhost:4000/graphql");
    println!("   Health Check:       http://localhost:4000/health");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "Query Service is healthy! ✅"
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
