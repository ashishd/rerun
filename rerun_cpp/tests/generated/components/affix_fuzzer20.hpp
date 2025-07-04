// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/testing/components/fuzzy.fbs".

#pragma once

#include "../datatypes/affix_fuzzer20.hpp"

#include <cstdint>
#include <memory>
#include <rerun/result.hpp>
#include <utility>

namespace rerun::components {
    struct AffixFuzzer20 {
        rerun::datatypes::AffixFuzzer20 nested_transparent;

      public:
        AffixFuzzer20() = default;

        AffixFuzzer20(rerun::datatypes::AffixFuzzer20 nested_transparent_)
            : nested_transparent(std::move(nested_transparent_)) {}

        AffixFuzzer20& operator=(rerun::datatypes::AffixFuzzer20 nested_transparent_) {
            nested_transparent = std::move(nested_transparent_);
            return *this;
        }

        /// Cast to the underlying AffixFuzzer20 datatype
        operator rerun::datatypes::AffixFuzzer20() const {
            return nested_transparent;
        }
    };
} // namespace rerun::components

namespace rerun {
    static_assert(sizeof(rerun::datatypes::AffixFuzzer20) == sizeof(components::AffixFuzzer20));

    /// \private
    template <>
    struct Loggable<components::AffixFuzzer20> {
        static constexpr std::string_view ComponentName = "rerun.testing.components.AffixFuzzer20";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype() {
            return Loggable<rerun::datatypes::AffixFuzzer20>::arrow_datatype();
        }

        /// Serializes an array of `rerun::components::AffixFuzzer20` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const components::AffixFuzzer20* instances, size_t num_instances
        ) {
            if (num_instances == 0) {
                return Loggable<rerun::datatypes::AffixFuzzer20>::to_arrow(nullptr, 0);
            } else if (instances == nullptr) {
                return rerun::Error(
                    ErrorCode::UnexpectedNullArgument,
                    "Passed array instances is null when num_elements> 0."
                );
            } else {
                return Loggable<rerun::datatypes::AffixFuzzer20>::to_arrow(
                    &instances->nested_transparent,
                    num_instances
                );
            }
        }
    };
} // namespace rerun
