PORT ?= 7878

CARGO := cargo

run-server:
	$(CARGO) run --bin server -- $(PORT)

run-client:
	$(CARGO) run --bin client -- $(PORT)