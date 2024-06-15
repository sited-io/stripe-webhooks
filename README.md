# Payment service for sited.io

## Prerequesites

Ensure `service-apis` git submodule is initialized. If not yet done run:

```sh
git submodule update --init
```

If `service-apis` git submodule was already initialized, ensure to pull the newest changes:

```sh
git submodule update --remote
```

## Build

```sh
cargo build
```

## Run locally

Ensure environment variables are set.

```sh
export RUST_LOG=info
export RUST_BACKTRACE=0

export HOST="[::1]:10000"

export DB_HOST='127.0.0.1'
export DB_PORT='5433'
export DB_USER='payment_user'
export DB_PASSWORD=''
export DB_DBNAME='payment'

export JWKS_URL='https://auth-dev.sited.io/oauth/v2/keys'
export JWKS_HOST='auth-dev.sited.io'

export COMMERCE_SERVICE_URL='https://grpc-dev.sited.io:443'

export STRIPE_SECRET_KEY="xxxx"
```

### local database

```sh
  docker run --rm -d \
    --name payment_db \
    -p $DB_HOST:$DB_PORT:$DB_PORT/tcp \
    -v "payment_db:/cockroach/cockroach-data" \
    --env COCKROACH_DATABASE=$DB_DBNAME \
    --env COCKROACH_USER=$DB_USER \
    cockroachdb/cockroach start-single-node --sql-addr=0.0.0.0:$DB_PORT --insecure
```

Then run:

```sh
cargo run
```
