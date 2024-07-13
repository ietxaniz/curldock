#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Set variables
IMAGE_NAME="curldock"
VERSION=$(git describe --tags --always --dirty)-testing
DOCKERHUB_USERNAME="inigoetxaniz"

echo "Building testing version: ${VERSION}"

# Build Docker image
echo "Building Docker image..."
docker build -t ${IMAGE_NAME}:${VERSION} -f docker/Dockerfile \
  --build-arg BUILD_MODE=testing \
  .

# Tag the image for DockerHub
docker tag ${IMAGE_NAME}:${VERSION} ${DOCKERHUB_USERNAME}/${IMAGE_NAME}:${VERSION}
docker tag ${IMAGE_NAME}:${VERSION} ${DOCKERHUB_USERNAME}/${IMAGE_NAME}:testing

# Push to DockerHub
echo "Pushing to DockerHub..."
docker push ${DOCKERHUB_USERNAME}/${IMAGE_NAME}:${VERSION}
docker push ${DOCKERHUB_USERNAME}/${IMAGE_NAME}:testing

echo "Testing build and push completed successfully."

# Build for multiple architectures
echo "Building testing version for multiple architectures..."
docker buildx create --use
docker buildx build --platform linux/amd64,linux/arm64 \
  -t ${DOCKERHUB_USERNAME}/${IMAGE_NAME}:${VERSION} \
  -t ${DOCKERHUB_USERNAME}/${IMAGE_NAME}:testing \
  -f docker/Dockerfile \
  --build-arg BUILD_MODE=testing \
  --push .

# Remove intermediate containers
docker buildx rm

echo "Multi-architecture testing build and push completed successfully."