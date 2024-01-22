# Compile crate to static website
build:
    wasm-pack build --target web

# Serve static website via HTTP server daemon
serve: build
    darkhttpd . --pidfile ./httpd.pid --daemon

# Open served website in web browser
visit:
    firefox localhost:8080

# Kill HTTP server daemon
kill:
    pkill httpd

# Run unit tests in web browser
test:
    wasm-pack test --headless --firefox

# Remove all temporary files
clean:
    rm -rf pkg
    rm -rf target
