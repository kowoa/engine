use std::marker::PhantomData;
use bevy_ecs::{prelude::*, schedule::ScheduleLabel};

pub type Ecs = World;

pub trait EcsBuilderState {}

pub struct WithoutRunner;
impl EcsBuilderState for WithoutRunner {}

pub struct WithRunner;
impl EcsBuilderState for WithRunner {}

pub struct EcsBuilder<E: EcsBuilderState> {
    world: World,
    schedules: Schedules,
    runner: Option<fn(Ecs)>,
    state: PhantomData<E>
}

// Methods for EcsBuilder in the WithoutRunner state
impl EcsBuilder<WithoutRunner> {
    pub fn new() -> Self {
        EcsBuilder {
            world: World::new(),
            schedules: Schedules::new(),
            runner: None,
            state: PhantomData,
        }
    }

    pub fn set_runner(self, runner: fn(Ecs)) -> EcsBuilder<WithRunner> {
        EcsBuilder {
            world: self.world,
            schedules: self.schedules,
            runner: Some(runner),
            state: PhantomData,
        }
    }

    pub fn add_resource<R: Resource>(mut self, resource: R) -> Self {
        self.world.insert_resource(resource);
        self
    }
    
    pub fn add_plugin<P: Plugin>(self, plugin: P) -> Self {
        plugin.build(self)
    }
    
    pub fn add_schedule(mut self, label: impl ScheduleLabel) -> Self {
        self.schedules.insert(label, Schedule::new());
        self
    }
    
    pub fn add_system<S>(mut self,
        system: impl IntoSystemConfig<S>,
        label: impl ScheduleLabel
    ) -> Self {
        let schedule = self.schedules.get_mut(&label).unwrap();
        schedule.add_system(system);
        self
    }
}

// Methods for EcsBuilder in the WithRunner state
impl EcsBuilder<WithRunner> {
    pub fn run(mut self) {
        self.world.insert_resource(self.schedules);
        (self.runner.unwrap())(self.world);
    }
}

pub trait Plugin {
    /// Configure the Ecs to which this plugin is added
    fn build<E>(&self, ecs_builder: EcsBuilder<E>) -> EcsBuilder<E>
        where E: EcsBuilderState;
}
