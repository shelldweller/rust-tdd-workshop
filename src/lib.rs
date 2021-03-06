use std::cmp;

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
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
    direction: Direction,
    plateau: Plateau,
}

// -- Plateau

struct Plateau {
    sw: Point,
    ne: Point,
}

impl Plateau {
    fn new(point1: Point, point2: Point) -> Self {
        let sw = Point::new(
            cmp::min(point1.x, point2.x),
            cmp::min(point1.y, point2.y),
        );
        let ne = Point::new(
            cmp::max(point1.x, point2.x),
            cmp::max(point1.y, point2.y),
        );
        Self {
            sw: sw,
            ne: ne,
        }
    }

    fn is_valid(&self, point: &Point) -> bool {
        point.x <= self.ne.x && point.x >= self.sw.x && point.y <= self.ne.y && point.y >= self.sw.y
    }

}

impl Rover {
    fn new(name: String, point: Point, direction: Direction, plateau: Plateau) -> Result<Self, String> {
        if plateau.is_valid(&point) {
            Ok(Self {
                name: name,
                point: point,
                direction: direction,
                plateau: plateau,
            })
        } else {
            Err(String::from(format!("Point {:?} is not on plateau", point)))
        }
    }

    fn step(&mut self) {
        let next_point = match self.direction {
            Direction::North => Point::new(self.point.x, self.point.y + 1),
            Direction::East => Point::new(self.point.x + 1, self.point.y),
            Direction::South => Point::new(self.point.x, self.point.y - 1),
            Direction::West => Point::new(self.point.x -1, self.point.y),
        };
        if self.plateau.is_valid(&next_point) {
            self.point.x = next_point.x;
            self.point.y = next_point.y;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{Point, Direction, Rover, Plateau};

    #[test]
    fn init_rover_success() {
        let plateau = Plateau::new(Point::new(0, 0), Point::new(10, 10));
        let rover = Rover::new(String::from("some name"), Point::new(3,4), Direction::East, plateau).unwrap();
        match rover.direction {
            Direction::East => assert!(true),
            _ => assert!(false),
        }
        assert_eq!(rover.name, String::from("some name"));
        assert_eq!(rover.point, Point::new(3, 4));
    }

    #[test]
    fn init_rover_error() {
        let plateau = Plateau::new(Point::new(1, 1), Point::new(10, 10));
        let result = Rover::new(String::from("some name"), Point::new(0,0), Direction::East, plateau);
        assert!(result.is_err())
    }

    #[test]
    fn step_rover() {
        let plateau = Plateau::new(Point::new(0, 0), Point::new(10, 10));
        let mut rover = Rover::new(
            String::from("some name"),
            Point::new(9,9),
            Direction::East,
            plateau
        ).unwrap();
        rover.step();
        assert_eq!(rover.point, Point::new(10, 9));

        rover.step();
        // end of plateau, should no longer move
        assert_eq!(rover.point, Point::new(10, 9));
    }

    #[test]
    fn init_plateau_basic() {
        let plateau = Plateau::new(Point::new(0, 0), Point::new(100, 100));
        assert_eq!(plateau.sw, Point::new(0, 0));
        assert_eq!(plateau.ne, Point::new(100, 100));
    }

    #[test]
    fn init_plateau_flexible() {
        let plateau = Plateau::new(Point::new(0, 0), Point::new(-100, 100));
        assert_eq!(plateau.sw, Point::new(-100, 0));
        assert_eq!(plateau.ne, Point::new(0, 100));
    }

    #[test]
    fn is_point_on_plateau() {
        let plateau = Plateau::new(Point::new(1, 1), Point::new(100, 100));
        assert!(plateau.is_valid(&Point::new(1, 1)));

        assert!(!plateau.is_valid(&Point::new(0, 1)));
        assert!(!plateau.is_valid(&Point::new(1, 0)));
        assert!(!plateau.is_valid(&Point::new(101, 1)));
        assert!(!plateau.is_valid(&Point::new(1, 101)));
    }
}
