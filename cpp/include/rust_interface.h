#ifndef RUST_PROCESSOR_H
#define RUST_PROCESSOR_H

#include <cstdint>

#ifdef __cplusplus
extern "C" {
#endif

// The new function signature with a path and a return value.
int32_t save_image_rust(const uint8_t* data, uint32_t width, uint32_t height, uint8_t channels, const char* output_path);

#ifdef __cplusplus
}
#endif

#endif // RUST_PROCESSOR_H
