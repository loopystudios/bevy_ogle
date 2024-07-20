use bevy::{ecs::world::Command, prelude::*};

use crate::{OgleMode, OgleTarget};

pub enum OgleCommand {
    Target(OgleTarget),
    ChangeMode(OgleMode),
}

pub trait OgleCommandExt {
    fn ogle_target(&mut self, target: OgleTarget);
    fn ogle_target_position(&mut self, pos: Vec2);
    fn ogle_target_entity(&mut self, entity: Entity);
    fn ogle_target_entity_with_offset(&mut self, entity: Entity, offset: Vec2);
    fn ogle_clear_target(&mut self);
    fn ogle_mode(&mut self, mode: OgleMode);
    fn ogle_freeze(&mut self);
    fn ogle_follow(&mut self);
    fn ogle_pancam(&mut self);
}

impl<'w, 's> OgleCommandExt for Commands<'w, 's> {
    fn ogle_target(&mut self, target: OgleTarget) {
        self.add(OgleCommand::Target(target));
    }

    fn ogle_target_position(&mut self, pos: Vec2) {
        self.add(OgleCommand::Target(OgleTarget::Position(pos)));
    }

    fn ogle_target_entity(&mut self, entity: Entity) {
        self.add(OgleCommand::Target(OgleTarget::Entity(entity)));
    }

    fn ogle_target_entity_with_offset(&mut self, entity: Entity, offset: Vec2) {
        self.add(OgleCommand::Target(OgleTarget::EntityWithOffset((
            entity, offset,
        ))));
    }

    fn ogle_clear_target(&mut self) {
        self.add(OgleCommand::Target(OgleTarget::None));
    }

    fn ogle_mode(&mut self, mode: OgleMode) {
        self.add(OgleCommand::ChangeMode(mode));
    }

    fn ogle_freeze(&mut self) {
        self.add(OgleCommand::ChangeMode(OgleMode::Frozen));
    }

    fn ogle_follow(&mut self) {
        self.add(OgleCommand::ChangeMode(OgleMode::Following));
    }

    fn ogle_pancam(&mut self) {
        self.add(OgleCommand::ChangeMode(OgleMode::Pancam));
    }
}

impl Command for OgleCommand {
    fn apply(self, world: &mut World) {
        match self {
            OgleCommand::Target(target) => {
                let mut next_target = world.resource_mut::<OgleTarget>();
                *next_target = target;
            }
            OgleCommand::ChangeMode(mode) => {
                let mut next_mode = world.resource_mut::<NextState<OgleMode>>();
                next_mode.set(mode);
            }
        }
    }
}
