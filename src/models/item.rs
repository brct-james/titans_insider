use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::{engine::remote::ws::Client, Error};

use chrono::offset::Utc;
use shopsniffer::models::{IBasicResponseItemLast, Item, ItemLast};

#[allow(dead_code)]
pub async fn save_item_last_response_to_db(
    db: &Surreal<Client>,
    response: &IBasicResponseItemLast,
) -> Result<(), Box<dyn std::error::Error>> {
    match response.data.clone() {
        Some(items) => {
            let item_count = items.len();
            tracing::debug!(
                "SNIFFER: Started Saving All Items to DB, received {} items",
                item_count
            );
            for (i, item_last) in items.iter().enumerate() {
                if i % 500 == 0 {
                    tracing::debug!(
                        "SNIFFER Now Saving Item {:>5} of {:<5} ({:.2}%)",
                        i,
                        item_count,
                        i as f64 / item_count as f64 * 100.0
                    );
                }
                create_item_hist_entry(db, item_last).await?;
            }
            tracing::debug!("SNIFFER: Finished Saving All Items to DB");
            Ok(())
        }
        None => Err("No items found in response data")?,
    }
}

/* TODO */
// Need to write a function/query that creates the itemhist table and sets up relationships?

#[derive(Debug, Serialize, Deserialize)]
pub struct DBItem {
    #[serde(rename = "item_id")]
    pub item_id: f32,
    #[serde(rename = "tType")]
    pub t_type: String,
    #[serde(rename = "uid")]
    pub uid: String,
    #[serde(rename = "tag1", deserialize_with = "Option::deserialize")]
    pub tag1: Option<String>,
    #[serde(rename = "tag2", deserialize_with = "Option::deserialize")]
    pub tag2: Option<String>,
    #[serde(rename = "tag3", deserialize_with = "Option::deserialize")]
    pub tag3: Option<String>,
    #[serde(rename = "goldQty")]
    pub gold_qty: f32,
    #[serde(rename = "gemsQty")]
    pub gems_qty: f32,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "tier", deserialize_with = "Option::deserialize")]
    pub tier: Option<f32>,
    #[serde(rename = "order", deserialize_with = "Option::deserialize")]
    pub order: Option<f32>,
    #[serde(rename = "cityId", deserialize_with = "Option::deserialize")]
    pub city_id: Option<f32>,
    #[serde(rename = "goldPrice", deserialize_with = "Option::deserialize")]
    pub gold_price: Option<f32>,
    #[serde(rename = "gemsPrice", deserialize_with = "Option::deserialize")]
    pub gems_price: Option<f32>,
    #[serde(rename = "requestCycle")]
    pub request_cycle: f32,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "dbTimestamp")]
    pub db_timestamp: i64,
}

impl From<ItemLast> for DBItem {
    fn from(item: ItemLast) -> Self {
        DBItem {
            item_id: item.id,
            t_type: item.t_type,
            uid: item.uid,
            tag1: item.tag1,
            tag2: item.tag2,
            tag3: item.tag3,
            gold_qty: item.gold_qty,
            gems_qty: item.gems_qty,
            created: item.created,
            tier: item.tier,
            order: item.order,
            city_id: item.city_id,
            gold_price: item.gold_price,
            gems_price: item.gems_price,
            request_cycle: item.request_cycle_last,
            created_at: item.created_at,
            updated_at: item.updated_at,
            db_timestamp: Utc::now().timestamp(),
        }
    }
}

impl From<Item> for DBItem {
    fn from(item: Item) -> Self {
        DBItem {
            item_id: item.id,
            t_type: item.t_type,
            uid: item.uid,
            tag1: item.tag1,
            tag2: item.tag2,
            tag3: item.tag3,
            gold_qty: item.gold_qty,
            gems_qty: item.gems_qty,
            created: item.created,
            tier: item.tier,
            order: item.order,
            city_id: item.city_id,
            gold_price: item.gold_price,
            gems_price: item.gems_price,
            request_cycle: item.request_cycle,
            created_at: item.created_at,
            updated_at: item.updated_at,
            db_timestamp: Utc::now().timestamp(),
        }
    }
}

impl Into<Item> for DBItem {
    fn into(self) -> Item {
        Item {
            id: self.item_id,
            t_type: self.t_type,
            uid: self.uid,
            tag1: self.tag1,
            tag2: self.tag2,
            tag3: self.tag3,
            gold_qty: self.gold_qty,
            gems_qty: self.gems_qty,
            created: self.created,
            tier: self.tier,
            order: self.order,
            city_id: self.city_id,
            gold_price: self.gold_price,
            gems_price: self.gems_price,
            request_cycle: self.request_cycle,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

// pub fn convert_item_last_to_item(item_last: &ItemLast) -> Item {
//     Item::new(
//         item_last.id.clone(),
//         item_last.t_type.clone(),
//         item_last.uid.clone(),
//         item_last.tag1.clone(),
//         item_last.tag2.clone(),
//         item_last.tag3.clone(),
//         item_last.gold_qty.clone(),
//         item_last.gems_qty.clone(),
//         item_last.created.clone(),
//         item_last.tier.clone(),
//         item_last.order.clone(),
//         item_last.city_id.clone(),
//         item_last.gold_price.clone(),
//         item_last.gems_price.clone(),
//         item_last.request_cycle_last.clone(),
//         item_last.created_at.clone(),
//         item_last.updated_at.clone(),
//     )
// }

#[allow(dead_code)]
pub async fn create_item_hist_entry(
    db: &Surreal<Client>,
    item_last: &ItemLast,
) -> Result<Vec<DBItem>, Error> {
    let item = DBItem::from(item_last.clone());
    let record: Result<Vec<DBItem>, Error> = db.create("item_hist").content(&item).await;
    match record {
        Ok(res) => Ok(res),
        Err(err_res) => Err(err_res),
    }
}

pub async fn get_full_item_hist(db: &Surreal<Client>, item_id: &str) -> Result<Vec<DBItem>, Error> {
    let hist_res = db
        .query(format!("SELECT * FROM item_hist WHERE uid = '{item_id}'"))
        .await;
    match hist_res {
        Ok(mut res) => {
            let dbitems: Vec<DBItem> = res.take(0)?;
            return Ok(dbitems);
        }
        Err(err_res) => return Err(err_res),
    }
}

// #[allow(dead_code)]
// pub async fn replace_agent(db: &Surreal<Client>, agent: &Agent) -> Result<(), Error> {
//     db.update(("agent", agent.symbol.as_str()))
//         .content(agent)
//         .await?;
//     Ok(())
// }

// #[allow(dead_code)]
// pub async fn update_agent_credits(
//     db: &Surreal<Client>,
//     agent_symbol: &str,
//     new_credits: &i64,
// ) -> Result<(), Error> {
//     #[derive(Serialize)]
//     struct CreditsUpdate {
//         credits: i64,
//     }

//     db.update(("agent", agent_symbol))
//         .merge(CreditsUpdate {
//             credits: *new_credits,
//         })
//         .await?;
//     Ok(())
// }
