use bevy::{ecs::world::Command, prelude::*};

use crate::{OgleMode, OgleTarget};

pub enum OgleCommand {
    TargetPosition(Vec2),
    TargetEntity(Entity),
    ClearTarget,
    ChangeMode(OgleMode),
}

pub trait OgleCommandExt {
    fn ogle_target_position(&mut self, pos: Vec2);
    fn ogle_target_entity(&mut self, entity: Entity);
    fn ogle_clear_target(&mut self);
    fn ogle_change_mode(&mut self, mode: OgleMode);
}

impl<'w, 's> OgleCommandExt for Commands<'w, 's> {
    fn ogle_target_position(&mut self, pos: Vec2) {
        self.add(OgleCommand::TargetPosition(pos));
    }

    fn ogle_target_entity(&mut self, entity: Entity) {
        self.add(OgleCommand::TargetEntity(entity));
    }

    fn ogle_clear_target(&mut self) {
        self.add(OgleCommand::ClearTarget);
    }

    fn ogle_change_mode(&mut self, mode: OgleMode) {
        self.add(OgleCommand::ChangeMode(mode));
    }
}

impl Command for OgleCommand {
    fn apply(self, world: &mut World) {
        match self {
            OgleCommand::TargetEntity(entity) => {
                let mut target = world.resource_mut::<OgleTarget>();
                *target = OgleTarget::Entity(entity);
            }
            OgleCommand::TargetPosition(pos) => {
                let mut target = world.resource_mut::<OgleTarget>();
                *target = OgleTarget::Position(pos);
            }
            OgleCommand::ClearTarget => {
                let mut target = world.resource_mut::<OgleTarget>();
                *target = OgleTarget::None;
            }
            OgleCommand::ChangeMode(mode) => {
                let mut next_mode = world.resource_mut::<NextState<OgleMode>>();
                next_mode.set(mode);
            }
        }
    }
}
