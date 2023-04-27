FROM rust

USER executing_user

WORKDIR /usr/src/myapp
COPY . .

RUN cargo clean
RUN cargo build --release

EXPOSE 3000

CMD [ "./target/release/trainingsapp-alpha" ]