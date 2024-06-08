# rest_articles_axum

_`rest_articles_axum`_ _is a blogging platform_

## Tech

`Rust axum server` using `DDD + Vertical Slices` and `CQRS` with `gRPC + REST Swagger API` module, `Identity` module, `HTMX Web` module + `Next.js Web` module extension, `MongoDB + PostgreSQL` databases and `Unit + e2e tests` currently under `2000 LOC`.

## Run

0. Ensure `MongoDB` running on port `27017`
1. `cargo run`
2. `npm run dev`
3. Go to [`localhost:3000`](http://localhost:3000)

## Test

- `cargo test` for rust unit tests (`api + identity` modules)
- `Run` first, then `cd e2e-playwright && npx playwright test` for e2e tests (`api + web + identity` modules)

## cloc

`cloc --exclude-dir=target,.next,node_modules,docs,assets,public,tests-examples,playwright-report,test-results --not-match-f=".*\.(json|md|toml)$" --not-match-f="fake_mongo_repository.rs"  --by-file-by-lang .`

## Databases

### postgres_api_schema

![postgres api schema img](docs/db/postgres_api_schema.png "postgres_api_schema")

## TODO

- implement and use postgres repository/orm solution
- implement e2e testing
- implement an event sourcing solution
- cleanup (unnecessary clones, wrong types, strange patterns, rustify code?)
- implement auth and secure endpoints + update swagger with auth
- implement logging
- implement transactions
- implement more complex endpoints and entities with refs?
