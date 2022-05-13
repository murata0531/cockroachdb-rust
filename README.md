# cockroachdb-rust

CockroachDBとPostgreSQLでベンチマーク

# 環境

CockroachDB : 19.2.4

PostgreSQL : 11.1

Rust : 1.60

Diesel : 1.4.1

Docker

## 構築

```
$ docker-compose up -d --build
```

```
$ docker-compose run --rm rust bash 

bash# diesel setup

bas# diesel migration run

bash# cargo run

bash# cargo build --release
```

## PostgreSQLにログイン
```
$ docker-compose exec postgres bash

bash# psql -h localhost -p 5432 -U root -d test
```

コンテナとイメージ破棄

```
$ docker-compose down --rmi all --volumes --remove-orphans
```
## ベンチマーク

Postgres

```
bash# time DATABASE_URL=postgresql://root@postgres:5432/test /tmp/target/release/app
```

Cockroach

```
bash# time DATABASE_URL=postgresql://root@roach1:26257/test?sslmode=disable /tmp/target/release/app
```
もしくは

```
bash# time /tmp/target/release/app
```

`http://localhost:8080`にアクセスするとCockroachDBの状態を確認できる