#pragma once

#include <cstdint>

#include "common/rust_ffi.h"

extern "C" uint64_t kudzu_common_checksum(kudzu::ffi::KudzuSliceU8 buffer);

