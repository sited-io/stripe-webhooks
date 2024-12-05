FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release


COPY src src
COPY migrations migrations
RUN touch src/main.rs
RUN cargo build --release

RUN strip target/release/stripe-webhooks

FROM debian:bookworm-slim AS release
WORKDIR /app

COPY --from=builder /app/target/release/stripe-webhooks .

RUN apt update && apt install -y --no-install-recommends ca-certificates adduser
RUN update-ca-certificates

# Create appuser
ENV USER=stripe-webhooks_user
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

# Use an unprivileged user.
USER ${USER}:${USER}

ENTRYPOINT [ "./stripe-webhooks" ]
