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
              destination_name = "nats"
              local_bind_port = 4222
            }
            upstreams {
              destination_name = "postgres-sql"
              local_bind_port  = 5432
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
        destination = "${NOMAD_SECRETS_DIR}/database_root_cert.crt"
        env         = false 
        change_mode = "restart"
        data        = <<EOF
{{- with secret "kv2/data/services" -}}
{{ .Data.data.DATABASE_ROOT_CERT }}
{{- end -}}
EOF
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

NATS_HOST='{{ env "NOMAD_UPSTREAM_ADDR_nats" }}'
NATS_USER='{{- with nomadVar "nomad/jobs" -}}{{ .NATS_USER }}{{- end -}}'
NATS_PASSWORD='{{- with secret "kv2/data/services" -}}{{ .Data.data.NATS_PASSWORD }}{{- end -}}'

DB_HOST='{{ env "NOMAD_UPSTREAM_IP_postgres-sql" }}'
DB_PORT='{{ env "NOMAD_UPSTREAM_PORT_postgres-sql" }}'
DB_DBNAME='stripe_webhooks'
DB_USER='stripe_webhooks_user'
DB_PASSWORD='{{- with secret "database/static-creds/stripe_webhooks_user" -}}{{ .Data.password }}{{- end -}}'

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
EOF
      }

      config {
        image = "__IMAGE__"
      }
    }
  }
}
