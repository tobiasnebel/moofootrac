## Build
```bash
$ cargo build
```

## Run
Run with default profile
```bash
$ cargo run
```
Run with dev profile
```bash
$ cargo run -- --config=config/dev.yml
```


## Docker
In `./docker/dev/`:
```bash
$ docker compose up -d postgres
```


## Docker Image
In `./` (project root):
```
$ docker build -t moofoolog:linux -f ./docker/moofoolog/Dockerfile .
```

## Running with Compose
In `./docker/prod/`:
```
$ docker compose up -d
```