pub struct Player{
    anim: Option<Box<dyn State>>,
    velocity: (i32,i32),
    easing: Option<Box<dyn State>>,
}

impl Player{
    pub fn new() -> Player{
        Player{
        anim : Some(Box::new(Idle {})),
        velocity :(0,0),
        easing: Some(Box::new(Default {})),
        }
    }

    pub fn set_velocity(&mut self, x:i32, y:i32) {
        self.velocity = (x,y);
        if x != 0 || y !=0 {
            if let Some(s) = self.anim.take(){
                let (a, e) = s.start_move();
                self.anim = Some(a);
                self.easing = Some(e);
            }
        }
        else{
            if let Some(s) = self.anim.take(){
                let (a, e) = s.stop_move();
                self.anim = Some(a);
                self.easing = Some(e);
            }
        }
    }
    pub fn die(&mut self) {
        if let Some(s) = self.anim.take(){
            let (a, e) = s.die();
            self.anim = Some(a);
            self.easing = Some(e);
        }
        
    }
    pub fn attack(&mut self){
if let Some(s) = self.anim.take(){
            let (a, e) = s.attack();
            self.anim = Some(a);
            self.easing = Some(e);
        }
    }
    pub fn collect(&mut self){
        if let Some(s) = self.anim.take(){
            let (a, e) = s.collect();
            self.anim = Some(a);
            self.easing = Some(e);
        }
    }
    pub fn update(&mut self){
        if let Some(e) = self.easing.take(){
            if e.base_case(){
                if let Some(_) = self.anim.take(){
                    self.anim = Some(self.easing.take().expect("lol").new_self());
                    self.easing = Some(Box::new(Default {}));
                }
            }
        }
    }
    pub fn print(&self) -> &str{
        if let Some(s) = &self.anim{
            return s.print()
        }
        else{
            return "broken"
        }
    }
}

trait State {
    fn die(self: Box<Self>) -> (Box<dyn State>, Box<dyn State>){
        (Box::new(Dieing {}), Box::new(Default {}))
    }
    fn print <'a>(&self) -> & 'a str{
        "Default anim"
    }
    fn start_move(self: Box<Self>) -> (Box<dyn State>, Box <dyn State>){
        (Box::new(Running {}), Box::new(Default {}))
    }
    fn stop_move(self: Box<Self>) -> (Box<dyn State>, Box <dyn State>){
        (Box::new(Idle {}), Box::new(Default {}))
    }
    fn attack(self: Box<Self>) -> (Box<dyn State>, Box <dyn State>){
        (Box::new(Attacking{}), Box::new(Default {}))
    }
    fn collect(self: Box<Self>) -> (Box<dyn State>, Box <dyn State>){
        (Box::new(Collecting{}), Box::new(Default {}))
    }
    fn base_case(self: Box<Self>) -> bool{
        false
    }
    fn new_self(self:Box<Self>) -> Box<dyn State>{
        Box::new(Default {})
    }
}

struct Idle{}
struct Walking{}
struct Running{}
struct Attacking{}
struct Dieing{}
struct Collecting{}
struct Default{}

impl State for Default{
    fn base_case(self: Box<Self>) -> bool{
        true
    }
}

impl State for Idle{
    fn start_move(self: Box<Self>) -> (Box<dyn State>, Box <dyn State>){
        (Box::new(Walking {}), Box::new(Running {}))
    }
    fn print <'a> (&self) -> &'a str{
        "Idle anim"
    }
    fn new_self(self:Box<Self>) -> Box<dyn State>{
        Box::new(Idle {})
    }
}

impl State for Walking{
    fn print <'a> (&self) -> &'a str{
        "Walking state"
    }
    fn new_self(self:Box<Self>) -> Box<dyn State>{
        Box::new(Walking {})
    }
}
impl State for Running{
    fn stop_move(self: Box<Self>) -> (Box<dyn State>, Box <dyn State>){
        (Box::new(Walking {}), Box::new(Idle {}))
    }
    fn attack(self: Box<Self>) -> (Box<dyn State>, Box <dyn State>){
        (Box::new(Idle{}), Box::new(Attacking {}))
    }
    fn collect(self: Box<Self>) -> (Box<dyn State>, Box <dyn State>){
        (Box::new(Idle{}), Box::new(Collecting {}))
    }
    fn print <'a> (&self) -> &'a str{
        "Running anim"
    }
    fn new_self(self:Box<Self>) -> Box<dyn State>{
        Box::new(Running {})
    }
}

impl State for Dieing{
    fn print <'a> (&self) -> &'a str{
        "Dying anim"
    }
    fn new_self(self:Box<Self>) -> Box<dyn State>{
        Box::new(Dieing {})
    }
}

impl State for Collecting{
    fn print <'a> (&self) -> &'a str{
        "collecting anim"
    }
    fn new_self(self:Box<Self>) -> Box<dyn State>{
        Box::new(Collecting {})
    }
}
impl State for Attacking{
    fn print <'a> (&self) -> &'a str{
        "Attacking anim"
    }
    fn new_self(self:Box<Self>) -> Box<dyn State>{
        Box::new(Attacking {})
    }
}

fn main() {
    let mut player = Player::new();
    println!("{}",player.print());
    player.set_velocity(4,4);
    println!("{}",player.print());
    player.update();
    println!("{}",player.print());
    player.attack();
    println!("{}",player.print());
    player.collect();
    println!("{}",player.print());
    
}
