use rocket::{routes, State};
use tokio_postgres::{NoTls, Error, Config};

#[macro_use] extern crate rocket;

// In a real application, this would likely be more complex.
struct MyConfig {
    //user_val: String,
    db_connect:tokio_postgres::Client,
}

#[get("/")]
async fn index(state: &State<MyConfig>){
    // MyConfig { user_val: "after change ".to_string() };
    // format!("The config value is: {}", state.user_val)

    let rows = state.db_connect.query("SELECT nm_category FROM category_pekerjaan", &[]).await.unwrap();
    
    for row in &rows {
        let cat: String = row.get("nm_category");

        println!("---->: {}", cat);
    }
}

#[get("/raw")]
async fn raw_config_value(state: &State<MyConfig>){
    // &state.user_val
    let rows = state.db_connect.query("SELECT nm_sub_category FROM sub_category_pekerjaan", &[]).await.unwrap();
    
    for row in &rows {
        let cat: String = row.get("nm_sub_category");

        println!("---->: {}", cat);
    }
}

async fn init_db() -> Result<tokio_postgres::Client, Error> {
    let (client, connection) = Config::new()
            .host("localhost")
            .user("postgres")
            .port(5432)
            // .password("")
            .dbname("tukang")
            .connect(NoTls)
            .await
            .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("database connection error: {}", e);
        }
    });

    Ok(client)

}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index,raw_config_value])
        .manage(MyConfig{db_connect:init_db().await.unwrap()})
}
