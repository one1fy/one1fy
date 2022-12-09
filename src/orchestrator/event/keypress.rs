use crate::framework::components::BarContainer;
use crate::framework::components::*;
use glutin::event::KeyboardInput;
use std::collections::HashMap;


pub fn handle_press(press: KeyboardInput, tree: &mut BarContainer) {
    let converter = HashMap::from(
        [
            (0, '1'),
            (1, '2'),
            (2, '3'),
            (3, '4'),
            (4, '5'),
            (5, '6'),
            (6, '7'),
            (7, '8'),
            (8, '9'),
            (9, '0'),
            (26, 'q'),
            (32, 'w'),
            (14,'e'),
            (27,'r'),
            (29,'t'),
            (34,'y'),
            (30,'u'),
            (18,'i'),
            (24,'o'),
            (25,'p'),
            (10,'a'),
            (28,'s'),
            (13,'d'),
            (15,'f'),
            (16,'g'),
            (17,'h'),
            (19,'j'),
            (20,'k'),
            (21,'l'),
            (35,'z'),
            (33,'x'),
            (12,'c'),
            (31,'v'),
            (11,'b'),
            (23,'n'),
            (22,'m'),
            (74, '~'),
            (75, '$'),
            (76, ' '),
        ]
    );
    //println!("{}", press.virtual_keycode.unwrap() as u8);
    let c = converter.get(&(press.virtual_keycode.unwrap() as u32));
    if c.is_none() {
        println!("Unrecognized Key!");
    }
    else {
        tree.on_press(*c.unwrap());
    }
    //tree.on_press(*converter.get(&(press.virtual_keycode.unwrap() as u32)).unwrap());
}