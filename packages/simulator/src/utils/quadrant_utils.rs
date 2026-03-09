use crate::quadrant::Quadrant;
use crate::rectangle::Rectangle;
use crate::vector2::Vector2;

pub fn quadrant_to_rectangle(rectangle: &Rectangle, quadrant: &Quadrant) -> Rectangle {
    let half_width = rectangle.width / 2.0;
    let half_height = rectangle.height / 2.0;
    let center_x = rectangle.position.x + half_width;
    let center_y = rectangle.position.y + half_height;

    match quadrant {
        Quadrant::NW => Rectangle {
            position: rectangle.position,
            width: half_width,
            height: half_height,
        },
        Quadrant::NE => Rectangle {
            position: Vector2 {
                x: center_x,
                y: rectangle.position.y,
            },
            width: half_width,
            height: half_height,
        },
        Quadrant::SW => Rectangle {
            position: Vector2 {
                x: rectangle.position.x,
                y: center_y,
            },
            width: half_width,
            height: half_height,
        },
        Quadrant::SE => Rectangle {
            position: Vector2 {
                x: center_x,
                y: center_y,
            },
            width: half_width,
            height: half_height,
        },
    }
}

pub fn find_quadrant(boundary: &Rectangle, position: &Vector2) -> Option<Quadrant> {
    let center_x = boundary.position.x + boundary.width / 2.0;
    let center_y = boundary.position.y + boundary.height / 2.0;

    // Check if the position is within the boundary
    if position.x >= boundary.position.x
        && position.x <= (boundary.position.x + boundary.width)
        && position.y >= boundary.position.y
        && position.y <= (boundary.position.y + boundary.height)
    {
        if position.x < center_x {
            if position.y < center_y {
                Some(Quadrant::NW)
            } else {
                Some(Quadrant::SW)
            }
        } else {
            if position.y < center_y {
                Some(Quadrant::NE)
            } else {
                Some(Quadrant::SE)
            }
        }
    } else {
        // The position is outside the boundary
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadrant_to_rectangle_all() {
        let rect = Rectangle {
            position: Vector2 { x: 0.0, y: 0.0 },
            width: 100.0,
            height: 100.0,
        };
        
        let nw = quadrant_to_rectangle(&rect, &Quadrant::NW);
        assert_eq!(nw.position.x, 0.0);
        assert_eq!(nw.position.y, 0.0);
        
        let ne = quadrant_to_rectangle(&rect, &Quadrant::NE);
        assert_eq!(ne.position.x, 50.0);
        assert_eq!(ne.position.y, 0.0);

        let sw = quadrant_to_rectangle(&rect, &Quadrant::SW);
        assert_eq!(sw.position.x, 0.0);
        assert_eq!(sw.position.y, 50.0);

        let se = quadrant_to_rectangle(&rect, &Quadrant::SE);
        assert_eq!(se.position.x, 50.0);
        assert_eq!(se.position.y, 50.0);
    }

    #[test]
    fn test_find_quadrant_inside() {
        let boundary = Rectangle {
            position: Vector2 { x: 0.0, y: 0.0 },
            width: 100.0,
            height: 100.0,
        };
        
        assert_eq!(find_quadrant(&boundary, &Vector2 { x: 25.0, y: 25.0 }), Some(Quadrant::NW));
        assert_eq!(find_quadrant(&boundary, &Vector2 { x: 75.0, y: 25.0 }), Some(Quadrant::NE));
        assert_eq!(find_quadrant(&boundary, &Vector2 { x: 25.0, y: 75.0 }), Some(Quadrant::SW));
        assert_eq!(find_quadrant(&boundary, &Vector2 { x: 75.0, y: 75.0 }), Some(Quadrant::SE));
    }

    #[test]
    fn test_find_quadrant_outside() {
        let boundary = Rectangle {
            position: Vector2 { x: 0.0, y: 0.0 },
            width: 100.0,
            height: 100.0,
        };
        let outside_position = Vector2 { x: -10.0, y: 110.0 };
        assert_eq!(find_quadrant(&boundary, &outside_position), None);
    }
}
