# Service catalog

Common infra services, their canonical devbox/nixpkgs package names, default ports, whether devbox ships a built-in plugin, and the env var to set in `devbox.json` to override the port.

Always confirm with `devbox info <pkg>` after adding — plugin behavior changes over time.

## Databases

| Service     | Package          | Default port | Plugin? | Port override env  | Notes                                                  |
|-------------|------------------|--------------|---------|--------------------|--------------------------------------------------------|
| PostgreSQL  | `postgresql`     | 5432         | Yes     | `PGPORT`           | Plugin sets `PGDATA`, `PGHOST`, `PGUSER`. `initdb` on first run. |
| MySQL       | `mysql80`        | 3306         | Yes     | `MYSQL_TCP_PORT`   | Use `mysql80` or `mysql84` for explicit versions.      |
| MariaDB     | `mariadb`        | 3306         | Yes     | `MYSQL_TCP_PORT`   | Plugin similar to MySQL.                               |
| MongoDB     | `mongodb`        | 27017        | Yes     | `MONGO_PORT`       | Some plugin versions key off `MONGODB_PORT`; verify with `devbox info mongodb`. |
| SQLite      | `sqlite`         | n/a          | n/a     | n/a                | App dep, not infra. No service.                        |

## Caches & KV

| Service     | Package          | Default port | Plugin? | Port override env  | Notes                                                  |
|-------------|------------------|--------------|---------|--------------------|--------------------------------------------------------|
| Redis       | `redis`          | 6379         | Yes     | `REDIS_PORT`       | Plugin sets `REDIS_HOST`, `REDIS_PORT`.                |
| Valkey      | `valkey`         | 6379         | Partial | `VALKEY_PORT`      | Redis-protocol fork; treat like redis.                 |
| Memcached   | `memcached`      | 11211        | No      | n/a                | Needs a `process-compose.yaml` entry.                  |

## Queues & messaging

| Service     | Package           | Default port    | Plugin? | Notes                                                            |
|-------------|-------------------|-----------------|---------|------------------------------------------------------------------|
| RabbitMQ    | `rabbitmq-server` | 5672, 15672     | No      | Needs `process-compose.yaml`. Mgmt UI on 15672.                  |
| NATS        | `nats-server`     | 4222            | No      | `process-compose.yaml`. Optional monitoring port 8222.           |
| Kafka       | `apacheKafka`     | 9092            | No      | Complex — prefer KRaft mode. Often easier in Docker for now.     |

## Search

| Service       | Package          | Default port    | Plugin? | Notes                                                                  |
|---------------|------------------|-----------------|---------|------------------------------------------------------------------------|
| Elasticsearch | `elasticsearch`  | 9200, 9300      | No      | Heavy. Set `ES_JAVA_OPTS=-Xms512m -Xmx512m` in `env` for dev.          |
| OpenSearch    | `opensearch`     | 9200, 9300      | No      | Same shape as Elasticsearch.                                            |
| Meilisearch   | `meilisearch`    | 7700            | No      | Lightweight alternative; easy `process-compose.yaml` entry.            |

## Object storage

| Service | Package | Default port | Plugin? | Notes                                                |
|---------|---------|--------------|---------|------------------------------------------------------|
| MinIO   | `minio` | 9000, 9001   | No      | S3-compatible. Web console on 9001.                  |

## Common app deps

App deps don't need service wiring — just `devbox add <pkg>`. Listed here so canonical names are obvious.

| Tool        | Package        | Notes                                |
|-------------|----------------|--------------------------------------|
| just        | `just`         | Task runner.                         |
| ripgrep     | `ripgrep`      | `rg` binary.                         |
| jq          | `jq`           |                                      |
| yq          | `yq-go`        | Go yq; `yq` (Python) also exists.    |
| fd          | `fd`           |                                      |
| Terraform   | `terraform`    | License-restricted in some channels. |
| OpenTofu    | `opentofu`     | Terraform fork.                      |
| kubectl     | `kubectl`      |                                      |
| Helm        | `kubernetes-helm` | Note the namespaced name.         |
| AWS CLI     | `awscli2`      |                                      |
| gcloud      | `google-cloud-sdk` |                                  |
| Docker CLI  | `docker-client` | Client only; daemon stays on host.  |

## Port-collision policy

For any service whose default port likely collides with a host install (Postgres, MySQL, Mongo, Redis), default to bumping by +1 (`5432→5433`, `3306→3307`, `6379→6380`, `27017→27018`). Set the override under `env` in `devbox.json`:

```json
{
  "env": {
    "PGPORT": "5433"
  }
}
```

Mention the override in the report so the user updates connection strings, `.env` files, and any CI config.
