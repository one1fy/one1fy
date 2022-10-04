use uuid:Uuid;
use skia_safe::{ Canvas };

enum Orientation {
    HORIZONTAL,
    VERTICAL
};

pub struct BarContainer {
    pub id: u32,
    pub onLoad: fn(),
    pub visible: bool,
    pub height: u32,
    pub width: u32,
    pub left: u32,
    pub top: u32,
    pub children: Vec<Box<dyn draw>>,
    pub orientation: Orientation,
    pub remaining_x: u32,
    pub remaining_y: u32,
};

impl BarContainer {
    pub fn new(
        onLoad: fn(),
        visible: bool,
        height, u32,
        width: u32,
        left: u32,
        top: u32,
        children: Vec<Box<dyn draw>>,
        orientation: Orientation,
    ) -> BarContainer {
        BarContainer {
            Uuid::new_v4(),
            onLoad,
            visible,
            height,
            width,
            left,
            top,
            children,
            orientation,
            width,
            height,
        }
        
    }

    pub fn add_to_children(&self, child: Box<dyn draw>) {
        match self.orientation {
            HORIZONTAL => {
                if (self.remaining_x + child.width <= self.width) {
                    self.remaining_x = self.remaining_x - child.width;
                    self.children.push(child);
                    for i in 0..self.children.len() {
                        let cur: Box<dyn draw> = self.children[i];
                        //TODO: build setters for box
                        cur.set_left((calculate_coordinate(self.width, self.children.len(), i, cur.width)));
                        cur.set_top((self.height / 2 - cur.height / 2));
                    }
                }
                else {
                    println!("Insufficient horizontal space in container.");
                }
            },
            VERTICAL => {
                if (self.remaining_y + child.height <= self.height) {
                    self.remaining_y = self.remaining_y - child.height;
                    self.children.push(child);
                    for i in 0..self.children.len() {
                        let cur: Box<dyn draw> = self.children[i];
                        //TODO: build setters for box
                        cur.set_top((calculate_coordinate(self.height, self.children.len(), i, cur.height)));
                        cur.set_left((self.width / 2 - cur.width / 2));
                    }
                }
                else {
                    println!("Insufficient vertical space in container.");
                }
            }
        }
        
    }

    fn calculate_coordinate(container_width: u32, num_children: u32, current_child: u32, child_width: u32) -> u32 {
        let current_slice: u32 = container_width * ((current_child + 1) / num_children);
        let previous_slice: u32 = container_width * (current_child / num_children);
        let left_to_change: u32 = ((current_slice + previous_slice) / 2) - child_width / 2;
        left_to_change
    }

    
}
impl Draw for BarContainer{
    fn draw(&self, canvas: &mut Canvas) {
        for child in self.children.iter() {
            child.draw(&canvas);
        }
    }
}