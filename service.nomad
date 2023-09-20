job "stripe-webhooks" {
  datacenters = ["dc1"]
  type        = "service"

  group "stripe-webhooks-api" {
    count = 2

    network {
      mode = "bridge"

      port "http" {}
    }

    service {
      name = "stripe-webhooks-api"
      port = "http"

      connect {
        sidecar_service {
          proxy {
            upstreams {
              destination_name = "zitadel"
              local_bind_port  = 8080
            }
            upstreams {
              destination_name = "cockroach-sql"
              local_bind_port  = 5432
            }
            upstreams {
              destination_name = "media-api"
              local_bind_port  = 10000
            }
          }
        }
      }

      check {
        type     = "http"
        interval = "20s"
        timeout  = "2s"
        path     = "/health"
        method   = "GET"
      }
    }

    task "stripe-webhooks-api" {
      driver = "docker"

      vault {
        policies = ["service-stripe-webhooks"]
      }

      template {
        destination = "${NOMAD_SECRETS_DIR}/.env"
        env         = true
        change_mode = "restart"
        data        = <<EOF
{{ with nomadVar "nomad/jobs/stripe-webhooks" }}
RUST_LOG='{{ .LOG_LEVEL }}'
{{ end }}

HOST='0.0.0.0:{{ env "NOMAD_PORT_http" }}'

DB_HOST='{{ env "NOMAD_UPSTREAM_IP_cockroach-sql" }}'
DB_PORT='{{ env "NOMAD_UPSTREAM_PORT_cockroach-sql" }}'
DB_DBNAME='stripe_webhooks'
DB_USER='stripe_webhooks_user'
{{ with secret "database/static-creds/stripe_webhooks_user" }}
DB_PASSWORD='{{ .Data.password }}'
{{ end }}

{{ with secret "kv2/data/services/stripe-webhooks" }}
SERVICE_USER_CLIENT_ID='{{ .Data.data.SERVICE_USER_CLIENT_ID }}'
SERVICE_USER_CLIENT_SECRET='{{ .Data.data.SERVICE_USER_CLIENT_SECRET }}'
STRIPE_ENDPOINT_SECRET='{{ .Data.data.STRIPE_ENDPOINT_SECRET }}'
{{ end }}

OAUTH_URL='http://{{ env "NOMAD_UPSTREAM_ADDR_zitadel" }}/oauth'
{{ with nomadVar "nomad/jobs/" }}
OAUTH_HOST='{{ .JWKS_HOST }}'
{{ end }}

CORS_ALLOWED_ORIGINS=""
MEDIA_SERVICE_URL='http://{{ env "NOMAD_UPSTREAM_ADDR_media-api" }}'
EOF
      }

      config {
        image = "__IMAGE__"
      }
    }
  }
}
