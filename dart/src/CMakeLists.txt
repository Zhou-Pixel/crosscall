# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

project(crosscall_library VERSION 0.0.1 LANGUAGES C)

add_library(crosscall SHARED
  "rinf.c"
)

set_target_properties(crosscall PROPERTIES
  PUBLIC_HEADER rinf.h
  OUTPUT_NAME "crosscall"
)

target_compile_definitions(crosscall PUBLIC DART_SHARED_LIB)
