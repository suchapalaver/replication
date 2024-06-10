CONTAINER_TOOL := $(shell which docker > /dev/null 2>&1 && echo docker || echo podman)
IMAGE_NAME = replication-image

build:
	$(CONTAINER_TOOL) build -t $(IMAGE_NAME) .

run:
	$(CONTAINER_TOOL) run -it -p 50051:50051 --env-file .env $(IMAGE_NAME)

clean:
	$(CONTAINER_TOOL) stop $$( $(CONTAINER_TOOL) ps -aq --filter ancestor=$(IMAGE_NAME)) || true
	$(CONTAINER_TOOL) rm $$( $(CONTAINER_TOOL) ps -aq --filter ancestor=$(IMAGE_NAME)) || true
	$(CONTAINER_TOOL) rmi $(IMAGE_NAME) || true

.PHONY: build run clean
