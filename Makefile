dev:
	cargo run --features bevy/dynamic_linking

check:
	cargo check --features bevy/dynamic_linking

clippy:
	cargo clippy --features bevy/dynamic_linking

.PHONY: dev check
