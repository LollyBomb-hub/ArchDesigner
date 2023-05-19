project(PackDesigner)

set(CMAKE_CXX_STANDARD 20)

cmake_minimum_required(VERSION 3.18)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif ()

if (WIN32)
    set(CLIENT_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/pack-designer.exe")
else()
    set(CLIENT_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/pack-designer")
endif()

set(CARGO_TARGET_DIR "${CMAKE_CURRENT_BINARY_DIR}")

add_custom_target(pack-deisgner ALL
        COMMENT "Compiling client module"
        COMMAND ${CARGO_CMD} --target-dir ${CARGO_TARGET_DIR}
        COMMAND cp ${CLIENT_SO} ${CMAKE_CURRENT_SOURCE_DIR}
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
set_target_properties(pack-deisgner PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})

find_package(jsoncpp CONFIG REQUIRED)

#add_library(first_algo SHARED toolkit/first_algo/main.cpp)
add_executable(first_algo toolkit/first_algo/main.cpp)
IF (WIN32)
    target_link_libraries(first_algo PRIVATE jsoncpp_lib jsoncpp_object JsonCpp::JsonCpp)
ELSE ()
    target_link_libraries(first_algoPRIVATE jsoncpp_object jsoncpp_static JsonCpp::JsonCpp)
ENDIF ()