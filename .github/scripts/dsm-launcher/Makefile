
BINARY_NAME=dsm-launcher
TARGET=x86_64-unknown-linux-musl
DIST_DIR=dist
ARCHIVE_NAME=$(BINARY_NAME)-$(TARGET).tar.gz

.PHONY: all build release archive clean

all: release archive

build:
	cross build --release --target $(TARGET)

release: build
	mkdir -p $(DIST_DIR)
	cp target/$(TARGET)/release/$(BINARY_NAME) $(DIST_DIR)/

archive: release
	cd $(DIST_DIR) && tar -czvf $(ARCHIVE_NAME) $(BINARY_NAME)

clean:
	rm -rf $(DIST_DIR)
	cargo clean
