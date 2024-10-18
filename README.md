# diesel demo

```bash
sudo apt-get update && sudo apt-get install -y libpq-dev

cp sample.env .env
export $(grep -v '^#' .env | xargs)

docker compose up -d

cargo build

cargo run --bin=migration
# cargo run --bin=write_post
# cargo run --bin=publish_post 1
# cargo run --bin=show_post
# cargo run --bin=get_post 1
# cargo run --bin=delete_post title_post
```
