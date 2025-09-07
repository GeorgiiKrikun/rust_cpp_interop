#include <cstdint>
#include "rust_interface.h"
#include <iostream>
#include "opencv4/opencv2/opencv.hpp"

int main(int argc, char** argv) {
    if (argc != 3) {
        std::cerr << "Usage: " << argv[0] << " <input_image_path> <output_path>" << std::endl;
        return -1;
    }

    const char* image_path = argv[1];
    const char* output_path = argv[2];

    // Load the image using OpenCV
    cv::Mat img = cv::imread(image_path);
    if (img.empty()) {
        std::cerr << "Failed to load image: " << image_path << std::endl;
        return -1;
    }

    const uint32_t w = img.cols;
    const uint32_t h = img.rows;
    const uint8_t channels = img.channels();

    // Call the Rust function to save the image
    int32_t result = save_image_rust(img.data, w, h, channels, output_path);
    if (result != 0) {
        std::cerr << "Failed to save image via Rust function." << std::endl;
        return -1;
    }

    std::cout << "Image saved successfully to " << output_path << std::endl;
    return 0;

}
