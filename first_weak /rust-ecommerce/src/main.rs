use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Product {
    id: i32,
    name: String,
    price: i32,
}

#[derive(Deserialize)]
struct NewProduct {
    name: String,
    price: i32,
}

fn init_db() {
    let conn = Connection::open("database.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            price INTEGER NOT NULL
        )",
        [],
    ).unwrap();
}

async fn get_products() -> impl Responder {
    let conn = Connection::open("database.db").unwrap();

    let mut stmt = conn
        .prepare("SELECT id, name, price FROM products")
        .unwrap();

    let product_iter = stmt
        .query_map([], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                price: row.get(2)?,
            })
        })
        .unwrap();

    let mut products = Vec::new();

    for product in product_iter {
        products.push(product.unwrap());
    }

    HttpResponse::Ok().json(products)
}

async fn add_product(product: web::Json<NewProduct>) -> impl Responder {
    let conn = Connection::open("database.db").unwrap();

    conn.execute(
        "INSERT INTO products (name, price) VALUES (?1, ?2)",
        params![product.name, product.price],
    ).unwrap();

    HttpResponse::Ok().body("Product added")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    init_db();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/products", web::get().to(get_products))
            .route("/add", web::post().to(add_product))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
async fn index() -> impl Responder {
    let html = std::fs::read_to_string("templates/index.html").unwrap();
    HttpResponse::Ok().content_type("text/html").body(html)
}

