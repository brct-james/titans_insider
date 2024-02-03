cd ..
docker pull openapitools/openapi-generator-cli:latest-release
docker run --rm \
  -v ${PWD}:/local openapitools/openapi-generator-cli:latest-release generate \
  -i /local/shopsniffer/src/spec/openapi.json \
  -g rust \
  -o /local/shopsniffer \
  --additional-properties=packageName=shopsniffer,supportAsync=true,supportMiddleware=true