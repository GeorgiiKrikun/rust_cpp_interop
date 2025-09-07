set dotenv-load := true

mod cpp
mod rust "rust/executable"

test:
  @echo "Running tests..."
  @echo "CPP Build Directory: ${CPP_BUILD_DIR}"

