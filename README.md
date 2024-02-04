# titans.brct.io

## Roadmap

1. ~~Connect to smarty titans api and save to database~~
2. ~~Convert from surreal to postgres using diesel~~
3. Create process for archiving data to conserve storage space (currently estimating +1gb every ~40 hours) (probably store daily average/high/low price, and similar for qty), consider only storing a subset of the smarty titans cols
4. Use item and translation info to make data more user friendly
5. Start using the historical endpoint to build back history into the past (may need to pass connect.sid cookie as part of request, which is currently valid for 14 days, to get historic data older than 2 days) - ideally 60 months of data would be great
6. Develop custom metrics (best time to buy and sell flawless T13, month-long arbitrage opportunities, etc.)

### Uncommitted

- Add event timing to DB and custom metrics (best items to sell and when best to sell specifically near dragon event for example)
- Create web UI @ titans.brct.io
- Develop triggers/alerts (discord, web, etc.)

## Dev Setup

Update your local image of openapitools/openapi-generator-cli:latest-release

```bash
docker pull openapitools/openapi-generator-cli:latest-release
```

Run the following command, which uses the openapi-generator-cli docker image to generate the client:

```bash
docker run --rm \
  -v ${PWD}:/local openapitools/openapi-generator-cli:latest-release generate \
  -i /local/titans_insider/src/spec/openapi.json \
  -g rust \
  -o /local/shopsniffer \
  --additional-properties=packageName=shopsniffer,supportAsync=true,supportMiddleware=true
```

Install diesel cli: `cargo install diesel_cli --no-default-features --features postgres`

Create .env and postgres_secrets.env based on example files.

Setup diesel: `diesel setup`

Run migration: `diesel migration run` OR Redo migration: `diesel migration redo --all`

## Usage

Expects shopsniffer client in parent directory. Generate with:

- `./rebuild_shopsniffer.sh`

Run script with:

- `./run.sh`

Create new tables with:

- `diesel migration generate create_<tablename>`

Wipe DB and rebuild schema with:

- `./wipe_db.sh`

Wipe, rebuild, then run with:

- `./fresh_start.sh`

If diesel is really borked, try:

- `diesel database reset`

If you need to take the database down for some reason:

- `down.sh`

To check size of database:

- `SELECT pg_size_pretty(pg_database_size('shopsniffer'))`

## Config Notes

- Port: `50249`
- Surreal DB Port: `8001`

- Item Data Rate Limit: 40 Requests / 15 Minutes or 2.66 Req/m
- Currently polling live every 24 seconds, using 37.5 requests/15m
