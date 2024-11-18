cmake_minimum_required(VERSION 3.29)
project(2gtr C)

set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED ON)

find_package(ICU COMPONENTS uc i18n REQUIRED)

include_directories(${ICU_INCLUDE_DIRS})

add_executable(2gtr main.c)

#target_link_libraries(2gtr PRIVATE ICU::uc ICU::i18n)

# Link ICU libraries
target_link_libraries(2gtr PRIVATE
        ICU::uc   # Core ICU library
        ICU::i18n # Internationalization library
)

# Include ICU headers
target_include_directories(2gtr PRIVATE ${ICU_INCLUDE_DIRS})

# If you're on macOS, you might need to link against CoreFoundation
if(APPLE)
    target_link_libraries(2gtr PRIVATE "-framework CoreFoundation")
endif()

# Set ICU-specific compile definitions
target_compile_definitions(2gtr PRIVATE
        U_DISABLE_RENAMING=1  # Disable ICU renaming, use "u_" prefixes
        U_STATIC_IMPLEMENTATION=1  # If using ICU as a static library
)