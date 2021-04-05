use ggez::event::EventHandler;
use crate::AsAny;
use std::any::{TypeId, Any};
use std::collections::HashMap;
use ggez::{Context, GameResult};

#[derive(Eq, PartialEq)]
pub enum Priority {
    Low,
    Mid,
    High,
}

impl Priority {
    fn val(&self) -> u8 {
        match self {
            Priority::Low => 64,
            Priority::Mid => 128,
            Priority::High => 255,
        }
    }
}

pub trait GameComponent: EventHandler + AsAny {
    fn priority(&self) -> Priority;
}

pub trait GameComponentContainer {
    fn add_component(&mut self, new_component: impl GameComponent + 'static);

    fn find_component<T: 'static>(&self) -> Option<&T>;
    fn find_component_mut<T: 'static>(&mut self) -> Option<&mut T>;
}

#[derive(Default)]
pub struct GameState {
    components: HashMap<TypeId, Box<dyn GameComponent>>,
}

impl GameState {
    fn draw_by_priority(&mut self, _ctx: &mut Context, priority: Priority) -> GameResult {
        self.components.values_mut()
            .filter(|x| x.priority() == priority)
            .try_for_each(|x| x.draw(_ctx))
    }
}

impl GameComponentContainer for GameState {
    fn add_component(&mut self, new_component: impl GameComponent + 'static) {
        let component_type_id = new_component.type_id();
        if self.components.contains_key(&component_type_id) {
            return;
        }
        self.components
            .insert(component_type_id, Box::new(new_component));
    }

    fn find_component<T: 'static>(&self) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|x| (**x).as_any().downcast_ref::<T>())
    }

    fn find_component_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .and_then(|x| (**x).as_any_mut().downcast_mut::<T>())
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for component in self.components.values_mut() {
            component.update(_ctx)?
        }
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        ggez::graphics::clear(_ctx, ggez::graphics::BLACK);
        self.draw_by_priority(_ctx, Priority::Low)?;
        self.draw_by_priority(_ctx, Priority::Mid)?;
        self.draw_by_priority(_ctx, Priority::High)?;
        ggez::graphics::present(_ctx)
    }
}