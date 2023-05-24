FROM rust:1.67

WORKDIR /usr/src/sir
COPY . .

RUN cargo install --path .

CMD ["sir"]
