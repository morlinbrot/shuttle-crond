default:
	just -lu

run:
	cargo shuttle run

debug:
	RUST_LOG="shuttle_crond=debug" cargo shuttle run