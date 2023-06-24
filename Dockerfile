# We use the latest Rust stable release as base image
FROM rust:1.68.0
# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app
# Copy all files from our working environment to our Docker image
COPY . .
# Force sqlx to load from the offline cache
ENV SQLX_OFFLINE true
# Set the environment to production
ENV APP_ENVIRONMENT production
# Let's build our binary!
# We'll use the release profile to make it faaaast
RUN cargo build --release
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./target/release/neu-backend"]
# Build a docker image tagged as "neu_backend" according to the recipe
# specified in `Dockerfile`
# docker build --tag neu_backend --file Dockerfile .