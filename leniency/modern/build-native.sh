#!/usr/bin/env bash
# Builds the native decoders/encoders used by the webp/jp2/jxl adapters from github.com sources under /tmp/claude-0/src3.
# Idempotent. Pinned commits are the ones the recorded runs used. System libwebp 1.5.0 (apt) needs no build.
set -euo pipefail
S=/tmp/claude-0/src3; mkdir -p $S; cd $S
fetch() { # dir url sha
  if [ ! -d "$1/.git" ]; then git init -q "$1"; git -C "$1" remote add origin "$2"; fi
  git -C "$1" fetch -q --depth 1 origin "$3"; git -C "$1" checkout -q FETCH_HEAD
}
if [ ! -x cm/bin/cmake ]; then
  curl -sL -o cmake.tgz https://github.com/Kitware/CMake/releases/download/v3.31.6/cmake-3.31.6-linux-x86_64.tar.gz
  mkdir -p cm && tar xzf cmake.tgz -C cm --strip-components=1
fi
export PATH=$S/cm/bin:$PATH
if [ ! -f opj-inst/lib/libopenjp2.a ]; then
  fetch openjpeg https://github.com/uclouvain/openjpeg 8314119b067c0fc77834731168daaebd379fdb12
  mkdir -p opj-build && (cd opj-build && cmake ../openjpeg -DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=OFF -DBUILD_CODEC=OFF -DBUILD_TESTING=OFF -DCMAKE_INSTALL_PREFIX=$S/opj-inst >cmake.log 2>&1 && make -j2 install >make.log 2>&1)
fi
if [ ! -f jxl-inst/lib/libjxl.a ]; then
  fetch libjxl https://github.com/libjxl/libjxl eaae1abb2478c23b2fc487359619740cb4b72259
  git -C libjxl submodule update -q --init --recursive --depth 1
  mkdir -p jxl-build && (cd jxl-build && cmake ../libjxl -DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=OFF -DBUILD_TESTING=OFF -DJPEGXL_ENABLE_TOOLS=OFF -DJPEGXL_ENABLE_JPEGLI=OFF -DJPEGXL_ENABLE_DOXYGEN=OFF -DJPEGXL_ENABLE_MANPAGES=OFF -DJPEGXL_ENABLE_BENCHMARK=OFF -DJPEGXL_ENABLE_EXAMPLES=OFF -DJPEGXL_ENABLE_VIEWERS=OFF -DJPEGXL_ENABLE_PLUGINS=OFF -DJPEGXL_ENABLE_SJPEG=OFF -DJPEGXL_ENABLE_OPENEXR=OFF -DJPEGXL_ENABLE_SKCMS=ON -DJPEGXL_FORCE_SYSTEM_BROTLI=ON -DJPEGXL_ENABLE_TRANSCODE_JPEG=ON -DCMAKE_INSTALL_PREFIX=$S/jxl-inst >cmake.log 2>&1 && make -j2 install >make.log 2>&1)
fi
if [ ! -f jasper-inst/lib/libjasper.a ]; then
  fetch jasper https://github.com/jasper-software/jasper 5ce57f67e0011e0d2483992a5d77a092b1b64eb7
  mkdir -p jasper-build && (cd jasper-build && cmake ../jasper -DCMAKE_BUILD_TYPE=Release -DJAS_ENABLE_SHARED=OFF -DJAS_ENABLE_PROGRAMS=OFF -DJAS_ENABLE_DOC=OFF -DJAS_ENABLE_LATEX=OFF -DALLOW_IN_SOURCE_BUILD=ON -DCMAKE_INSTALL_PREFIX=$S/jasper-inst >cmake.log 2>&1 && make -j2 install >make.log 2>&1)
fi
