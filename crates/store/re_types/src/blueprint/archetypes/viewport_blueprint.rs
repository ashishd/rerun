// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/viewport_blueprint.fbs".

#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::try_serialize_field;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch as _, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: The top-level description of the viewport.
///
/// ⚠️ **This type is _unstable_ and may change significantly in a way that the data won't be backwards compatible.**
#[derive(Clone, Debug, Default)]
pub struct ViewportBlueprint {
    /// The layout of the views
    pub root_container: Option<SerializedComponentBatch>,

    /// Show one tab as maximized?
    pub maximized: Option<SerializedComponentBatch>,

    /// Whether the viewport layout is determined automatically.
    ///
    /// If `true`, the container layout will be reset whenever a new view is added or removed.
    /// This defaults to `false` and is automatically set to `false` when there is user determined layout.
    pub auto_layout: Option<SerializedComponentBatch>,

    /// Whether or not views should be created automatically.
    ///
    /// If `true`, the viewer will only add views that it hasn't considered previously (as identified by `past_viewer_recommendations`)
    /// and which aren't deemed redundant to existing views.
    /// This defaults to `false` and is automatically set to `false` when the user adds views manually in the viewer.
    pub auto_views: Option<SerializedComponentBatch>,

    /// Hashes of all recommended views the viewer has already added and that should not be added again.
    ///
    /// This is an internal field and should not be set usually.
    /// If you want the viewer from stopping to add views, you should set `auto_views` to `false`.
    ///
    /// The viewer uses this to determine whether it should keep adding views.
    pub past_viewer_recommendations: Option<SerializedComponentBatch>,
}

impl ViewportBlueprint {
    /// Returns the [`ComponentDescriptor`] for [`Self::root_container`].
    ///
    /// The corresponding component is [`crate::blueprint::components::RootContainer`].
    #[inline]
    pub fn descriptor_root_container() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ViewportBlueprint".into()),
            component_name: Some("rerun.blueprint.components.RootContainer".into()),
            archetype_field_name: "root_container".into(),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::maximized`].
    ///
    /// The corresponding component is [`crate::blueprint::components::ViewMaximized`].
    #[inline]
    pub fn descriptor_maximized() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ViewportBlueprint".into()),
            component_name: Some("rerun.blueprint.components.ViewMaximized".into()),
            archetype_field_name: "maximized".into(),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::auto_layout`].
    ///
    /// The corresponding component is [`crate::blueprint::components::AutoLayout`].
    #[inline]
    pub fn descriptor_auto_layout() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ViewportBlueprint".into()),
            component_name: Some("rerun.blueprint.components.AutoLayout".into()),
            archetype_field_name: "auto_layout".into(),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::auto_views`].
    ///
    /// The corresponding component is [`crate::blueprint::components::AutoViews`].
    #[inline]
    pub fn descriptor_auto_views() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ViewportBlueprint".into()),
            component_name: Some("rerun.blueprint.components.AutoViews".into()),
            archetype_field_name: "auto_views".into(),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::past_viewer_recommendations`].
    ///
    /// The corresponding component is [`crate::blueprint::components::ViewerRecommendationHash`].
    #[inline]
    pub fn descriptor_past_viewer_recommendations() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ViewportBlueprint".into()),
            component_name: Some("rerun.blueprint.components.ViewerRecommendationHash".into()),
            archetype_field_name: "past_viewer_recommendations".into(),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: None,
            component_name: None,
            archetype_field_name: "rerun.blueprint.components.ViewportBlueprintIndicator".into(),
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [ViewportBlueprint::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ViewportBlueprint::descriptor_root_container(),
            ViewportBlueprint::descriptor_maximized(),
            ViewportBlueprint::descriptor_auto_layout(),
            ViewportBlueprint::descriptor_auto_views(),
            ViewportBlueprint::descriptor_past_viewer_recommendations(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 6usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ViewportBlueprint::descriptor_indicator(),
            ViewportBlueprint::descriptor_root_container(),
            ViewportBlueprint::descriptor_maximized(),
            ViewportBlueprint::descriptor_auto_layout(),
            ViewportBlueprint::descriptor_auto_views(),
            ViewportBlueprint::descriptor_past_viewer_recommendations(),
        ]
    });

impl ViewportBlueprint {
    /// The total number of components in the archetype: 0 required, 1 recommended, 5 optional
    pub const NUM_COMPONENTS: usize = 6usize;
}

/// Indicator component for the [`ViewportBlueprint`] [`::re_types_core::Archetype`]
pub type ViewportBlueprintIndicator = ::re_types_core::GenericIndicatorComponent<ViewportBlueprint>;

