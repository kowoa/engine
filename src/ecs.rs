use std::marker::PhantomData;

use specs::prelude::*;

pub struct Ecs<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Ecs<'a, 'b> {
    pub fn dispatch(&mut self) {
        self.dispatcher.dispatch(&self.world);
    }
}

pub trait EcsBuilderState {}

pub struct WithRunner;
impl EcsBuilderState for WithRunner {}

pub struct WithoutRunner;
impl EcsBuilderState for WithoutRunner {}

pub struct EcsBuilder<'a, 'b, E: EcsBuilderState> {
    world: World,
    dispatcher_builder: DispatcherBuilder<'a, 'b>,
    runner: Option<fn(Ecs<'a, 'b>)>,
    state: PhantomData<E>
}

// Common methods for EcsBuilder
impl<'a, 'b, E: EcsBuilderState> EcsBuilder<'a, 'b, E> {
    pub fn insert_resource<R>(mut self, resource: R) -> Self
        where R: Resource {
        self.world.insert(resource);
        self
    }

    pub fn add_plugin<P>(mut self, plugin: P) -> Self
        where P: Plugin {
        plugin.build(&mut self);
        self
    }
    
    pub fn add_system<S>(mut self, system: S, name: &str, dep: &[&str]) -> Self
        where S: for<'c> System<'c> + Send + 'a {
        self.dispatcher_builder.add(system, name, dep);
        self
    }
    
    pub fn set_runner(self, runner: fn(Ecs<'a, 'b>)) -> EcsBuilder<'a, 'b, WithRunner> {
        EcsBuilder {
            world: self.world,
            dispatcher_builder: self.dispatcher_builder,
            runner: Some(runner),
            state: PhantomData,
        }
    }
}

// Methods for EcsBuilder in the WithRunner state
impl EcsBuilder<'_, '_, WithRunner> {
    pub fn run(self) {
        let mut world = self.world;
        let mut dispatcher = self.dispatcher_builder.build();
        dispatcher.setup(&mut world);
        (self.runner.unwrap())(Ecs { world, dispatcher });
    }
}

// Methods for EcsBuilder in the WithoutRunner state
impl<'a, 'b> EcsBuilder<'a, 'b, WithoutRunner> {
    pub fn new() -> Self {
        EcsBuilder {
            world: World::new(),
            dispatcher_builder: DispatcherBuilder::new(),
            runner: None,
            state: PhantomData,
        }
    }

    pub fn build(self) -> Ecs<'a, 'b> {
        let mut world = self.world;
        let mut dispatcher = self.dispatcher_builder.build();
        dispatcher.setup(&mut world);
        Ecs { world, dispatcher }
    }
}

pub trait Plugin {
    /// Configure the Ecs to which this plugin is added
    fn build<E>(&self, ecs: &mut EcsBuilder<E>) where E: EcsBuilderState;
}
