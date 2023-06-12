use std::{marker::PhantomData};
use bevy_ecs::{prelude::*, schedule::{ScheduleLabel, ExecutorKind}};


/* Schedule Labels */
#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct StartupSingleThreaded;

#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct Startup;

#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct PreUpdate;

#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct Update;

#[derive(ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone)]
pub struct Render;
/* -------------- */


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
            .add_schedule({
                let mut startup_st = Schedule::new();
                startup_st.set_executor_kind(ExecutorKind::SingleThreaded);
                startup_st
            }, StartupSingleThreaded).unwrap()
            .add_schedule(Schedule::new(), Startup).unwrap()
            .add_schedule(Schedule::new(), PreUpdate).unwrap()
            .add_schedule(Schedule::new(), Update).unwrap()
            .add_schedule(Schedule::new(), Render).unwrap()
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
    
    pub fn add_schedule(mut self, schedule: Schedule, label: impl ScheduleLabel) -> Result<Self, &'static str> {
        match self.schedules.insert(label, schedule) {
            Some(_) => Err("schedule with label already exists"),
            None => Ok(self),
        }
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
