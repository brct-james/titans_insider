use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenv;

use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

use shopsniffer::apis::configuration::Configuration;
use shopsniffer::apis::item_api::item_swagger_get_slash_last_slash_all;

mod models;
mod rules;
mod schema;
mod types;
mod yaml_util;

static POLLING_RATE_SECONDS: u64 = 24;

pub fn get_connection_pool(url: String) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

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
    dotenv::from_filename("postgres_secrets.env").ok();
    let postgres_user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let postgres_password =
        std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let postgres_port = std::env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set");
    let postgres_db = std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");

    // Connect to the server
    tracing::debug!("--Connecting to DB--");
    let dburl = format!(
        "postgresql://{postgres_user}:{postgres_password}@localhost:{postgres_port}/{postgres_db}"
    );
    let pool = get_connection_pool(dburl);

    tracing::debug!("--DB Connected and Ready--");

    tracing::debug!("--Loading Rules--");
    let staleness_rules: types::StalenessRules = yaml_util::load_staleness_rules();
    tracing::debug!("Staleness Rules: {:#?}", staleness_rules);

    tracing::debug!("--Creating Configuration--");
    let mut conf = Configuration::new();
    conf.base_path = String::from("https://smartytitans.com");

    tracing::debug!("--==Shopsniffer Ready==--");

    tracing::debug!("--Spawning Sniffer Process--");
    let sniffer_pool = pool.clone();
    let sniffer_conf = conf.clone();
    let sniffer = tokio::task::spawn(async move {
        let mut interval =
            tokio::time::interval(std::time::Duration::from_secs(POLLING_RATE_SECONDS));
        loop {
            interval.tick().await;

            tracing::debug!("--SNIFFER: Getting Items--");
            match item_swagger_get_slash_last_slash_all(&sniffer_conf).await {
                Ok(res) => {
                    models::item_hist::save_item_last_response_to_db(
                        &mut sniffer_pool.get().unwrap(),
                        &res,
                    )
                    .await
                    .unwrap();
                }
                Err(err_res) => {
                    panic!("{:#?}", err_res);
                }
            }
            tracing::debug!("--SNIFFER: Finished Saving Items--");
        }
    });
    tracing::debug!("--Sniffer Process Spawned Successfully--");
    sniffer.await.unwrap();
    tracing::debug!("--Sniffer Process Died--");

    // tracing::debug!("--Querying Item Hist for sultandagger--");
    // match models::item::get_full_item_hist(&db, "sultandagger").await {
    //     Ok(mut res) => {
    //         tracing::debug!("--Found {:?} Historical Records--", res.len());
    //         res.sort_by(|a, b| {
    //             if a.t_type == b.t_type {
    //                 a.created_at.cmp(&b.created_at)
    //             } else {
    //                 a.t_type.cmp(&b.t_type)
    //             }
    //         });
    //         for (i, item) in res.iter().enumerate() {
    //             if i % 20 == 0 {
    //                 tracing::debug!("{}", "");
    //                 tracing::debug!(
    //                     "{:^5} | {:^7} | {:^24} | {:^19} | {:^19}",
    //                     "#",
    //                     "TYPE",
    //                     "TIMESTAMP",
    //                     "GOLD",
    //                     "GEMS"
    //                 );
    //                 tracing::debug!(
    //                     "{:^5} | {:^7} | {:^24} | {:^7} : {:^9} | {:^7} : {:^9}",
    //                     "",
    //                     "",
    //                     "",
    //                     "QTY",
    //                     "BESTPRICE",
    //                     "QTY",
    //                     "BESTPRICE"
    //                 );
    //                 tracing::debug!(
    //                     "{:-^5} | {:-^8}|{:-^26}|{:-^9}:{:-^11}|{:-^9}:{:-^11}",
    //                     "",
    //                     "",
    //                     "",
    //                     "",
    //                     "",
    //                     "",
    //                     ""
    //                 );
    //             }
    //             let type_string = match item.t_type.as_str() {
    //                 "r" => "request",
    //                 "o" => "offer",
    //                 _ => "other",
    //             };
    //             tracing::debug!(
    //                 "{:^5} | {:^7} | {:^24} | {:>7} : {:<9} | {:>7} : {:<9}",
    //                 i,
    //                 type_string,
    //                 item.created_at,
    //                 item.gold_qty,
    //                 item.gold_price.unwrap_or(0.0),
    //                 item.gems_qty,
    //                 item.gems_price.unwrap_or(0.0)
    //             );
    //         }
    //     }
    //     Err(err_res) => {
    //         panic!("{:#?}", err_res);
    //     }
    // }
    tracing::debug!("--==Closing==--");
}
