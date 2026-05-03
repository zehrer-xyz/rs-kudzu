#pragma once

#include <cstddef>
#include <cstdint>

namespace kudzu::ffi {

struct KudzuSliceU8 {
    const uint8_t* data;
    size_t len;
};

enum class KudzuStatusCode : uint32_t { OK = 0, ERROR = 1 };

struct KudzuStringView {
    const uint8_t* data;
    size_t len;
};

struct KudzuStatus {
    KudzuStatusCode code;
    KudzuStringView message;
};

} // namespace kudzu::ffi

