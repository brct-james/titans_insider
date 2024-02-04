# titans.brct.io

## Usage

Expects shopsniffer client in parent directory. Generate with:

- `./rebuild_shopsniffer.sh`

Run script with:

- `./run.sh`

## Config

- Port: `50249`
- Surreal DB Port: `8001`

- Item Data Rate Limit: 40 Requests / 15 Minutes or 2.66 Req/m

## Generate Client

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

## TODO

Need to pass connect.sid cookie as part of request, which is currently valid for 14 days, to get historic data older than 2 days (presumably only if upograded)
