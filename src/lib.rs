use std::cmp;
use std::collections::HashMap;

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone)]
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
    point: Point,
    direction: Direction,
}

// -- Plateau

struct Plateau {
    sw: Point,
    ne: Point,
    rovers: HashMap<String, Rover>,
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
            rovers: HashMap::new(),
        }
    }

    fn add_rover(&mut self, name: String, point: Point, direction: Direction) -> Result<(), &str> {
        if self.rover_exists(&name) {
            return Err("Rover already exists");
        }

        if !self.contains(&point) {
            return Err("Point outside of plateau");
        }

        if self.has_rover_at(&point) {
            return Err("Point is taken by another rover");
        }

        let rover = Rover::new(point, direction);
        self.rovers.insert(name, rover);
        return Ok(());
    }

    fn move_rover(&mut self, name: &str) -> Result<(), &str> {
        if !self.rover_exists(&name) {
            return Err("Invalid rover name");
        }
        let rover = self.rovers.get(name).unwrap();

        let next_point = match rover.direction {
            Direction::North => Point::new(rover.point.x, rover.point.y + 1),
            Direction::East => Point::new(rover.point.x + 1, rover.point.y),
            Direction::South => Point::new(rover.point.x, rover.point.y - 1),
            Direction::West => Point::new(rover.point.x -1, rover.point.y),
        };
        if self.contains(&next_point) && !self.has_rover_at(&next_point) {
            // NOTE: Rust complains if rover is borrowed as mutable above and then call `self.contains` and change rover's point.
            // Therefore it is borrowed again here.
            // Is there a better workaround? (Replacing `self.contains` with its content also solves the problem.)
            let mut rover = self.rovers.get_mut(name).unwrap();
            rover.point.x = next_point.x;
            rover.point.y = next_point.y;
        }

        Ok(())
    }

    fn rover_position(&self, name: &str) -> Result<Point, &str> {
        if !self.rover_exists(&name) {
            return Err("Invalid rover name");
        }
        Ok(self.rovers.get(name).unwrap().point)
    }

    fn rover_exists(&self, name: &str) -> bool {
        self.rovers.contains_key(name)
    }

    fn has_rover_at(&self, point: &Point) -> bool {
        for rover in self.rovers.values() {
            if &rover.point == point {
                return true
            }
        }
        false
    }

    fn contains(&self, point: &Point) -> bool {
        point.x <= self.ne.x && point.x >= self.sw.x && point.y <= self.ne.y && point.y >= self.sw.y
    }

}

impl Rover {
    fn new(point: Point, direction: Direction) -> Self {
        Self {
            point: point,
            direction: direction,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{Point, Direction, Rover, Plateau};

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
    fn add_rover_success() {
        let mut plateau = Plateau::new(Point::new(0, 0), Point::new(100, 100));
        plateau.add_rover(String::from("My Rover"), Point::new(0, 0), Direction::East).unwrap();
        assert_eq!(plateau.rover_exists("My Rover"), true);
    }

    #[test]
    fn add_rover_outside_of_plateau_error() {
        let mut plateau = Plateau::new(Point::new(1, 1), Point::new(10, 10));
        let result = plateau.add_rover(String::from("R1"), Point::new(20, 20), Direction::East);
        assert!(result.is_err());
    }

    #[test]
    fn add_rover_multiple_times_error() {
        let mut plateau = Plateau::new(Point::new(1, 1), Point::new(10, 10));
        plateau.add_rover(String::from("R1"), Point::new(5, 5), Direction::East).unwrap();
        let result = plateau.add_rover(String::from("R1"), Point::new(6, 5), Direction::East);
        assert!(result.is_err());
    }

    #[test]
    fn add_rover_collision_error() {
        let mut plateau = Plateau::new(Point::new(1, 1), Point::new(10, 10));
        plateau.add_rover(String::from("R1"), Point::new(5, 5), Direction::East).unwrap();
        let result = plateau.add_rover(String::from("R2"), Point::new(5, 5), Direction::East);
        assert!(result.is_err());
    }

    #[test]
    fn plateau_contains_point() {
        let plateau = Plateau::new(Point::new(1, 1), Point::new(100, 100));
        assert!(plateau.contains(&Point::new(1, 1)));
        assert!(plateau.contains(&Point::new(10, 10)));
        assert!(plateau.contains(&Point::new(100, 100)));

        assert!(!plateau.contains(&Point::new(0, 1)));
        assert!(!plateau.contains(&Point::new(1, 0)));
        assert!(!plateau.contains(&Point::new(101, 1)));
        assert!(!plateau.contains(&Point::new(1, 101)));
        assert!(!plateau.contains(&Point::new(-1, 1001)));
    }

    #[test]
    fn move_single_rover() {
        let mut plateau = Plateau::new(Point::new(0, 0), Point::new(10, 10));
        plateau.add_rover(
            String::from("R1"),
            Point::new(9,9),
            Direction::East,
        ).unwrap();
        plateau.move_rover("R1").unwrap();
        assert_eq!(plateau.rover_position("R1").unwrap(), Point::new(10, 9));

        plateau.move_rover("R1").unwrap();
        // end of plateau, should no longer move
        assert_eq!(plateau.rover_position("R1").unwrap(), Point::new(10, 9));
    }

    #[test]
    fn move_multiple_rovers(){
        let mut plateau = Plateau::new(Point::new(0, 0), Point::new(100, 100));
        plateau.add_rover(String::from("R1"), Point::new(0, 1), Direction::East).unwrap();
        plateau.add_rover(String::from("R2"), Point::new(1, 1), Direction::North).unwrap();

        // r1 cannot move east: r2 is blocking it
        plateau.move_rover("R1");
        assert_eq!(plateau.rover_position("R1").unwrap(), Point::new(0, 1));

        // Move r2
        plateau.move_rover("R2");

        // Now r1 can move
        plateau.move_rover("R1");
        assert_eq!(plateau.rover_position("R1").unwrap(), Point::new(1, 1));
    }
}
