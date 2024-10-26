if [ -z "$DOCKER_REGISTRY" ]; then
    echo "Error: DOCKER_REGISTRY environment variable is not set"
    exit 1
fi

# Установка имени образа
IMAGE_NAME="${DOCKER_REGISTRY}/shuller-bot"
VERSION="${VERSION:-latest}"

echo "Building image: ${IMAGE_NAME}:${VERSION}"

# Проверка наличия buildx
if ! docker buildx version > /dev/null 2>&1; then
    echo "Error: docker buildx is not available"
    exit 1
fi

# Создание и использование buildx builder
docker buildx create --use --name shuller-builder || true

# Сборка и пуш для обеих архитектур
docker buildx build --platform linux/amd64,linux/arm64 \
  -t "${IMAGE_NAME}:${VERSION}" \
  --push \
  --build-arg BUILD_DATE="$(date -u +'%Y-%m-%dT%H:%M:%SZ')" \
  --build-arg VCS_REF="$(git rev-parse --short HEAD)" \
  .

echo "Build completed successfully"
