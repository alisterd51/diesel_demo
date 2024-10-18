FROM rust:1.82-bookworm AS build

RUN apt-get update && apt-get install -y libpq-dev

COPY . .

RUN  cargo build --release --bin=migration

FROM debian:bookworm-slim AS runtime

COPY --from=build /target/release/migration /migration

CMD [ "/migration" ]
