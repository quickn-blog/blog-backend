FROM rust

RUN mkdir /app
WORKDIR /app
COPY . /app
RUN cargo install .
CMD ["blog-backend"]