SOURCE_DIR = $(PWD)
BUILD_DIR = ~/tmp/$(shell basename '$(SOURCE_DIR)')

all: format run

build: sync
	cd $(BUILD_DIR) && cargo build

doc: sync
	cd $(BUILD_DIR) && cargo doc --open

run: sync
	cd $(BUILD_DIR) && cargo run $(ARGS)

run_add: sync
	cd $(BUILD_DIR) && cargo run add

run_view: sync
	cd $(BUILD_DIR) && cargo run view

run_plan: sync
	cd $(BUILD_DIR) && cargo run plan --date 2022-15-22

run_behold: sync
	cd $(BUILD_DIR) && cargo run behold

test: sync
	cd $(BUILD_DIR) && cargo test -- --color always --nocapture

sync:
	mkdir -p $(BUILD_DIR)
	rsync -av '$(SOURCE_DIR)/' $(BUILD_DIR)/ --exclude .git --exclude target

fetch:
	rsync -av $(BUILD_DIR)/recipes ./
	rsync -av $(BUILD_DIR)/plans ./

format:
	rustfmt src/*

build_and_fetch_release: sync
	rm -rf ./target
	cd $(BUILD_DIR) && cargo build --release 
	rsync -av $(BUILD_DIR)/target ./

clean: 
	rm -rf $(BUILD_DIR)
	rm -rf ./target
