# Pull from dockerhub
```
docker pull bjacklyn/rust-chat-app:1.0 # https://hub.docker.com/repository/docker/bjacklyn/rust-chat-app
```

# Alternatively build from source (optional)
```
docker build . --tag rust_chat_app:1.0 # NOTE: takes 20 mins to build
```

# Run
```
docker run -it --network=host rust_chat_app:1.0 /opt/rust_chat_app/target/release/axum_backend
```

# Try it out
Go to http://localhost:3000/ in a couple browser tabs

# Example Chat

## Person 1:
![image](https://github.com/user-attachments/assets/ef193b8e-3b72-4a0e-b383-d56f3558a617)

## Person 2:
![image](https://github.com/user-attachments/assets/233b5ee7-ac09-4718-8a7a-39983fb0e2c2)

## Person 3 (observer):
![image](https://github.com/user-attachments/assets/2c7b89a6-c009-4ef6-b274-b41ba708f3ff)
