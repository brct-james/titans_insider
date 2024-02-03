use dotenv;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

use shopsniffer::apis::configuration::Configuration;
use shopsniffer::apis::item_api::item_swagger_get_slash_last_slash_all;

mod models;
mod types;
mod yaml_util;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "titans_insider=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::debug!("--Intializing Shopsniffer--");
    // Load env files
    tracing::debug!("--Loading Env Vars--");
    dotenv::from_filename("surreal_secrets.env").ok();
    let surreal_user = std::env::var("SURREAL_USER").expect("SURREAL_USER must be set");
    let surreal_pass = std::env::var("SURREAL_PASS").expect("SURREAL_PASS must be set");

    // Connect to the server
    tracing::debug!("--Connecting to DB--");
    let db = Surreal::new::<Ws>("localhost:8001")
        .await
        .expect("Could not connect to DB");

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: surreal_user.as_str(),
        password: surreal_pass.as_str(),
    })
    .await
    .expect("Could not sign in to DB");

    // Select a specific namespace / database
    db.use_ns("shopsniffer")
        .use_db("shopsniffer")
        .await
        .expect("Could not select shopsniffer namespace or db");

    tracing::debug!("--DB Connected and Ready--");

    tracing::debug!("--Loading Rules--");
    let staleness_rules: types::StalenessRules = yaml_util::load_staleness_rules();
    tracing::debug!("Staleness Rules: {:#?}", staleness_rules);

    tracing::debug!("--Creating Configuration--");
    let mut conf = Configuration::new();
    conf.base_path = String::from("https://smartytitans.com");

    tracing::debug!("--==Shopsniffer Ready==--");

    tracing::debug!("--Getting Last--");
    match item_swagger_get_slash_last_slash_all(&conf).await {
        Ok(res) => {
            models::item::save_item_last_response_to_db(&db, &res)
                .await
                .unwrap();
        }
        Err(err_res) => {
            panic!("{:#?}", err_res);
        }
    }
    tracing::debug!("--Finished Saving Last--");
    tracing::debug!("--Querying Item Hist for sultandagger--");
    match models::item::get_full_item_hist(&db, "sultandagger").await {
        Ok(mut res) => {
            tracing::debug!("--Found {:?} Historical Records--", res.len());
            res.sort_by(|a, b| {
                if a.t_type == b.t_type {
                    a.created_at.cmp(&b.created_at)
                } else {
                    a.t_type.cmp(&b.t_type)
                }
            });
            for (i, item) in res.iter().enumerate() {
                if i % 20 == 0 {
                    tracing::debug!("{}", "");
                    tracing::debug!(
                        "{:^5} | {:^7} | {:^24} | {:^19} | {:^19}",
                        "#",
                        "TYPE",
                        "TIMESTAMP",
                        "GOLD",
                        "GEMS"
                    );
                    tracing::debug!(
                        "{:^5} | {:^7} | {:^24} | {:^7} : {:^9} | {:^7} : {:^9}",
                        "",
                        "",
                        "",
                        "QTY",
                        "BESTPRICE",
                        "QTY",
                        "BESTPRICE"
                    );
                    tracing::debug!(
                        "{:-^5} | {:-^8}|{:-^26}|{:-^9}:{:-^11}|{:-^9}:{:-^11}",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        ""
                    );
                }
                let type_string = match item.t_type.as_str() {
                    "r" => "request",
                    "o" => "offer",
                    _ => "other",
                };
                tracing::debug!(
                    "{:^5} | {:^7} | {:^24} | {:>7} : {:<9} | {:>7} : {:<9}",
                    i,
                    type_string,
                    item.created_at,
                    item.gold_qty,
                    item.gold_price.unwrap_or(0.0),
                    item.gems_qty,
                    item.gems_price.unwrap_or(0.0)
                );
            }
        }
        Err(err_res) => {
            panic!("{:#?}", err_res);
        }
    }
    tracing::debug!("--==Closing==--");
}
