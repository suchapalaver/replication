CONTAINER_TOOL := $(shell which docker > /dev/null 2>&1 && echo docker || echo podman)
IMAGE ?= replication

PORT ?= 50051

build:
	$(CONTAINER_TOOL) build -t replicate-$(IMAGE) --build-arg PORT=$(PORT) .

run:
	$(CONTAINER_TOOL) run -it -p $(PORT):$(PORT) --env-file .env replicate-$(IMAGE)

clean:
	$(CONTAINER_TOOL) stop $$( $(CONTAINER_TOOL) ps -aq --filter ancestor=replicate-$(IMAGE)) || true
	$(CONTAINER_TOOL) rm $$( $(CONTAINER_TOOL) ps -aq --filter ancestor=replicate-$(IMAGE)) || true
	$(CONTAINER_TOOL) rmi replicate-$(IMAGE) || true

.PHONY: build run clean
