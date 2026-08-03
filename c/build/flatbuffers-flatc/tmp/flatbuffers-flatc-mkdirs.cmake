# Distributed under the OSI-approved BSD 3-Clause License.  See accompanying
# file Copyright.txt or https://cmake.org/licensing for details.

cmake_minimum_required(VERSION ${CMAKE_VERSION}) # this file comes with cmake

# If CMAKE_DISABLE_SOURCE_CHANGES is set to true and the source directory is an
# existing directory in our source tree, calling file(MAKE_DIRECTORY) on it
# would cause a fatal error, even though it would be a no-op.
if(NOT EXISTS "C:/Users/marcn/Desktop/litert-lm-rust/c/build/flatbuffers")
  file(MAKE_DIRECTORY "C:/Users/marcn/Desktop/litert-lm-rust/c/build/flatbuffers")
endif()
file(MAKE_DIRECTORY
  "C:/Users/marcn/Desktop/litert-lm-rust/c/build/flatbuffers-flatc/src/flatbuffers-flatc-build"
  "C:/Users/marcn/Desktop/litert-lm-rust/c/build/flatbuffers-flatc"
  "C:/Users/marcn/Desktop/litert-lm-rust/c/build/flatbuffers-flatc/tmp"
  "C:/Users/marcn/Desktop/litert-lm-rust/c/build/flatbuffers-flatc/src/flatbuffers-flatc-stamp"
  "C:/Users/marcn/Desktop/litert-lm-rust/c/build/flatbuffers-flatc/src"
  "C:/Users/marcn/Desktop/litert-lm-rust/c/build/flatbuffers-flatc/src/flatbuffers-flatc-stamp"
)

set(configSubDirs Debug;Release;MinSizeRel;RelWithDebInfo)
foreach(subDir IN LISTS configSubDirs)
    file(MAKE_DIRECTORY "C:/Users/marcn/Desktop/litert-lm-rust/c/build/flatbuffers-flatc/src/flatbuffers-flatc-stamp/${subDir}")
endforeach()
if(cfgdir)
  file(MAKE_DIRECTORY "C:/Users/marcn/Desktop/litert-lm-rust/c/build/flatbuffers-flatc/src/flatbuffers-flatc-stamp${cfgdir}") # cfgdir has leading slash
endif()
