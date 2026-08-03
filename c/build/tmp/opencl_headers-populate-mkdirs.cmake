# Distributed under the OSI-approved BSD 3-Clause License.  See accompanying
# file Copyright.txt or https://cmake.org/licensing for details.

cmake_minimum_required(VERSION ${CMAKE_VERSION}) # this file comes with cmake

# If CMAKE_DISABLE_SOURCE_CHANGES is set to true and the source directory is an
# existing directory in our source tree, calling file(MAKE_DIRECTORY) on it
# would cause a fatal error, even though it would be a no-op.
if(NOT EXISTS "C:/Users/marcn/Desktop/litert-lm-rust/c/build/opencl_headers")
  file(MAKE_DIRECTORY "C:/Users/marcn/Desktop/litert-lm-rust/c/build/opencl_headers")
endif()
file(MAKE_DIRECTORY
  "C:/Users/marcn/Desktop/litert-lm-rust/c/build/_deps/opencl_headers-build"
  "C:/Users/marcn/Desktop/litert-lm-rust/c/build"
  "C:/Users/marcn/Desktop/litert-lm-rust/c/build/tmp"
  "C:/Users/marcn/Desktop/litert-lm-rust/c/build/src/opencl_headers-populate-stamp"
  "C:/Users/marcn/Desktop/litert-lm-rust/c/build/src"
  "C:/Users/marcn/Desktop/litert-lm-rust/c/build/src/opencl_headers-populate-stamp"
)

set(configSubDirs Debug)
foreach(subDir IN LISTS configSubDirs)
    file(MAKE_DIRECTORY "C:/Users/marcn/Desktop/litert-lm-rust/c/build/src/opencl_headers-populate-stamp/${subDir}")
endforeach()
if(cfgdir)
  file(MAKE_DIRECTORY "C:/Users/marcn/Desktop/litert-lm-rust/c/build/src/opencl_headers-populate-stamp${cfgdir}") # cfgdir has leading slash
endif()
