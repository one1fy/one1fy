use uuid::Uuid;
use skia_safe::{ Canvas };
use crate::components::*;

pub enum Orientation {
    HORIZONTAL,
    VERTICAL
}

pub struct BarContainer {
    pub id: Uuid,
    pub onLoad: Option<fn()>,
    pub visible: bool,
    pub height: u32,
    pub width: u32,
    pub left: u32,
    pub top: u32,
    pub children: Vec<Box<dyn ComponentTraits>>,
    pub orientation: Orientation,
    pub remaining_x: u32,
    pub remaining_y: u32,
    pub componentType: Type,
    pub parent: Box<dyn ComponentTraits>,
}

impl BarContainer {
    pub fn new(
        onLoad: Option<fn()>,
        visible: bool,
        height: u32,
        width: u32,
        left: u32,
        top: u32,
        children: Option<Vec<Box<dyn ComponentTraits>>>,
        orientation: Orientation,
        parent: Box<dyn ComponentTraits>,
    ) -> BarContainer {
        let id = Uuid::new_v4();
        let rem_x = width;
        let rem_y = height;

        let x: Vec<Box<dyn ComponentTraits>>;

        if let None = children {
            x = vec!();
        } else {
            x = children.unwrap();
        }

        BarContainer {
            id: Uuid::new_v4(),
            onLoad: onLoad,
            visible: visible,
            height: height,
            width: width,
            left: left,
            top: top,
            children: x,
            orientation: orientation,
            remaining_x: width,
            remaining_y: height,
            componentType: Type::CONTAINER,
            parent,
        }
        
    }

    pub fn calculate_coordinate(container_width: u32, num_children: u32, current_child: u32, child_width: u32) -> u32 {
        let current_slice: f32 = container_width as f32 * ((current_child as f32 + 1.0) / num_children as f32);
        let previous_slice: f32 = container_width as f32 * (current_child as f32 / num_children as f32);
        let left_to_change: f32 = ((current_slice as f32 + previous_slice as f32) / 2.0) - child_width as f32 / 2.0;
        if left_to_change < 0.0 {
            return 0 as u32
        }
        left_to_change as u32
    }

    pub fn add_to_children(&mut self, child: Box<dyn ComponentTraits>) {
        match &self.orientation {
            HORIZONTAL => {
                if self.remaining_x - child.get_width() >= 0 {
                    self.remaining_x = self.remaining_x - child.get_width();
                    self.children.push(child);
                    let size: u32 = self.children.len() as u32;
                    for i in 0..self.children.len() {
                        let cur = &mut self.children[i];
                        //TODO: build setters for box
                        cur.set_left((BarContainer::calculate_coordinate(self.width, size, i as u32, cur.get_width())));
                        cur.set_top(self.height / 2 - cur.get_height() / 2);
                    }
                }
                else {
                    println!("Insufficient horizontal space in container.");
                }
            },
            VERTICAL => {
                if (self.remaining_y - child.get_height() >= 0) {
                    self.remaining_y = self.remaining_y - child.get_height();
                    self.children.push(child);
                    let size = self.children.len() as u32;
                    for i in 0..self.children.len() {
                        let cur = &mut self.children[i];
                        //TODO: build setters for box
                        cur.set_top((BarContainer::calculate_coordinate(self.height, size, i as u32, cur.get_height())));
                        cur.set_left((self.width / 2 - cur.get_width() / 2));
                    }
                }
                else {
                    println!("Insufficient vertical space in container.");
                }
            }
        }
    }

    pub fn remove_from_children(&mut self, index: u32) {
        let toRemove = &mut self.children[index as usize];
        
        match &self.orientation {
            HORIZONTAL => {
                self.remaining_x = self.remaining_x + toRemove.get_width();
                self.children.remove(index as usize);
                let size = self.children.len() as u32;
                for i in 0..self.children.len() {
                    let cur = &mut self.children[i];
                    //TODO: build setters for box
                    cur.set_left((BarContainer::calculate_coordinate(self.width, size, i as u32, cur.get_width())));
                    cur.set_top(self.height / 2 - cur.get_height() / 2);
                }
            },
            VERTICAL => {
                self.remaining_y = self.remaining_y + toRemove.get_height();
                self.children.remove(index as usize);
                let size = self.children.len() as u32;
                for i in 0..self.children.len() {
                    let cur = &mut self.children[i];
                    //TODO: build setters for box
                    cur.set_top((BarContainer::calculate_coordinate(self.height, size, i as u32, cur.get_height())));
                    cur.set_left((self.width / 2 - cur.get_width() / 2));
                }
            }
        }
    }
}
impl Draw for BarContainer{
    fn draw(&mut self, canvas: &mut Canvas) {
        let imm = &*self;
        if self.visible {
            for child in self.children.iter_mut() {
                child.draw(canvas);
            }
        } 
    }
}

impl Find for BarContainer {
    fn find(&mut self, x: u32, y: u32) -> Option<Uuid> {
        for i in 0..self.children.len() {
            let cur = &mut self.children[i];
            let val: Option<Uuid> = cur.find(x, y);
            if let None = val {
                continue;
            }
            else {
                return val;
            }
        }
        None
    }
}

impl Remove for BarContainer {
    fn remove(&mut self, id: Uuid) -> bool {
        if self.id == id {
            return true;
        }
        else {
            for i in 0..self.children.len() {
                let cur = &mut self.children[i];
                let val = cur.remove(id);
                if val {
                    self.remove_from_children(i as u32);
                    return false;
                }
    
            }
            return false;
        }
        
    }
}

impl ToggleVisible for BarContainer {
    fn toggle_visible(&mut self) {
        self.visible = !self.visible;
    }
}

impl GetHeight for BarContainer {
    fn get_height(&self) -> u32 {
        self.height
    }
}

impl GetWidth for BarContainer {
    fn get_width(&self) -> u32 {
        self.width
    }
}

impl SetLeft for BarContainer {
    fn set_left(&mut self, val: u32) {
        self.left = val;
    }
}

impl SetTop for BarContainer {
    fn set_top(&mut self, val: u32) {
        self.top = val;
    }
}

impl GetType for BarContainer {
    fn get_type(&self) -> Option<Type> {
        Some(Type::CONTAINER)
    }
}


