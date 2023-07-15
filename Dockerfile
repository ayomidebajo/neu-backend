FROM lukemathwalker/cargo-chef:latest-rust-1.65.0 as chef
WORKDIR /app
FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Copy all files from our working environment to our Docker image
COPY . .
# Force sqlx to load from the offline cache
ENV SQLX_OFFLINE true
ENV APP_ENVIRONMENT production
# Let's build our binary!
RUN cargo build --release --bin neu-backend


# Runtime stage
FROM debian:bullseye-slim AS runtime
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
RUN apt-get update -y \
	&& apt-get install -y --no-install-recommends openssl \
	# Clean up
	&& apt-get autoremove -y \
	&& apt-get clean -y \
	&& rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/neu-backend neu-backend
COPY configuration configuration
ENV APP_ENVIRONMENT production
# ENV SQLX_OFFLINE true
ENTRYPOINT ["./neu-backend"]
# Build a docker image tagged as "neu_backend" according to the recipe
# specified in `Dockerfile`
# docker build --tag neu_backend --file Dockerfile .