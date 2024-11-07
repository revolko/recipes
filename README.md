# recipes
Amazing recipes management platform.


## How to run Rust service
Change working directory to Rust service.
```bash
cd recipes-rs
```
Prepare `.env` file from `.env.template`.
```bash
mv .env.template .env
```

Build the Rust recipe service.
```bash
docker build -t recipes-rs .
```

Start containers.
```bash
docker compose up -d
```
