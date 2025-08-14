use axum::{routing::get, Router, Json};
use serde_json::{json, Value};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/products", get(get_products))
        .route("/products/search", get(search_products))
        .route("/products/:id", get(get_product))
        .route("/products/compare", get(compare_products))
        .route("/categories", get(get_categories))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("📦 Product Service running on port 8000");
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<Value> {
    Json(json!({"status": "healthy", "service": "product-service", "features": ["catalog", "search", "comparison"]}))
}

async fn get_products() -> Json<Value> {
    Json(json!({
        "products": [
            {"id": "prod_1", "name": "Laptop", "price": 999.99, "category": "Electronics"},
            {"id": "prod_2", "name": "Phone", "price": 699.99, "category": "Electronics"}
        ],
        "service": "product-service"
    }))
}

async fn search_products() -> Json<Value> {
    Json(json!({
        "results": [
            {"id": "prod_1", "name": "Laptop", "price": 999.99, "relevance": 0.95}
        ],
        "query": "laptop",
        "service": "product-service"
    }))
}

async fn get_product() -> Json<Value> {
    Json(json!({
        "product": {"id": "prod_1", "name": "Laptop", "price": 999.99, "description": "High-performance laptop"},
        "service": "product-service"
    }))
}

async fn compare_products() -> Json<Value> {
    Json(json!({
        "comparison": [
            {"vendor": "Amazon", "price": 999.99, "rating": 4.5},
            {"vendor": "Best Buy", "price": 1099.99, "rating": 4.3}
        ],
        "service": "product-service"
    }))
}

async fn get_categories() -> Json<Value> {
    Json(json!({
        "categories": ["Electronics", "Clothing", "Books", "Home"],
        "service": "product-service"
    }))
}
