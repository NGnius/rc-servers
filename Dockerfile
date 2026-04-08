FROM rust:latest
WORKDIR /app
RUN git config --global --add safe.directory '*'

CMD ["sleep", "infinity"]