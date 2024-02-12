# CMakeLists.txt

project(video_codec_sdk)

# Include directories
set(FFNVCodec_INCLUDE_DIR ${CMAKE_SOURCE_DIR}/externals/nv-codec-headers_n11.1.5.2/include/ffnvcodec)
set(SDK_DIR ${CMAKE_SOURCE_DIR}/externals/Video_Codec_SDK_11.1.5)

set(VideoCodecSDK_INCLUDE_DIRS
    ${SDK_DIR}
    ${SDK_DIR}/Interface
    ${SDK_DIR}/Samples/Utils
    ${SDK_DIR}/Samples/NvCodec
    ${SDK_DIR}/Samples/NvCodec/NVEncoder
    ${SDK_DIR}/Samples/NvCodec/NVDecoder
)

include_directories(${FFNVCodec_INCLUDE_DIR})
include_directories(${VideoCodecSDK_INCLUDE_DIRS})

# Source files
set(SOURCES
    ${SDK_DIR}/Samples/NvCodec/NvEncoder/NvEncoder.cpp
    ${SDK_DIR}/Samples/NvCodec/NvEncoder/NvEncoderCuda.cpp
    ${SDK_DIR}/Samples/NvCodec/NvEncoder/NvEncoderD3D11.cpp
    ${SDK_DIR}/Samples/NvCodec/NvDecoder/NvDecoder.cpp
)

# Build target
add_library(video_codec_sdk STATIC ${SOURCES})
