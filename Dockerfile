FROM lukemathwalker/cargo-chef:latest-rust-1.70.0 AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
# Compute a lock file for the project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build our project dependencies not our application
RUN cargo chef cook --release --recipe-path recipe.json

    
# Up to this point, if our dependency tree stays the same, 
# all layers should be cached
COPY . .
ENV SQLX_OFFLINE true 
# Build binary
# Use release 
RUN cargo build --release --bin zero2prod


FROM debian:bullseye-slim AS runtime
# Let's switch our working directory to 'app' (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not exist already 
WORKDIR /app

# Install required system dependencies for our linking configuration
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \ 
    # Clean up 
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
# When `docker run` is executed, launch the binary!
ENTRYPOINT [ "./zero2prod" ]