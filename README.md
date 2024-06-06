# rest_articles_axum

`Rust axum server` using `DDD + Vertical Slices` with `gRPC + REST Swagger API` module, `Identity` module, `HTMX Web` module + `Next.js Web` module extension, `MongoDB + PostgreSQL` databases and `Unit tests`.

## Run

0. Ensure `MongoDB` running on port `27017`
1. `cargo run`
2. `npm run dev`

## cloc

`cloc --exclude-dir=target,.next,node_modules --not-match-f=package-lock.json .`

## TODO

- implement and use postgres repository/orm solution
- implement an event sourcing solution
- cleanup (unnecessary clones, wrong types, strange patterns, rustify code?)
- implement auth and secure endpoints + update swagger with auth
- implement logging
- implement transactions
- implement more complex endpoints and entities with refs?
