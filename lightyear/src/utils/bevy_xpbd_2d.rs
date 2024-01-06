//! Implement lightyear traits for some common bevy types
use crate::_reexport::{InterpolatedComponent, LinearInterpolation};
use crate::client::components::{ComponentSyncMode, SyncComponent};
use crate::client::interpolation::InterpFn;
use bevy::prelude::Entity;
use bevy::utils::EntityHashSet;
use bevy_xpbd_2d::components::*;
use std::ops::{Add, Mul};
use tracing::{info, trace};

use crate::prelude::{EntityMapper, MapEntities, Message, Named};

pub mod position {
    use super::*;
    impl Named for Position {
        fn name(&self) -> &'static str {
            "Position"
        }
    }

    pub struct PositionLinearInterpolation;

    impl InterpFn<Position> for PositionLinearInterpolation {
        fn lerp(start: Position, other: Position, t: f32) -> Position {
            let res = Position::new(start.0 * (1.0 - t) + other.0 * t);
            trace!(
                "position lerp: start: {:?} end: {:?} t: {} res: {:?}",
                start,
                other,
                t,
                res
            );
            res
        }
    }

    impl InterpolatedComponent<Position> for Position {
        type Fn = PositionLinearInterpolation;
    }

    impl<'a> MapEntities<'a> for Position {
        fn map_entities(&mut self, entity_mapper: Box<dyn EntityMapper + 'a>) {}

        fn entities(&self) -> EntityHashSet<Entity> {
            EntityHashSet::default()
        }
    }

    impl Message for Position {}

    impl SyncComponent for Position {
        fn mode() -> ComponentSyncMode {
            ComponentSyncMode::Full
        }
    }
}

pub mod rotation {
    use super::*;
    impl Named for Rotation {
        fn name(&self) -> &'static str {
            "Rotation"
        }
    }

    pub struct RotationLinearInterpolation;

    impl InterpFn<Rotation> for RotationLinearInterpolation {
        fn lerp(start: Rotation, other: Rotation, t: f32) -> Rotation {
            let res =
                Rotation::from_degrees(start.as_degrees() * (1.0 - t) + other.as_degrees() * t);
            trace!(
                "rotation lerp: start: {:?} end: {:?} t: {} res: {:?}",
                start,
                other,
                t,
                res
            );
            res
        }
    }

    impl InterpolatedComponent<Rotation> for Rotation {
        type Fn = RotationLinearInterpolation;
    }

    impl<'a> MapEntities<'a> for Rotation {
        fn map_entities(&mut self, entity_mapper: Box<dyn EntityMapper + 'a>) {}

        fn entities(&self) -> EntityHashSet<Entity> {
            EntityHashSet::default()
        }
    }

    impl Message for Rotation {}

    impl SyncComponent for Rotation {
        fn mode() -> ComponentSyncMode {
            ComponentSyncMode::Full
        }
    }
}

pub mod linear_velocity {
    use super::*;
    impl Named for LinearVelocity {
        fn name(&self) -> &'static str {
            "LinearVelocity"
        }
    }

    pub struct LinearVelocityLinearInterpolation;

    impl InterpFn<LinearVelocity> for LinearVelocityLinearInterpolation {
        fn lerp(start: LinearVelocity, other: LinearVelocity, t: f32) -> LinearVelocity {
            let res = LinearVelocity(start.0 * (1.0 - t) + other.0 * t);
            trace!(
                "linear velocity lerp: start: {:?} end: {:?} t: {} res: {:?}",
                start,
                other,
                t,
                res
            );
            res
        }
    }

    impl InterpolatedComponent<LinearVelocity> for LinearVelocity {
        type Fn = LinearVelocityLinearInterpolation;
    }

    impl<'a> MapEntities<'a> for LinearVelocity {
        fn map_entities(&mut self, entity_mapper: Box<dyn EntityMapper + 'a>) {}

        fn entities(&self) -> EntityHashSet<Entity> {
            EntityHashSet::default()
        }
    }

    impl Message for LinearVelocity {}

    impl SyncComponent for LinearVelocity {
        fn mode() -> ComponentSyncMode {
            ComponentSyncMode::Full
        }
    }
}

pub mod angular_velocity {
    use super::*;
    impl Named for AngularVelocity {
        fn name(&self) -> &'static str {
            "AngularVelocity"
        }
    }

    pub struct AngularVelocityAngularInterpolation;

    impl InterpFn<AngularVelocity> for AngularVelocityAngularInterpolation {
        fn lerp(start: AngularVelocity, other: AngularVelocity, t: f32) -> AngularVelocity {
            let res = AngularVelocity(start.0 * (1.0 - t) + other.0 * t);
            trace!(
                "angular velocity lerp: start: {:?} end: {:?} t: {} res: {:?}",
                start,
                other,
                t,
                res
            );
            res
        }
    }

    impl InterpolatedComponent<AngularVelocity> for AngularVelocity {
        type Fn = AngularVelocityAngularInterpolation;
    }

    impl<'a> MapEntities<'a> for AngularVelocity {
        fn map_entities(&mut self, entity_mapper: Box<dyn EntityMapper + 'a>) {}

        fn entities(&self) -> EntityHashSet<Entity> {
            EntityHashSet::default()
        }
    }

    impl Message for AngularVelocity {}

    impl SyncComponent for AngularVelocity {
        fn mode() -> ComponentSyncMode {
            ComponentSyncMode::Full
        }
    }
}

// TODO: in some cases the mass does not change, but in others it doesn't!
//  this is an example of where we don't why to attach the interpolation to the component type,
//  but instead do it per entity?
pub mod mass {
    use super::*;
    impl Named for Mass {
        fn name(&self) -> &'static str {
            "Mass"
        }
    }
    impl<'a> MapEntities<'a> for Mass {
        fn map_entities(&mut self, entity_mapper: Box<dyn EntityMapper + 'a>) {}

        fn entities(&self) -> EntityHashSet<Entity> {
            EntityHashSet::default()
        }
    }

    impl Message for Mass {}

    impl SyncComponent for Mass {
        fn mode() -> ComponentSyncMode {
            ComponentSyncMode::Once
        }
    }
}