cmake_minimum_required(VERSION 3.10)
project(cv_im_saver LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 17)
include(GNUInstallDirs)

find_package(OpenCV REQUIRED)

add_library(cv_im_saver SHARED src/img_saver.cpp)
message(STATUS "Current source dir: ${CMAKE_CURRENT_SOURCE_DIR}")
target_include_directories(cv_im_saver PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include ${OpenCV_INCLUDE_DIRS})
target_link_libraries(cv_im_saver PUBLIC ${OpenCV_LIBS})

message(STATUS "Dependency " ${RUST_BUILD_DIR})

add_executable(cpp_executable src/executable.cpp)
target_include_directories(cpp_executable PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include ${OpenCV_INCLUDE_DIRS})
target_link_directories(cpp_executable PUBLIC ${RUST_BUILD_DIR})
target_link_libraries(cpp_executable PUBLIC rust_image_saver)
target_link_libraries(cpp_executable PUBLIC ${OpenCV_LIBS})




