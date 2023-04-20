FROM rust:1.67-alpine as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/sprint_task_csv_generator /usr/local/bin/sprint_task_csv_generator
RUN ["ls","-la","/usr/local/bin"]

ENTRYPOINT ["/bin/bash"]
