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

// -- Plateau

struct Plateau {
    sw: Point,
    ne: Point,
}

impl Plateau {
    fn new(sw: Point, ne: Point) -> Self {
        Self {
            sw: sw,
            ne: ne,
        }
    }

    fn is_valid(&self, point: Point) -> bool {
        point.x <= self.ne.x && point.x >= self.sw.x && point.y <= self.ne.y && point.y >= self.sw.y
    }

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
    use super::{Point, Direction, Rover, Plateau};

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

    #[test]
    fn init_plateau() {
        let plateau = Plateau::new(Point::new(0, 0), Point::new(100, 100));
        assert_eq!(plateau.sw, Point::new(0, 0));
        assert_eq!(plateau.ne, Point::new(100, 100));
    }

    #[test]
    fn is_point_on_plateau() {
        let plateau = Plateau::new(Point::new(1, 1), Point::new(100, 100));
        assert!(plateau.is_valid(Point::new(1, 1)));

        assert!(!plateau.is_valid(Point::new(0, 1)));
        assert!(!plateau.is_valid(Point::new(1, 0)));
        assert!(!plateau.is_valid(Point::new(101, 1)));
        assert!(!plateau.is_valid(Point::new(1, 101)));
    }


}
