use axum::{ routing::{ get, post, put, delete }, Router, Json, extract::Path, http::StatusCode };
use serde::{ Deserialize, Serialize };
use once_cell::sync::Lazy;
use std::sync::{ Arc, Mutex };

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Product {
    id: String,
    name: String,
    category: String,
    quantity: f64,
    price: u32,
}

type Db = Arc<Mutex<Vec<Product>>>;

static DB: Lazy<Db> = Lazy::new(|| {
    Arc::new(
        Mutex::new(
            vec![
                Product {
                    id: "1".into(),
                    name: "Quantum Rice".into(),
                    category: "Grains".into(),
                    quantity: 8.0,
                    price: 89,
                },
                Product {
                    id: "2".into(),
                    name: "Neon CBD Oil".into(),
                    category: "Liquids".into(),
                    quantity: 120.0,
                    price: 3,
                },
                Product {
                    id: "3".into(),
                    name: "Cyber Beans".into(),
                    category: "Legumes".into(),
                    quantity: 45.0,
                    price: 67,
                },
                Product {
                    id: "4".into(),
                    name: "Plasma Salt".into(),
                    category: "Seasoning".into(),
                    quantity: 159.0,
                    price: 1,
                },
                Product {
                    id: "26b3".into(),
                    name: "test fuck".into(),
                    category: "nigga".into(),
                    quantity: 4.2,
                    price: 69,
                }
            ]
        )
    )
});

async fn get_products() -> Json<Vec<Product>> {
    let db = DB.lock().unwrap();
    Json(db.clone())
}

async fn add_product(mut payload: Json<Product>) -> (StatusCode, Json<Product>) {
    let mut db = DB.lock().unwrap();
    let new_id = (db.len() + 1).to_string();
    payload.id = new_id.clone();
    db.push(payload.0.clone());
    (StatusCode::CREATED, Json(payload.0))
}

async fn update_product(
    Path(id): Path<String>,
    payload: Json<Product>
) -> Result<Json<Product>, StatusCode> {
    let mut db = DB.lock().unwrap();
    if let Some(pos) = db.iter().position(|p| p.id == id) {
        let mut updated = payload.0;
        updated.id = id.clone();
        db[pos] = updated.clone();
        Ok(Json(updated))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_product(Path(id): Path<String>) -> StatusCode {
    let mut db = DB.lock().unwrap();
    let len_before = db.len();
    db.retain(|p| p.id != id);
    if db.len() < len_before {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/cfs-api", get(get_products).post(add_product))
        .route("/cfs-api/:id", put(update_product).delete(delete_product));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
