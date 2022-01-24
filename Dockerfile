# select starting image
FROM rust as build


# set working directory
WORKDIR /repo


# copy files across directories
COPY Cargo.lock .
COPY Cargo.toml .
COPY src src


# build release
RUN cargo install --path .


# allow requests to port 80
EXPOSE 80


# run command 
CMD ["aws-rust-api"]
