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

## test it is running:
```
psql postgres://infomarket:password@localhost:5432
```

# start the api server
``` 
export DATABASE_URL=postgres://macdev:timistheshit@localhost/paymentapi
cargo build
cargo run --bin payment_api
```

# cli 
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


# updating the db
First drop the table:
```
psql postgres://macdev:timistheshit@localhost/paymentapi
select * from users;
drop table users;
select * from users;
```

Then create a new up.sql:
```
diesel migration generate newSqlThingy
```

Then edit the schema in up.sql and run it:
```
diesel migration run
```

The new table should be here now:
```
select * from users;
```
