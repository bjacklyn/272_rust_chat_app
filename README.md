# Build
```
docker build . --tag rust_chat_app:1.0
```

# Run
```
docker run -it --network=host rust_chat_app:1.0 /opt/rust_chat_app/target/release/axum_backend
```

# Try it out
Go to http://localhost:3000/ in a couple browser tabs
