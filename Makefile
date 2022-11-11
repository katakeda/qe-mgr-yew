IMAGE=qe-mgr-yew
CONTAINER=qe-mgr-yew
LOCAL_PORT=8080
REMOTE_PORT=8080
LOCAL_DIST_DIR=$(shell pwd)/dist
REMOTE_DIST_DIR=/usr/local/bin/dist
SRC=.

.PHONY: run
run:
	docker container run --name $(CONTAINER) \
	-p $(LOCAL_PORT):$(REMOTE_PORT) \
	-v $(LOCAL_DIST_DIR):${REMOTE_DIST_DIR} \
	-d $(IMAGE)

.PHONY: build
build:
	docker image build -t $(IMAGE) $(SRC)

.PHONY: clean
clean:
	docker container rm -f $(CONTAINER)
	docker image rm -f $(IMAGE)