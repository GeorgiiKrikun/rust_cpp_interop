#include "img_saver.h"
#include <opencv2/opencv.hpp>
#include <iostream>
#include <vector>

int32_t save_image_from_rust(const uint8_t* data,
                               int32_t width,
                               int32_t height,
                               int32_t channels,
                               const char* output_path) {
    int cv_type = 0;
    if (channels == 3) {
        // Assuming BGR format, which is OpenCV's default for color images
        cv_type = CV_8UC3;
    } else if (channels == 4) {
        // Assuming BGRA format
        cv_type = CV_8UC4;
    } else if (channels == 1) {
        cv_type = CV_8UC1;
    } else {
        std::cerr << "Unsupported number of channels: " << channels << std::endl;
        return -1; // Indicate failure
    }

    try {
        // Create an OpenCV Mat header for the existing data.
        // This does NOT copy the data. It just points to the Rust-managed memory.
        cv::Mat image(height, width, cv_type, const_cast<uint8_t*>(data));

        // OpenCV's imread/highgui often works with BGR, but the `image` crate
        // in Rust typically loads as RGB. If we get 3 channels, we need to swap
        // the Red and Blue channels before saving for correct colors.
        cv::Mat image_to_save;
        if (channels == 3) {
            cv::cvtColor(image, image_to_save, cv::COLOR_RGB2BGR);
        } else if (channels == 4) {
            cv::cvtColor(image, image_to_save, cv::COLOR_RGBA2BGRA);
        } else {
            image_to_save = image;
        }

        bool success = cv::imwrite(output_path, image_to_save);

        if (success) {
            std::cout << "[C++] Successfully saved image to " << output_path << std::endl;
            return 0; // Success
        } else {
            std::cerr << "[C++] Failed to save image to " << output_path << std::endl;
            return -1; // Failure
        }
    } catch (const cv::Exception& e) {
        std::cerr << "[C++] OpenCV Error: " << e.what() << std::endl;
        return -1; // Failure
    }
}

void print_user_from_rust(const UserProfile* user) {
   if (!user) {
       std::cout << "[C++] Received a null pointer!" << std::endl;
       return;
   }
   std::cout << "[C++] --- User Profile ---" << std::endl;
   std::cout << "[C++] ID: " << user->user_id << std::endl;
   std::cout << "[C++] Score: " << user->score << std::endl;
   std::cout << "[C++] Active: " << (user->is_active ? "yes" : "no") << std::endl;
   std::cout << "[C++] Name: " << (user->name ? user->name : "null") << std::endl;
   std::cout << "[C++] --------------------" << std::endl;
}
