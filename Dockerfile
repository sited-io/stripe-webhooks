FROM debian:bookworm-slim

RUN apt update && apt install -y --no-install-recommends ca-certificates adduser
RUN update-ca-certificates

# Copy our build
COPY target/release/stripe-webhooks /usr/local/bin/stripe-webhooks

# Create appuser
ENV USER=stripe_webhooks_user
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

ENTRYPOINT ["stripe-webhooks"]
