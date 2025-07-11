// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/components/tensor_data.fbs".

#pragma once

#include "../datatypes/tensor_data.hpp"
#include "../result.hpp"

#include <cstdint>
#include <memory>
#include <utility>

namespace rerun::components {
    /// **Component**: An N-dimensional array of numbers.
    ///
    /// The number of dimensions and their respective lengths is specified by the `shape` field.
    /// The dimensions are ordered from outermost to innermost. For example, in the common case of
    /// a 2D RGB Image, the shape would be `[height, width, channel]`.
    ///
    /// These dimensions are combined with an index to look up values from the `buffer` field,
    /// which stores a contiguous array of typed values.
    struct TensorData {
        rerun::datatypes::TensorData data;

      public: // START of extensions from tensor_data_ext.cpp:
        /// New tensor data from shape and tensor buffer.
        ///
        /// \param shape Shape of the tensor.
        /// \param buffer The tensor buffer containing the tensor's data.
        TensorData(rerun::Collection<uint64_t> shape, rerun::datatypes::TensorBuffer buffer)
            : data(rerun::datatypes::TensorData(std::move(shape), std::move(buffer))) {}

        /// New tensor data from dimensions and pointer to tensor data.
        ///
        /// Type must be one of the types supported by `rerun::datatypes::TensorData`.
        /// \param shape Shape of the tensor. Determines the number of elements expected to be in `data_`.
        /// \param data_ Target of the pointer must outlive the archetype.
        template <typename TElement>
        explicit TensorData(Collection<uint64_t> shape, const TElement* data_)
            : data(rerun::datatypes::TensorData(std::move(shape), data_)) {}

        // END of extensions from tensor_data_ext.cpp, start of generated code:

      public:
        TensorData() = default;

        TensorData(rerun::datatypes::TensorData data_) : data(std::move(data_)) {}

        TensorData& operator=(rerun::datatypes::TensorData data_) {
            data = std::move(data_);
            return *this;
        }

        /// Cast to the underlying TensorData datatype
        operator rerun::datatypes::TensorData() const {
            return data;
        }
    };
} // namespace rerun::components

namespace rerun {
    static_assert(sizeof(rerun::datatypes::TensorData) == sizeof(components::TensorData));

    /// \private
    template <>
    struct Loggable<components::TensorData> {
        static constexpr std::string_view ComponentName = "rerun.components.TensorData";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype() {
            return Loggable<rerun::datatypes::TensorData>::arrow_datatype();
        }

        /// Serializes an array of `rerun::components::TensorData` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const components::TensorData* instances, size_t num_instances
        ) {
            if (num_instances == 0) {
                return Loggable<rerun::datatypes::TensorData>::to_arrow(nullptr, 0);
            } else if (instances == nullptr) {
                return rerun::Error(
                    ErrorCode::UnexpectedNullArgument,
                    "Passed array instances is null when num_elements> 0."
                );
            } else {
                return Loggable<rerun::datatypes::TensorData>::to_arrow(
                    &instances->data,
                    num_instances
                );
            }
        }
    };
} // namespace rerun
