// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/testing/components/enum_test.fbs".

#pragma once

#include "enum_test.hpp"
#include "valued_enum.hpp"

#include <cstdint>
#include <memory>
#include <optional>
#include <rerun/result.hpp>

namespace arrow {
    class Array;
    class DataType;
    class StructBuilder;
} // namespace arrow

namespace rerun::datatypes {
    struct MultiEnum {
        /// The first value.
        rerun::datatypes::EnumTest value1;

        /// The second value.
        std::optional<rerun::datatypes::ValuedEnum> value2;

      public:
        MultiEnum() = default;
    };
} // namespace rerun::datatypes

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<datatypes::MultiEnum> {
        static constexpr std::string_view ComponentName = "rerun.testing.datatypes.MultiEnum";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Serializes an array of `rerun::datatypes::MultiEnum` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const datatypes::MultiEnum* instances, size_t num_instances
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::StructBuilder* builder, const datatypes::MultiEnum* elements, size_t num_elements
        );
    };
} // namespace rerun
