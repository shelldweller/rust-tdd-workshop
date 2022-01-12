enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x,
            y: y
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Rover {
    name: String,
    point: Point,
    direction: Direction
}

impl Rover {
    fn new(name: String, point: Point, direction: Direction) -> Self {
        Self {
            name: name,
            point: point,
            direction: direction
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{Point, Direction, Rover};

    #[test]
    fn init_rover() {
        let rover = Rover::new(String::from("some name"), Point::new(3,4), Direction::East);
        match rover.direction {
            Direction::East => assert!(true),
            _ => assert!(false),
        }
        assert_eq!(rover.name, String::from("some name"));
        assert_eq!(rover.point, Point::new(3, 4));
    }
}
