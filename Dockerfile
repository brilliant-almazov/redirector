# Build stage
FROM rust:1.88-alpine AS builder

RUN apk add --no-cache musl-dev pkgconfig openssl-dev

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn lib() {}" > src/lib.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release --bin redirector && \
    rm -rf src

# Copy source code
COPY src ./src
COPY templates ./templates
COPY static ./static

# Build the application
RUN touch src/main.rs src/lib.rs && \
    cargo build --release --bin redirector

# Compress with UPX
RUN apk add --no-cache upx && \
    upx --best --lzma /app/target/release/redirector

# Runtime stage
FROM alpine:3.20

RUN apk add --no-cache ca-certificates

WORKDIR /app

# Copy binary and static assets from builder
COPY --from=builder /app/target/release/redirector /app/redirector
COPY --from=builder /app/static /app/static

# Copy config example (actual config should be mounted)
COPY config.yaml.example /app/config.yaml.example

# Create non-root user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/healthz || exit 1

# Run
ENV CONFIG_PATH=/app/config.yaml
CMD ["./redirector"]
