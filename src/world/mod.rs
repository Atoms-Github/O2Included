use ggez::Context;

mod pitch;
mod dupe;
mod device;

pub struct World{
    pub pitches: Vec<pitch::Pitch>,
    pub dupes: Vec<dupe::Dupe>,
}
impl World{
    pub fn new() -> World{
        World{

        }
    }
    pub fn draw(&self, ctx: &mut Context){

    }
    pub fn update(&mut self, delta: f32){
        for dupe in &mut self.dupes{

        }
    }
}