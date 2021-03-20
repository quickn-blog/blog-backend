FROM rust

RUN mkdir /app
WORKDIR /app
COPY . /app
RUN cargo install --path .
RUN cargo install diesel_cli --no-default-features --features postgres
ENTRYPOINT ["blog-backend"]
RUN diesel_cli migration run