use crate::schema::item_hist;
use chrono::offset::Utc;
use diesel::prelude::*;
use diesel::result::Error;
use shopsniffer::models::{IBasicResponseItemLast, Item, ItemLast};
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::item_hist)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ItemHist {
    pub uuid: String,
    pub item_id: i32,
    pub t_type: String,
    pub uid: String,
    pub tag1: Option<String>,
    pub tag2: Option<String>,
    pub tag3: Option<String>,
    pub gold_qty: i32,
    pub gems_qty: i32,
    pub created: Option<String>,
    pub tier: Option<i32>,
    pub item_order: Option<i32>,
    pub city_id: Option<i32>,
    pub gold_price: i32,
    pub gems_price: i32,
    pub request_cycle: i32,
    pub created_at: String,
    pub updated_at: String,
    pub db_timestamp: i64,
}

impl From<ItemLast> for ItemHist {
    fn from(item: ItemLast) -> Self {
        ItemHist {
            uuid: Uuid::new_v4().to_string(),
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
            item_order: item.order,
            city_id: item.city_id,
            gold_price: item.gold_price.unwrap_or(0),
            gems_price: item.gems_price.unwrap_or(0),
            request_cycle: item.request_cycle_last,
            created_at: item.created_at,
            updated_at: item.updated_at,
            db_timestamp: Utc::now().timestamp_millis(),
        }
    }
}

impl From<Item> for ItemHist {
    fn from(item: Item) -> Self {
        ItemHist {
            uuid: Uuid::new_v4().to_string(),
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
            item_order: item.order,
            city_id: item.city_id,
            gold_price: item.gold_price.unwrap_or(0),
            gems_price: item.gems_price.unwrap_or(0),
            request_cycle: item.request_cycle,
            created_at: item.created_at,
            updated_at: item.updated_at,
            db_timestamp: Utc::now().timestamp_millis(),
        }
    }
}

impl Into<Item> for ItemHist {
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
            order: self.item_order,
            city_id: self.city_id,
            gold_price: Some(self.gold_price),
            gems_price: Some(self.gems_price),
            request_cycle: self.request_cycle,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

pub async fn save_item_last_response_to_db(
    conn: &mut PgConnection,
    response: &IBasicResponseItemLast,
) -> Result<(), Box<dyn std::error::Error>> {
    match response.data.clone() {
        Some(items) => {
            let item_count = items.len();
            tracing::debug!(
                "SNIFFER: Started Saving All Items to DB, received {} items",
                item_count
            );
            let item_hists = items.into_iter().map(|x| ItemHist::from(x)).collect();
            let inserted = item_hist_batch_insert(conn, item_hists).await.unwrap();
            tracing::debug!(
                "SNIFFER: Finished Saving All Items to DB, total inserted: {}",
                inserted
            );
            Ok(())
        }
        None => Err("No items found in response data")?,
    }
}

pub async fn item_hist_batch_insert(
    conn: &mut PgConnection,
    items: Vec<ItemHist>,
) -> Result<usize, Error> {
    tracing::debug!("SNIFFER: Pre-filtering t_type!=os OR (t_type==os AND item_order is null). Original Item Count: {}", items.len());
    let filtered_items: Vec<ItemHist> = items
        .into_iter()
        .filter(|x| x.t_type != "os" || (x.t_type == "os" && !x.item_order.is_none()))
        .collect();

    let item_hist_field_count = 19;
    // Subtract 1 for safety, though this should always be fine
    let max_items_per_batch = (65535 / item_hist_field_count) - 1;
    let filterd_item_count = filtered_items.len();
    tracing::debug!(
        "SNIFFER: Need to insert {} items. Batching item hist insert into {} chunks of {} items each",
        filterd_item_count,
        filterd_item_count / max_items_per_batch,
        max_items_per_batch
    );
    let mut records_inserted: usize = 0;
    for (i, chunk) in filtered_items.chunks(max_items_per_batch).enumerate() {
        tracing::debug!(
            "SNIFFER: Started chunk {}, inserted {} records so far",
            i,
            records_inserted
        );
        records_inserted += diesel::insert_into(item_hist::table)
            .values(chunk)
            .on_conflict_do_nothing()
            .execute(conn)?
    }
    tracing::debug!(
        "SNIFFER: Inserted {} of {} items after deduplication",
        records_inserted,
        filterd_item_count
    );
    Ok(records_inserted)
}

#[allow(dead_code)]
pub async fn item_hist_select_by_item_id(conn: &mut PgConnection, item_id: &str) -> Vec<ItemHist> {
    item_hist::table
        .filter(item_hist::uid.eq(item_id))
        .select(item_hist::all_columns)
        .load(conn)
        .expect("Error loading posts")
}
