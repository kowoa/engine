use specs::prelude::*;

pub struct Ecs<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Ecs<'a, 'b> {
    pub fn new() -> Self {
        let mut world = World::new();
        let mut dispatcher = DispatcherBuilder::new().build();
        dispatcher.setup(&mut world);

        Self { world, dispatcher }
    }
    
    pub fn insert_resource<R>(&mut self, resource: R) where R: Resource {
        self.world.insert(resource);
    }
    
    pub fn dispatch(&mut self) {
        self.dispatcher.dispatch(&self.world);
    }
}