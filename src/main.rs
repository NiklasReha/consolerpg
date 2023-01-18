use crossterm;
fn main() {
    
}

struct Player{
    direction:Direction,
    pos_x:usize,
    pos_y:usize,
    health_points:i32,
    intelligence:i32,
    charisma:i32,
    strength:i32,
    cuddlieness:f32,
    inventory:Inventory
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Event{
    name:String,
    choices:Vec<(String,Effect)>
}

struct Map{
    renderrange:usize,
    geograpy:Vec<Vec<(char,Option<Event>)>>
}

impl  Map {
    fn render_view(&self,pos_x:usize,pos_y:usize){
        
    }
    
    fn range_vertical(&self,pos_y:usize)->(usize,usize){
        let mut up:usize=self.renderrange;
        let mut down:usize=up;
        (up,down)
    }
}

struct Effect{

}

struct Inventory{
    items:Vec<(Item,i32)>
}

struct Item{
    name:String,
    health_points:i32,
    intelligence:i32,
    charisma:i32,
    strength:i32,
    cuddlieness:f32,
    effect:Effect
}