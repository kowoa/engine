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

pub struct Ecs {
    world: World,
    runner: fn(World),
}

impl Ecs {
    pub fn run(self) {
        (self.runner)(self.world);
    }
}

pub trait EcsBuilderState {}

pub struct Incomplete;
impl EcsBuilderState for Incomplete {}

pub struct Complete;
impl EcsBuilderState for Complete {}

pub struct EcsBuilder<E: EcsBuilderState> {
    world: World,
    schedules: Schedules,
    runner: Option<fn(World)>,
    state: PhantomData<E>
}

// EcsBuilder can only transition to the Complete state once set_runner is called.
impl EcsBuilder<Incomplete> {
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
            }, StartupSingleThreaded)
            .add_schedule(Schedule::new(), Startup)
            .add_schedule(Schedule::new(), PreUpdate)
            .add_schedule(Schedule::new(), Update)
            .add_schedule({
                let mut render = Schedule::new();
                render.set_executor_kind(ExecutorKind::SingleThreaded);
                render
            }, Render)
    }
    

    // Transition to the Complete state once runner is set
    pub fn set_runner(self, runner: fn(World)) -> EcsBuilder<Complete> {
        EcsBuilder {
            world: self.world,
            schedules: self.schedules,
            runner: Some(runner),
            state: PhantomData,
        }
    }

    pub fn insert_resource<R: Resource>(mut self, resource: R) -> Self {
        self.world.insert_resource(resource);
        self
    }
    
    pub fn insert_non_send_resource<R: Resource>(mut self, resource: R) -> Self {
        self.world.insert_non_send_resource(resource);
        self
    }
    
    pub fn add_plugin<P: Plugin>(self, plugin: P) -> Self {
        plugin.build(self)
    }
    
    pub fn add_schedule(mut self, schedule: Schedule, label: impl ScheduleLabel) -> Self {
        let label_clone = label.dyn_clone();
        if self.schedules.insert(label, schedule).is_some() {
            panic!("schedule with label {label_clone:?} already exists");
        }
        self
    }
    
    pub fn add_system<S>(mut self,
        system: impl IntoSystemConfig<S>,
        label: impl ScheduleLabel
    ) -> Self {
        let schedule = self.schedules.get_mut(&label)
            .unwrap_or_else(|| panic!("schedule with label {label:?} does not exist"));
        schedule.add_system(system);
        self
    }
}

// Methods for EcsBuilder in the Complete state
impl EcsBuilder<Complete> {
    pub fn build(mut self) -> Ecs {
        self.world.insert_resource(self.schedules);
        Ecs {
            world: self.world,
            runner: self.runner.unwrap(),
        }
    }
}

pub trait Plugin {
    /// Configure the Ecs to which this plugin is added.
    /// The plugin will not be able to call `ecs_builder.run()`.
    fn build(&self, ecs_builder: EcsBuilder<Incomplete>) -> EcsBuilder<Incomplete>;
}
