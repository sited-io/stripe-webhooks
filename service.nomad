job "stripe-webhooks" {
  datacenters = ["dc1"]
  type        = "service"

  group "stripe-webhooks-api" {
    count = 1

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
              destination_name = "media-api"
              local_bind_port  = 10000
            }
          }
        }
      }
    }

    task "stripe-webhooks-api" {
      driver = "docker"

      resources {
        cpu        = 200
        memory     = 256
        memory_max = 256
      }

      vault {
        policies = ["service-stripe-webhooks"]
      }

      template {
        destination = "${NOMAD_SECRETS_DIR}/.env"
        env         = true
        change_mode = "restart"
        data        = <<EOF
{{ with nomadVar "nomad/jobs/stripe-webhooks" }}
RUST_LOG='{{ .RUST_LOG }}'
{{ end }}

HOST='0.0.0.0:{{ env "NOMAD_PORT_http" }}'

{{ with nomadVar "nomad/jobs/stripe-webhooks"}}
DB_HOST='{{ .DB_HOST }}'
DB_PORT='{{ .DB_PORT }}'
DB_DBNAME='{{ .DB_DBNAME }}'
DB_USER='{{ .DB_USER }}'
{{ end }}
DB_ROOT_CERT='{{ env "NOMAD_SECRETS_DIR" }}/database_root_cert.crt'
{{ with secret "kv2/data/services/stripe-webhooks" }}
DB_PASSWORD='{{ .Data.data.DB_PASSWORD }}'
{{ end }}

{{ with secret "kv2/data/services/stripe-webhooks" }}
STRIPE_ENDPOINT_SECRET='{{ .Data.data.STRIPE_ENDPOINT_SECRET }}'
{{ end }}

CORS_ALLOWED_ORIGINS=""
MEDIA_SERVICE_URL='http://{{ env "NOMAD_UPSTREAM_ADDR_media-api" }}'

{{ with nomadVar "nomad/jobs" }}
NATS_HOST='{{ .NATS_HOST }}'
NATS_USER='{{ .NATS_USER }}'
{{ end }}
{{ with secret "kv2/data/services" }}
NATS_PASSWORD='{{ .Data.data.NATS_PASSWORD }}'
{{ end }}
EOF
      }

      config {
        image = "__IMAGE__"
      }
    }
  }
}
