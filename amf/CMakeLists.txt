project(amf)

set(AMF_DIR ${CMAKE_SOURCE_DIR}/externals/AMF_v1.4.29)
set(AMF_COMMON_DIR ${AMF_DIR}/amf/public/common)

# Source files
set(SOURCES
    ${AMF_COMMON_DIR}/AMFFactory.cpp
    ${AMF_COMMON_DIR}/AMFSTL.cpp
    ${AMF_COMMON_DIR}/Thread.cpp
    ${AMF_COMMON_DIR}/TraceAdapter.cpp
    ${AMF_COMMON_DIR}/Windows/ThreadWindows.cpp
)

# Include directories
set(AMF_INCLUDE_DIRS
    ${AMF_DIR}/amf
    ${AMF_COMMON_DIR}
)
include_directories(${AMF_INCLUDE_DIRS})

# Build target
add_library(amf STATIC ${SOURCES})
