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
- [mobc-redis](https://github.com/importcjj/mobc)  
  `with_mobc.rs`.
- [r2d2-redis](https://github.com/sorccu/r2d2-redis)  
  `with_alt_r2d2.rs`.

## Requirements
- cargo
- docker-compose

## Run
```sh
docker-compose up -d

cd app
cargo run

# to stop...

# Press Ctrl-C
# and
cd ../
docker-compose down
```

## Endpoints
All endpoints accept HTTP GET method.

`uuid` in `/foo/{uuid}` is the string you'll get as the response of `/foo`.

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
- `/alt_r2d2` 
- `/alt_r2d2/{uuid}`


Example
```sh
$ curl localhost:8080/direct
e5aa716d-e647-484a-ad91-8864f20380b1 # no newline here actually
$ curl localhost:8080/direct/e5aa716d-e647-484a-ad91-8864f20380b1
hi # no newline here actually
```

## License
MIT