# recipes
Amazing recipes management platform.


## How to run
Change working directory to Rust service.
```bash
cd recipes-rs
```

Start database container.
```bash
docker compose up -d
```

If starting for the first time, initialize database
```bash
diesel setup
```

Prepare `.env` file from `.env.template`.
```bash
mv .env.template .env
```

Start recipes-rs service.
```bash
cargo run
```
