cmake_minimum_required(VERSION 3.15)
project(mallib CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED True)

file(GLOB SRC_FILES
    "src/**/*.cpp"
    "src/*.cpp"
)

add_executable(mallib ${SRC_FILES})

find_package(PkgConfig REQUIRED)
target_include_directories(mallib PRIVATE ${CMAKE_SOURCE_DIR}/include)

add_custom_target(run
    COMMAND mallib
    DEPENDS mallib
    WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
    COMMENT "Running project..."
)