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
  "C:/Users/marcn/Desktop/litert-lm-rust/c/build/_deps/flatbuffers-build"
  "C:/Users/marcn/Desktop/litert-lm-rust/c/build/_deps/flatbuffers-subbuild/flatbuffers-populate-prefix"
  "C:/Users/marcn/Desktop/litert-lm-rust/c/build/_deps/flatbuffers-subbuild/flatbuffers-populate-prefix/tmp"
  "C:/Users/marcn/Desktop/litert-lm-rust/c/build/_deps/flatbuffers-subbuild/flatbuffers-populate-prefix/src/flatbuffers-populate-stamp"
  "C:/Users/marcn/Desktop/litert-lm-rust/c/build/_deps/flatbuffers-subbuild/flatbuffers-populate-prefix/src"
  "C:/Users/marcn/Desktop/litert-lm-rust/c/build/_deps/flatbuffers-subbuild/flatbuffers-populate-prefix/src/flatbuffers-populate-stamp"
)

set(configSubDirs Debug)
foreach(subDir IN LISTS configSubDirs)
    file(MAKE_DIRECTORY "C:/Users/marcn/Desktop/litert-lm-rust/c/build/_deps/flatbuffers-subbuild/flatbuffers-populate-prefix/src/flatbuffers-populate-stamp/${subDir}")
endforeach()
if(cfgdir)
  file(MAKE_DIRECTORY "C:/Users/marcn/Desktop/litert-lm-rust/c/build/_deps/flatbuffers-subbuild/flatbuffers-populate-prefix/src/flatbuffers-populate-stamp${cfgdir}") # cfgdir has leading slash
endif()
