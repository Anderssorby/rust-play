# Rust play

Build docker image with nix
```
nix-build docker.nix
```

Load

```
docker load -i result
```

Run backend

```
docker run -p 5000:5000 -it anderscs/rust-play-backend
```
