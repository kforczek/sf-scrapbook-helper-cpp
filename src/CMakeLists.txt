cmake_minimum_required(VERSION 3.10)

# Set project name
project(ScrapbookHelper)

# Set the C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Define the source files
set(SOURCES main.cpp)

# Include the header directory
include_directories(../api/target)

# Add the executable
add_executable(main ${SOURCES})

# Link with the sf_api library
target_link_libraries(main
    PRIVATE
    -L../api/target/release
    -lsf_api
    pthread
    dl
)

# Ensure that the linker can find the library
link_directories(../api/target/release)
