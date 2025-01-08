# Makefile
.PHONY: all build test clean

# Build everything
all: engine lua mobile

# Build specific components
engine:
    cd engine && cargo build

lua:
    cd lua-compact && make

mobile:
    cd mobile && npm install && npm run build

# Run all tests
test:
    cd engine && cargo test
    cd lua-compact && make test
    cd mobile && npm test

# Clean all builds
clean:
    cd engine && cargo clean
    cd lua-compact && make clean
    cd mobile && rm -rf node_modules build
