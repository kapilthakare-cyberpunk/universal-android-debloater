# Justfile for Universal Android Debloater
# Install just: https://github.com/casey/just
# Usage: just <recipe>

# ── Default ───────────────────────────────────────────────────────────
default:
    just check

# ── Development ───────────────────────────────────────────────────────

# Check code compiles without errors
check:
    cargo check --all-features

# Format all code
fmt:
    cargo fmt --all

# Run linter
lint:
    cargo clippy --all-features -- -D warnings

# Run all tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Run the application
run:
    cargo run

# Full CI pipeline: format, check, lint, test
ci:
    just fmt
    just check
    just lint
    just test

# ── Building ──────────────────────────────────────────────────────────

# Build release binary (default: WGPU/Vulkan)
build:
    cargo build --release

# Build with OpenGL support (older hardware)
build-opengl:
    cargo build --release --no-default-features --features glow,self-update

# Build without self-update (for package managers)
build-no-update:
    cargo build --release --no-default-features --features wgpu,no-self-update

# ── Packaging ─────────────────────────────────────────────────────────

# Create macOS .app bundle
app:
    chmod +x scripts/package-macos.sh
    ./scripts/package-macos.sh wgpu

# Create macOS .app bundle (OpenGL)
app-opengl:
    chmod +x scripts/package-macos.sh
    ./scripts/package-macos.sh glow

# Create macOS .dmg installer
dmg: app
    chmod +x scripts/create-dmg.sh
    ./scripts/create-dmg.sh wgpu

# Create macOS .dmg installer (OpenGL)
dmg-opengl: app-opengl
    chmod +x scripts/create-dmg.sh
    ./scripts/create-dmg.sh glow

# Create Windows installer
windows:
    powershell -ExecutionPolicy Bypass -File scripts/package-windows.ps1 wgpu

# Create Windows installer (OpenGL)
windows-opengl:
    powershell -ExecutionPolicy Bypass -File scripts/package-windows.ps1 glow

# Create Linux .deb + .AppImage
linux:
    chmod +x scripts/package-linux.sh
    ./scripts/package-linux.sh wgpu

# Create Linux packages (OpenGL)
linux-opengl:
    chmod +x scripts/package-linux.sh
    ./scripts/package-linux.sh glow

# Build and package for ALL platforms (local only — CI does this automatically)
release: dmg windows linux
    @echo ""
    @echo "=== All platform packages created ==="
    @echo "Check the dist/ directory for output files."

# ── Maintenance ───────────────────────────────────────────────────────

# Clean build artifacts
clean:
    cargo clean

# Clean dist directory
clean-dist:
    rm -rf dist

# Check for security vulnerabilities
audit:
    cargo audit

# Update dependencies
update-deps:
    cargo update

# Generate documentation
doc:
    cargo doc --no-deps --open

# ── Info ──────────────────────────────────────────────────────────────

# Show project info
info:
    @echo "=== Universal Android Debloater ==="
    @echo "Rust version: $(rustc --version)"
    @echo "Cargo version: $(cargo --version)"
    @echo ""
    @echo "Available recipes:"
    @just --list

# Show dist contents
dist-info:
    @echo "=== dist/ contents ==="
    @ls -lh dist/ 2>/dev/null || echo "dist/ is empty or doesn't exist yet"
