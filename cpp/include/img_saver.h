#ifndef IMAGE_SAVER_H
#define IMAGE_SAVER_H

#include <cstdint>
#include "user_profile.h"

#ifdef __cplusplus
extern "C" {
#endif

int32_t save_image_from_rust(const uint8_t* data,
                               int32_t width,
                               int32_t height,
                               int32_t channels,
                               const char* output_path);
void print_user_from_rust(const UserProfile* user);

#ifdef __cplusplus
}
#endif

#endif // IMAGE_SAVER_H
