#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Check if the current commit is tagged
if [ -z "$(git tag --points-at HEAD)" ]; then
    echo "Error: Current commit is not tagged. Please tag the commit before running this script."
    echo "You can tag the current commit with:"
    echo "git tag -a v1.0.0 -m \"Version 1.0.0\""
    echo "git push origin v1.0.0"
    exit 1
else
    echo "Current commit is tagged. Proceeding with build and push."
fi

# Set variables
IMAGE_NAME="curldock"
VERSION=$(git describe --tags --always --dirty)
DOCKERHUB_USERNAME="your-dockerhub-username"  # Replace with your DockerHub username

# Build Docker image
echo "Building Docker image..."
docker build -t ${IMAGE_NAME}:${VERSION} .

# Tag the image for DockerHub
docker tag ${IMAGE_NAME}:${VERSION} ${DOCKERHUB_USERNAME}/${IMAGE_NAME}:${VERSION}
docker tag ${IMAGE_NAME}:${VERSION} ${DOCKERHUB_USERNAME}/${IMAGE_NAME}:latest

# Push to DockerHub
echo "Pushing to DockerHub..."
docker push ${DOCKERHUB_USERNAME}/${IMAGE_NAME}:${VERSION}
docker push ${DOCKERHUB_USERNAME}/${IMAGE_NAME}:latest

echo "Build and push completed successfully."

# Build for multiple architectures
echo "Building for multiple architectures..."
docker buildx create --use
docker buildx build --platform linux/amd64,linux/arm64,linux/arm/v7 \
  -t ${DOCKERHUB_USERNAME}/${IMAGE_NAME}:${VERSION} \
  -t ${DOCKERHUB_USERNAME}/${IMAGE_NAME}:latest \
  --push .

echo "Multi-architecture build and push completed successfully."
