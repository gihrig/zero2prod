# Zero To Production / Code (Chapter 5)

<div align="center"><a href="https://zero2prod.com" target="_blank"><img src="https://static-2.gumroad.com/res/gumroad/3629854790655/asset_previews/bc9026cad3ece1746327c1d70218f602/retina/rsz_zero_to_production_punk.png" /></a></div>

[Zero To Production In Rust](https://zero2prod.com) is an opinionated introduction to backend development using Rust.

This repository serves as supplementary material for [the book](https://zero2prod.com/): it hosts snapshots of the codebase of our email newsletter project at end of each chapter.

**This branch is a snapshot of the project at the end of Chapter 5.**

## Pre-requisite

You'll need to install:

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)
- [DigitalOcean Doctl](https://docs.digitalocean.com/reference/doctl/how-to/install/)

Launch a (migrated) Postgres database via Docker:

```bash
./scripts/init_db.sh
```

## How to build

```bash
cargo build
```

## How to test

```bash
cargo test
```

## How to run

```bash
cargo run
```
In separate terminal

```bash
curl -v http://127.0.0.1:8000/health_check
or
curl -v POST --data 'name=le%20guin&email=ursula_le_guin%40gmail.com' 127.0.0.1:8000/subscriptions
```

## Build Docker container

```bash
docker build --tag zero2prod --file Dockerfile .
```

## Run in Docker container

```bash
docker run -p 8000:8000 zero2prod
```

## Show Docker image build size

```bash
docker images zero2prod
```

## Create app at DigitalOcean

Connect doctl to DigitalOcean

```bash
doctl auth init
```

Link GitHub account to [DigitalOcean](https://www.digitalocean.com/docs/app-platform/how-to/troubleshoot-app/#review-github-permissions)


```bash
doctl apps create --spec spec.yaml
```

List apps on DigitalOcean

```bash
doctl apps list
```

Alternatively: [DigitalOcean Dashboard](https://cloud.digitalocean.com/apps/?i=cac6e3)

Update DO app

```bash
doctl apps update YOUR-APP-ID --spec=spec.yaml
```

## DigitalOcean API

Working with `doctl`

```bash
doctl --help
doctl apps --help
doctl apps update --help
```

[DigitalOcean API V2 docs](https://developers.digitalocean.com/documentation/v2/)

Get instance_sizes
```bash
curl -X GET -H "Content-Type: application/json" -H "Authorization: Bearer e7a53616474e591c80758a2d014928d40af916d182aff51683419fc8bb351fa5" "https://api.digitalocean.com/v2/apps/tiers/instance_sizes"
```
