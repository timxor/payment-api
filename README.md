# payment-api
payment api for sending crypto to friends using their phone number or email

# mac development

## install depencies
```
cargo build
```
start Postgres app, version 10.
use pgadmin4.

## configure .env local db
payment-api/payment-api/.env should have something like this:
```
DATABASE_URL=postgres://infomarket:password@localhost/infomarket
```

##test it is running:
```
psql postgres://infomarket:password@localhost:5432
```

# cli // soon to be api
## create a transfer
```
cargo run --bin create_transfer
```


## show transfers
```
cargo run --bin show_transfers
```


# repo url
```
https://github.com/tcsiwula/payment-api 
```
