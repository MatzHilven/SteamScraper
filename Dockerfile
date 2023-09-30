FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/steam-scraper
COPY . .

RUN cargo build --release

FROM debian:bullseye
COPY --from=builder /usr/src/steam-scraper/target/release/steam-scraper /usr/local/bin/steam-scraper

RUN apt update && apt install -y pkg-config libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin
CMD ["steam-scraper"]
