#include <array>
#include <cstdint>

#include "common/checksum.h"
#include "gtest/gtest.h"

using namespace kuzu::common;

TEST(ChecksumTest, EmptyBufferUsesSeedValue) {
    auto buffer = std::array<uint8_t, 1>{0};
    EXPECT_EQ(checksum(buffer.data(), 0), 5381);
}

TEST(ChecksumTest, StableForMixedBlockAndTailInput) {
    auto buffer = std::array<uint8_t, 19>{
        114, 117, 115, 116, 45, 111, 119, 110, 101, 100, 45, 99, 104, 101, 99, 107, 115, 117,
        109};
    EXPECT_EQ(checksum(buffer.data(), buffer.size()), 17544447021567405525ULL);
}
