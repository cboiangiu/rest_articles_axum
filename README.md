# rest_articles_axum

_`rest_articles_axum`_ _is a blogging platform_

## Tech

`Rust axum server` using `DDD + Vertical Slices` and `CQRS` with `gRPC + REST Swagger API` module, `Identity` module, `HTMX Web` module + `Next.js Web` module extension, `MongoDB + PostgreSQL` databases, `Client KMP` (mobile) app and `Unit + e2e tests` currently under `3000 LOC`.

## Run

0. Ensure `MongoDB` running on port `27017`
1. `cargo run`
2. `npm run dev`
3. Go to [`localhost:3000`](http://localhost:3000)
4. Open `client-kmp` folder using `Android Studio` and start app.

## Test

- `cargo test` for rust unit tests (`api + identity` modules)
- `Run` first, then `cd e2e-playwright && npx playwright test` for e2e tests (`api + web + identity` modules)

## cloc

`cloc --exclude-dir=target,.next,node_modules,docs,assets,public,tests-examples,playwright-report,test-results,res,gradle,build,.idea,.gradle,.fleet,project.xcworkspace,xcuserdata --not-match-f=".*\.(json|md|toml|xml|yaml|properties|kts|plist)$" --not-match-f="fake_mongo_repository.rs" --not-match-f="multiplex_service.rs" --not-match-f="gradlew.bat" --not-match-f="gradlew" --by-file-by-lang .`

| Language         |    files |    blank |  comment |     code |
| :--------------- | -------: | -------: | -------: | -------: |
| Rust             |       36 |      129 |       86 |     1217 |
| Kotlin           |       13 |       52 |        2 |      417 |
| TypeScript       |        7 |       27 |       49 |      199 |
| Swift            |        3 |       21 |        0 |      142 |
| Protocol Buffers |        5 |       20 |        0 |       61 |
| HTML             |        3 |        3 |        0 |       37 |
| CSS              |        3 |        4 |        0 |       36 |
| JavaScript       |        2 |        2 |        2 |       10 |
| --------         | -------- | -------- | -------- | -------- |
| SUM:             |       72 |      258 |      139 |     2119 |

## Databases

### postgres

![postgres schemas img](docs/db/postgres_schemas.png "postgres_schemas")

### mongo

![mongo schemas img](docs/db/mongo_schemas.png "mongo_schemas")

## TODO

- add tags, follow profiles, follow custom tags list
- implement and use postgres repository/orm solution
- implement e2e testing
- implement an event sourcing solution
- cleanup (unnecessary clones, wrong types, strange patterns, rustify code?)
- implement auth and secure endpoints + update swagger with auth
- implement logging
- implement transactions
- implement more complex endpoints and entities with refs?
