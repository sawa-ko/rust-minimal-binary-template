# Set image base to alpine linux latest version and name the stage as builder
FROM alpine:latest as builder

# Set environment variables
ENV PATH="/root/.cargo/bin:${PATH}:/app/target/release/migration"
ENV RUSTFLAGS="-Ctarget-feature=-crt-static"

# Install build dependencies
RUN apk update && \
    apk add --no-cache \
    curl \
    gcc \
    g++ \
    libc-dev \
    openssl-dev

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Check rust version
RUN rustc --version && cargo --version

# Set working directory
WORKDIR /app

# Copy source code
COPY . .

# Build application
RUN cargo build --release --package naeko-bot

# Set image base to alpine linux latest version and name the stage as runtime
FROM alpine:latest as runtime

# Set environment variables
ENV PATH="/app/target/release:${PATH}/app/naeko-bot"

# Install runtime dependencies
RUN apk update && \
    apk add --no-cache \
    openssl \
    libgcc \
    libstdc++

# Set working directory
WORKDIR /app

# Copy binary
COPY --from=builder /app/target/release/naeko-bot .

# Expose port
EXPOSE 3000

# Run application
ENTRYPOINT ["./naeko-bot"]

# Labels for the image
LABEL maintainer="kanamepng"
LABEL version="1.0"
LABEL description="Compile and run the application in a single container using a multi-stage build pattern. This pattern is useful when you want to compile your application in one container and run it in another container."
LABEL license="AGPL-3.0"