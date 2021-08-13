### Development

Environment

```
docker-compose up -d
docker-compose exec rust sh
```

Compile and run

```
cargo run 0.0.0.0:8899 0.0.0.0:8899
```

### Links

- https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo
- https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.set_nonblocking
- https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html#display-formatted-date-and-time
- https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
