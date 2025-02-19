FROM rust:1.84 AS builder

WORKDIR /app

RUN cargo init --bin

# Then, copy over dependencies
COPY Cargo.toml Cargo.lock ./
# And compile for these dependencies
RUN cargo build --release --locked

# Now, copy over the source code
COPY src ./src

# And compile this
RUN cargo build --release


# Move compiled binary to smaller image, and run it
FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/msc-backend ./msc-backend

EXPOSE 3000

CMD ["./msc-backend"]