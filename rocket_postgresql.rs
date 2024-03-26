use rocket::serde::json::{Value, json};
use tokio_postgres::{NoTls, Error, Config};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/get_api")]
async fn get_api()-> Value{
    // println!("inside get api");
    let _client = fetch_query().await.unwrap();
    // println!("{:?}", client);
    // json!(client)
    json!("from get api")
}

async fn fetch_query() -> Result<serde_json::Value, Error>{

    println!("inisde fetch");

    /*
    let (_client, connection) =
        tokio_postgres::connect("host=localhost user=postgres dbname=tukang", NoTls)
        .await?;
    */


    let (client, connection) = Config::new()
        .host("localhost")
        .user("tukang_app")
        .port(5432)
        .password("tukang123")
        .dbname("tukang")
        .connect(NoTls)
        .await
        .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
  
    println!("after tokio");

    let rows = client.query("SELECT nm_category FROM category_pekerjaan", &[]).await?;

    for row in &rows {
        let cat: String = row.get("nm_category");

        println!("---->: {}", cat);
    }

    Ok(json!({"status": "ookkkk"}))

}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_api]).register("/", catchers![not_found])
}
