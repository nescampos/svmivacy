#!/bin/bash
set -e

# Create logs directory if it doesn't exist
LOGS_DIR="logs"
mkdir -p $LOGS_DIR

# Set timestamp for log files
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
LOG_FILE="$LOGS_DIR/test_run_$TIMESTAMP.log"

# Function to log messages to both console and file
log() {
    echo "[$(date +"%Y-%m-%d %H:%M:%S")] $1" | tee -a "$LOG_FILE"
}

# Function to handle errors
handle_error() {
    log "ERROR: Test execution failed at line $1"
    exit 1
}

# Set error trap
trap 'handle_error $LINENO' ERR

log "Starting test suite execution..."
log "Logging detailed output to: $LOG_FILE"

# Set Rust logging
export RUST_LOG=debug
export RUST_BACKTRACE=1

# Run unit tests
log "Running unit tests..."
(cd contracts && cargo test -- --nocapture) 2>&1 | tee -a "$LOG_FILE"

# Run Anchor tests
log "Running Anchor tests..."
(cd contracts && anchor test) 2>&1 | tee -a "$LOG_FILE"

# Check code formatting
log "Checking code formatting..."
(cd contracts && cargo fmt --all -- --check) 2>&1 | tee -a "$LOG_FILE"

# Run clippy for linting
log "Running linter..."
(cd contracts && cargo clippy -- -D warnings) 2>&1 | tee -a "$LOG_FILE"

# Print test summary
log "Test Summary:"
log "✓ Unit tests completed"
log "✓ Anchor tests completed"
log "✓ Format check completed"
log "✓ Linting completed"
log "All tests completed successfully!"

# Print log location
echo ""
echo "Detailed logs available at: $LOG_FILE"