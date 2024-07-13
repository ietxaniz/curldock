# Build stage for the Rust backend
FROM rust:1.79-alpine AS backend-builder
WORKDIR /usr/src/app
RUN apk add --no-cache musl-dev
COPY . .
RUN cargo build --release

# Build stage for the frontend
FROM node:20.15-alpine3.20 AS frontend-builder
WORKDIR /usr/src/app
COPY front/ .
RUN npm ci
RUN npm run build
RUN apk add --no-cache sed findutils && \
    BUNDLE_FILE=$(find dist/assets -name 'index-*.js') && \
    if [ -n "$BUNDLE_FILE" ]; then \
        sed -i 's|https://cdn.jsdelivr.net/npm/monaco-editor@0.43.0/min/vs|/monaco-editor/min/vs|g' "$BUNDLE_FILE" && \
        echo "Modified bundle file: $BUNDLE_FILE" ; \
    else \
        echo "Bundle file not found" && exit 1 ; \
    fi && \
    mkdir -p dist/monaco-editor/min && \
    cp -r node_modules/monaco-editor/min/vs dist/monaco-editor/min/ ;

# Final stage
FROM alpine:3.20.1
RUN apk add --no-cache curl
WORKDIR /usr/src/app

# Copy the built backend binary
COPY --from=backend-builder /usr/src/app/target/release/curldock .

# Copy the built frontend assets
COPY --from=frontend-builder /usr/src/app/dist ./front/dist

# Set environment variables
ENV MODE=PROD
ENV SCRIPTSFOLDER=/scripts
ENV PORT=2080

EXPOSE 2080

CMD ["./curldock"]