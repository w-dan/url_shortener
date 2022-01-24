# select a starting image to build off
FROM rust:1.58.1 as build


# create new empty shell project
RUN USER=root cargo new --bin url-shortener


# set our working directory in the container
WORKDIR /url-shortener


# copy all our files across from our local repo to the /repo directory in the container
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml


# build the release, cache dependencies
RUN echo "// empty fule" > src/lib.rs
RUN cargo build --release


# copy source tree
COPY ./src ./src


# build for release
RUN cargo install --offline --path .
RUN cargo build --release


# slim image for running container (reducing bloat)
FROM rust:slim
WORKDIR /app


# allow requests to port 8000
EXPOSE 80


# copy the build artifact from the build stage
COPY --from=build /usr/local/cargo/bin/aws-rust-api /usr/local/bin/aws-rust-api


# copy config file
COPY Rocket.toml .


# set the startup command to run binary
CMD ["aws-rust-api"]
