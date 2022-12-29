FROM rust
WORKDIR /home/app
COPY . .
RUN apt-get update
RUN apt-get install libasound2-dev -y
RUN cargo build