impl ::re_types_core::Archetype for ViewportBlueprint {
    type Indicator = ViewportBlueprintIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.ViewportBlueprint".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Viewport blueprint"
    }

    #[inline]
    fn indicator() -> SerializedComponentBatch {
        #[allow(clippy::unwrap_used)]
        ViewportBlueprintIndicator::DEFAULT
            .serialized(Self::descriptor_indicator())
            .unwrap()
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentDescriptor, arrow::array::ArrayRef)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_descr: ::nohash_hasher::IntMap<_, _> = arrow_data.into_iter().collect();
        let root_container = arrays_by_descr
            .get(&Self::descriptor_root_container())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_root_container())
            });
        let maximized = arrays_by_descr
            .get(&Self::descriptor_maximized())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_maximized())
            });
        let auto_layout = arrays_by_descr
            .get(&Self::descriptor_auto_layout())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_auto_layout())
            });
        let auto_views = arrays_by_descr
            .get(&Self::descriptor_auto_views())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_auto_views())
            });
        let past_viewer_recommendations = arrays_by_descr
            .get(&Self::descriptor_past_viewer_recommendations())
            .map(|array| {
                SerializedComponentBatch::new(
                    array.clone(),
                    Self::descriptor_past_viewer_recommendations(),
                )
            });
        Ok(Self {
            root_container,
            maximized,
            auto_layout,
            auto_views,
            past_viewer_recommendations,
        })
    }
}

impl ::re_types_core::AsComponents for ViewportBlueprint {
    #[inline]
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            self.root_container.clone(),
            self.maximized.clone(),
            self.auto_layout.clone(),
            self.auto_views.clone(),
            self.past_viewer_recommendations.clone(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for ViewportBlueprint {}

impl ViewportBlueprint {
    /// Create a new `ViewportBlueprint`.
    #[inline]
    pub fn new() -> Self {
        Self {
            root_container: None,
            maximized: None,
            auto_layout: None,
            auto_views: None,
            past_viewer_recommendations: None,
        }
    }

    /// Update only some specific fields of a `ViewportBlueprint`.
    #[inline]
    pub fn update_fields() -> Self {
        Self::default()
    }

    /// Clear all the fields of a `ViewportBlueprint`.
    #[inline]
    pub fn clear_fields() -> Self {
        use ::re_types_core::Loggable as _;
        Self {
            root_container: Some(SerializedComponentBatch::new(
                crate::blueprint::components::RootContainer::arrow_empty(),
                Self::descriptor_root_container(),
            )),
            maximized: Some(SerializedComponentBatch::new(
                crate::blueprint::components::ViewMaximized::arrow_empty(),
                Self::descriptor_maximized(),
            )),
            auto_layout: Some(SerializedComponentBatch::new(
                crate::blueprint::components::AutoLayout::arrow_empty(),
                Self::descriptor_auto_layout(),
            )),
            auto_views: Some(SerializedComponentBatch::new(
                crate::blueprint::components::AutoViews::arrow_empty(),
                Self::descriptor_auto_views(),
            )),
            past_viewer_recommendations: Some(SerializedComponentBatch::new(
                crate::blueprint::components::ViewerRecommendationHash::arrow_empty(),
                Self::descriptor_past_viewer_recommendations(),
            )),
        }
    }

    /// The layout of the views
    #[inline]
    pub fn with_root_container(
        mut self,
        root_container: impl Into<crate::blueprint::components::RootContainer>,
    ) -> Self {
        self.root_container =
            try_serialize_field(Self::descriptor_root_container(), [root_container]);
        self
    }

    /// Show one tab as maximized?
    #[inline]
    pub fn with_maximized(
        mut self,
        maximized: impl Into<crate::blueprint::components::ViewMaximized>,
    ) -> Self {
        self.maximized = try_serialize_field(Self::descriptor_maximized(), [maximized]);
        self
    }

    /// Whether the viewport layout is determined automatically.
    ///
    /// If `true`, the container layout will be reset whenever a new view is added or removed.
    /// This defaults to `false` and is automatically set to `false` when there is user determined layout.
    #[inline]
    pub fn with_auto_layout(
        mut self,
        auto_layout: impl Into<crate::blueprint::components::AutoLayout>,
    ) -> Self {
        self.auto_layout = try_serialize_field(Self::descriptor_auto_layout(), [auto_layout]);
        self
    }

    /// Whether or not views should be created automatically.
    ///
    /// If `true`, the viewer will only add views that it hasn't considered previously (as identified by `past_viewer_recommendations`)
    /// and which aren't deemed redundant to existing views.
    /// This defaults to `false` and is automatically set to `false` when the user adds views manually in the viewer.
    #[inline]
    pub fn with_auto_views(
        mut self,
        auto_views: impl Into<crate::blueprint::components::AutoViews>,
    ) -> Self {
        self.auto_views = try_serialize_field(Self::descriptor_auto_views(), [auto_views]);
        self
    }

    /// Hashes of all recommended views the viewer has already added and that should not be added again.
    ///
    /// This is an internal field and should not be set usually.
    /// If you want the viewer from stopping to add views, you should set `auto_views` to `false`.
    ///
    /// The viewer uses this to determine whether it should keep adding views.
    #[inline]
    pub fn with_past_viewer_recommendations(
        mut self,
        past_viewer_recommendations: impl IntoIterator<
            Item = impl Into<crate::blueprint::components::ViewerRecommendationHash>,
        >,
    ) -> Self {
        self.past_viewer_recommendations = try_serialize_field(
            Self::descriptor_past_viewer_recommendations(),
            past_viewer_recommendations,
        );
        self
    }
}

impl ::re_byte_size::SizeBytes for ViewportBlueprint {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.root_container.heap_size_bytes()
            + self.maximized.heap_size_bytes()
            + self.auto_layout.heap_size_bytes()
            + self.auto_views.heap_size_bytes()
            + self.past_viewer_recommendations.heap_size_bytes()
    }
}
