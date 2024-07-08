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
ENV SCRIPTSFOLDER=rest-examples
ENV PORT=2080

EXPOSE 2080

CMD ["./curldock"]