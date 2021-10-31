# cockroachdb-rust

CockroachDBとPostgreSQLでベンチマーク

# 環境

CockroachDB

PostgreSQL

Rust - Diesel

## 構築

```
$ docker-compose up -d --build
```

```
$ docker-compose run --rm rust bash 

bash# diesel setup

bas# diesel migration run

bash# cargo run

bash# build --release
```

## ベンチマーク

Postgres

```
bash# time DATABASE_URL=postgresql://root@postgres:5432/test /tmp/target/release/app
```

Cockroach

```
bash# time /tmp/target/release/app
```
