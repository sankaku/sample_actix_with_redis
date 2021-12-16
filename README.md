# sample_actix_with_redis
Usage sample for redis crates below with actix-web.

- [redis-rs](https://github.com/mitsuhiko/redis-rs)
  - without pooling  
    `direct.rs`.
  - with r2d2 pooling  
    `with_r2d2.rs`.
- [bb8-redis](https://github.com/djc/bb8)  
  `with_bb8.rs`.
- [deadpool-redis](https://github.com/bikeshedder/deadpool)  
  `with_deadpool.rs`.
- [mobc](https://github.com/importcjj/mobc)  
  `with_mobc.rs`.
- [r2d2-redis](https://github.com/sorccu/r2d2-redis)  
  `with_old_r2d2.rs`.

## Requirements
- cargo
- docker-compose

## Build
```sh
cd app
cargo build
```

## Run
```sh
docker-compose up -d

cd app
cargo run

# to stop...
# Press Ctrl-C
cd ../
docker-compose down
```

## Endpoints
All endpoints accept HTTP GET method.

`uuid` in `localhost:8080/foo/{uuid}` is the string you'll get as the response of `localhost:8080/foo`.

- `/direct` 
- `/direct/{uuid}`
- `/with_r2d2` 
- `/with_r2d2/{uuid}`
- `/bb8` 
- `/bb8/{uuid}`
- `/deadpool` 
- `/deadpool/{uuid}`
- `/mobc` 
- `/mobc/{uuid}`
- `/old_r2d2` 
- `/old_r2d2/{uuid}`


Example
```sh
$ curl localhost:8080/direct
e5aa716d-e647-484a-ad91-8864f20380b1 # no newline here actually
$ curl localhost:8080/direct/e5aa716d-e647-484a-ad91-8864f20380b1
hi # no newline here actually
```

## License
MIT