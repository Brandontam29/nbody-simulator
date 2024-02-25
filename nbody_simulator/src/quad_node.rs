use crate::{particle::Particle, vector2::Vector2};

pub struct QuadNode {
    center_of_mass: Vector2,
    total_mass: f64,
    boundary: Rectangle,
    children: [Option<Box<QuadNode>>; 4],
    value: Option<Particle>,
}

impl QuadNode {
    pub fn new(boundary: Rectangle) -> Self {
        QuadNode {
            center_of_mass: Vector2 { x: 0.0, y: 0.0 },
            total_mass: 0.0,
            boundary,
            children: [None, None, None, None],
            value: None,
        }
    }

    // pub fn insert(&mut self, particle: Particle) {
    //     let has_children = &self.has_children();

    //     if *has_children == true {
    //         let (region, boundary) = match get_region(&self.boundary, &particle.position) {
    //             Some(v) => v,
    //             None => return,
    //         };

    //         let mut option_child_q = &self.children[usize::from(region)];

    //         match &mut option_child_q {
    //             Some(child_q) => {
    //                 child_q.insert(particle);
    //             }

    //             None => {
    //                 let mut q = QuadNode::new(boundary);

    //                 q.value = Some(particle);
    //             }
    //         }

    //         return;
    //     }

    //     match &self.value {
    //         Some(_) => {
    //             self.subdivide();
    //             // let scaled_center = self.center_of_mass.scale(self.total_mass)
    //             //     + particle.position.scale(particle.mass);

    //             // self.total_mass += particle.mass;

    //             // self.center_of_mass = scaled_center.scale(1.0 / self.total_mass);

    //             self.insert(particle);
    //         }
    //         None => {
    //             self.value = Some(particle);
    //         }
    //     };
    // }

    pub fn insert(&mut self, particle: Particle) {
        if self.has_children() {
            let (region, boundary) = match get_region(&self.boundary, &particle.position) {
                Some(v) => v,
                None => return,
            };

            if let Some(child) = &mut self.children[usize::from(&region)] {
                child.insert(particle); // This calls insert on the existing child.
            } else {
                let mut child = Box::new(QuadNode::new(boundary));
                child.insert(particle); // Insert the particle into the new child.
                self.children[usize::from(&region)] = Some(child); // Update the array with the new child.
            }
        } else {
            if self.value.is_some() {
                // Handle subdivision and re-insertion here.
                // This will require updating the `subdivide` method to be mutable and properly handling child nodes.
            } else {
                self.value = Some(particle);
                // Update center of mass and total mass here.
            }
        }
    }

    fn subdivide(&self) {
        let particle = match self.value {
            Some(v) => v,
            None => return,
        };

        let (region, boundary) = match get_region(&self.boundary, &particle.position) {
            Some(v) => v,
            None => return,
        };

        let mut option_child_q = &self.children[usize::from(&region)];

        match &mut option_child_q {
            Some(ref mut child_q) => {
                child_q.insert(particle);
            }

            None => {
                let mut q = QuadNode::new(boundary);

                q.value = Some(particle);
            }
        }
    }

    fn has_children(&self) -> bool {
        let has_non_none = self.children.iter().any(|x| x.is_some());

        return has_non_none;
    }
}

enum Region {
    Zero,
    One,
    Two,
    Three,
}

impl From<&Region> for usize {
    fn from(index: &Region) -> Self {
        match index {
            Region::Zero => 0,
            Region::One => 1,
            Region::Two => 2,
            Region::Three => 3,
        }
    }
}

pub struct Rectangle {
    x: f64, // Center x
    y: f64, // Center y
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }
}

fn get_region(boundary: &Rectangle, position: &Vector2) -> Option<(Region, Rectangle)> {
    let half_w = boundary.width / 2.0;
    let half_h = boundary.height / 2.0;
    let x = boundary.x + half_w;
    let y = boundary.y + half_h;

    if position.x < x && position.y < y {
        return Some((
            Region::Zero,
            Rectangle {
                x: boundary.x,
                y: boundary.y,
                width: half_w,
                height: half_h,
            },
        ));
    }
    if position.x >= x && position.y < y {
        return Some((
            Region::One,
            Rectangle {
                x: x,
                y: boundary.y,
                width: half_w,
                height: half_h,
            },
        ));
    }
    if position.x < x && position.y >= y {
        return Some((
            Region::Two,
            Rectangle {
                x: boundary.x,
                y: y,
                width: half_w,
                height: half_h,
            },
        ));
    }
    if position.x >= x && position.y >= y {
        return Some((
            Region::Three,
            Rectangle {
                x: x,
                y: y,
                width: half_w,
                height: half_h,
            },
        ));
    }

    return None;
}

/*
---------
| 0 | 1 |
---------
| 2 | 3 |
---------
*/
