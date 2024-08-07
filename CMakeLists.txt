if (MSVC)
 cmake_minimum_required(VERSION 3.26)
else()
 cmake_minimum_required(VERSION 3.29)
endif()
set(CMAKE_BUILD_TYPE Debug)
option(BUILD_RELEASE "BUILD_RELEASE" ON)
if(BUILD_RELEASE)
 set(CMAKE_BUILD_TYPE Release)
endif()

project (nostril C)

include_directories(${CMAKE_SOURCE_DIR}/deps/secp256k1/include)
add_subdirectory(${CMAKE_SOURCE_DIR}/deps/secp256k1)


#//////////////////////////
#windows.h header file has macros for min and max, nostril defines max
#//////////////////////////

if (MSVC)
  add_definitions(-DNOMINMAX)
  add_definitions(-D_CRT_SECURE_NO_WARNINGS)
  add_definitions(-D_CRT_NONSTDC_NO_DEPRECATE)
endif()

#//////////////////////////
#sources
#//////////////////////////

set(src ${src})
set(src ${src} include/hex.h)
set(src ${src} include/proof.h)
set(src ${src} include/cursor.h)
set(src ${src} include/endian.h)
set(src ${src} include/random.h)
set(src ${src} include/sha256.h)
set(src ${src} src/sha256.c)
set(src ${src} include/base64.h)
set(src ${src} src/base64.c)
set(src ${src} include/aes.h)
set(src ${src} src/aes.c)
## set(src ${src} src/simple.c)
if (MSVC)
  set(src ${src} include/clock_gettime.h)
endif()

#//////////////////////////
#link with libraries
#lib_dep contains a cascade definition of all the libraries needed to link
#//////////////////////////

set(lib_dep ${lib_dep})
set(lib_dep ${lib_dep} secp256k1)
if (MSVC)
  set(lib_dep ${lib_dep} ${CMAKE_SOURCE_DIR}/deps/secp256k1/src/Debug/libsecp256k1.lib)
  set(lib_dep ${lib_dep} Bcrypt.lib)
else()
  ## set(lib_dep ${lib_dep} ${CMAKE_SOURCE_DIR}/deps/secp256k1/src/libsecp256k1.a)
endif()


#//////////////////////////
#executables
#//////////////////////////

add_executable(simple ${src} ${CMAKE_SOURCE_DIR}/src/simple.c)
target_link_libraries (simple ${lib_dep})

# tool to generate file 'config.h' , generate from a shell with
# configurator.exe > include/config.h
add_executable(configurator ${CMAKE_SOURCE_DIR}/src/configurator.c)

add_executable(nostril ${src} ${CMAKE_SOURCE_DIR}/src/nostril.c)
target_link_libraries (nostril ${lib_dep})

add_executable(gnostr ${src} ${CMAKE_SOURCE_DIR}/src/nostril.c)
target_link_libraries (gnostr ${lib_dep})

#//////////////////////////
# generate  config.h
#//////////////////////////

add_custom_command(
    TARGET configurator
    POST_BUILD
    COMMAND configurator > config.h
    COMMAND ${CMAKE_COMMAND} -E copy "${CMAKE_SOURCE_DIR}/config.h" ${CMAKE_SOURCE_DIR}
    COMMENT "generating include/config.h"
)

#//////////////////////////
# Install
#//////////////////////////

install(TARGETS simple
        CONFIGURATIONS Debug
        RUNTIME DESTINATION ${CMAKE_SOURCE_DIR}/bin)
install(TARGETS simple
        CONFIGURATIONS Release
        RUNTIME DESTINATION /usr/local/bin)
install(TARGETS simple
        CONFIGURATIONS RelWithDebInfo
        RUNTIME DESTINATION /usr/local/bin)

install(TARGETS nostril
        CONFIGURATIONS Debug
        RUNTIME DESTINATION ${CMAKE_SOURCE_DIR}/bin)
install(TARGETS nostril
        CONFIGURATIONS Release
        RUNTIME DESTINATION /usr/local/bin)
install(TARGETS nostril
        CONFIGURATIONS RelWithDebInfo
        RUNTIME DESTINATION /usr/local/bin)

install(TARGETS gnostr
        CONFIGURATIONS Debug
        RUNTIME DESTINATION ${CMAKE_SOURCE_DIR}/bin)
install(TARGETS gnostr
        CONFIGURATIONS Release
        RUNTIME DESTINATION /usr/local/bin)
install(TARGETS gnostr
        CONFIGURATIONS RelWithDebInfo
        RUNTIME DESTINATION /usr/local/bin)
