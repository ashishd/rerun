// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/components/apply_latest_at.fbs".

#pragma once

#include "../../datatypes/bool.hpp"
#include "../../result.hpp"

#include <cstdint>
#include <memory>

namespace rerun::blueprint::components {
    /// **Component**: Whether empty cells in a dataframe should be filled with a latest-at query.
    ///
    /// ⚠ **This type is _unstable_ and may change significantly in a way that the data won't be backwards compatible.**
    ///
    struct ApplyLatestAt {
        rerun::datatypes::Bool apply_latest_at;

      public:
        ApplyLatestAt() = default;

        ApplyLatestAt(rerun::datatypes::Bool apply_latest_at_)
            : apply_latest_at(apply_latest_at_) {}

        ApplyLatestAt& operator=(rerun::datatypes::Bool apply_latest_at_) {
            apply_latest_at = apply_latest_at_;
            return *this;
        }

        ApplyLatestAt(bool value_) : apply_latest_at(value_) {}

        ApplyLatestAt& operator=(bool value_) {
            apply_latest_at = value_;
            return *this;
        }

        /// Cast to the underlying Bool datatype
        operator rerun::datatypes::Bool() const {
            return apply_latest_at;
        }
    };
} // namespace rerun::blueprint::components

namespace rerun {
    static_assert(sizeof(rerun::datatypes::Bool) == sizeof(blueprint::components::ApplyLatestAt));

    /// \private
    template <>
    struct Loggable<blueprint::components::ApplyLatestAt> {
        static constexpr std::string_view ComponentName =
            "rerun.blueprint.components.ApplyLatestAt";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype() {
            return Loggable<rerun::datatypes::Bool>::arrow_datatype();
        }

        /// Serializes an array of `rerun::blueprint:: components::ApplyLatestAt` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const blueprint::components::ApplyLatestAt* instances, size_t num_instances
        ) {
            if (num_instances == 0) {
                return Loggable<rerun::datatypes::Bool>::to_arrow(nullptr, 0);
            } else if (instances == nullptr) {
                return rerun::Error(
                    ErrorCode::UnexpectedNullArgument,
                    "Passed array instances is null when num_elements> 0."
                );
            } else {
                return Loggable<rerun::datatypes::Bool>::to_arrow(
                    &instances->apply_latest_at,
                    num_instances
                );
            }
        }
    };
} // namespace rerun
