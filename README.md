# move-rs

## Prerequesites

* docker
* docker-compose

## Getting started

```sh
cp .env.sample .env
docker pull marionebl/move_rs_dev
docker-compose build
docker-compose run move_rs run # binds to port 8000
```