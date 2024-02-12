# CMakeLists.txt

cmake_minimum_required(VERSION 3.12)
project(sdk)

set(CMAKE_CXX_STANDARD 11)

# 添加子目录
add_subdirectory(video_codec_sdk)
add_subdirectory(vpl)
add_subdirectory(amf)
