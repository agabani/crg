# crg

```sh
RUST_LOG=debug CRG__http_server__host=0.0.0.0 cargo run | bunyan
```

```sh
sudo mkdir -p /run/containers/1000 && sudo chown 1000:1000 /run/containers/1000
```

```sh
skopeo login --tls-verify=false 127.0.0.1:8080 -u <username> -p <password> --debug
```

```
skopeo copy --src-tls-verify=false docker://127.0.0.1:8080/library/alpine:latest dir:./crg --debug
skopeo copy --src-tls-verify=false docker://registry-1.docker.io/library/alpine:latest dir:./docker-hub --debug
```

```
skopeo copy --dest-tls-verify=false docker://registry-1.docker.io/library/alpine:latest docker://127.0.0.1:8080/<username>/test:latest --debug
```

```
skopeo copy --override-os windows --dest-tls-verify=false docker://registry-1.docker.io/library/python:latest docker://127.0.0.1:8080/<username>/test:python --debug
```
