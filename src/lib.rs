use std::cmp;
use std::collections::HashMap;

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

        let rover = Rover::new(point, direction);
        self.rovers.insert(name, rover);
        return Ok(());
    }

    fn rover_exists(&self, name: &str) -> bool {
        self.rovers.contains_key(name)
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

    // fn step(&mut self) {
    //     let next_point = match self.direction {
    //         Direction::North => Point::new(self.point.x, self.point.y + 1),
    //         Direction::East => Point::new(self.point.x + 1, self.point.y),
    //         Direction::South => Point::new(self.point.x, self.point.y - 1),
    //         Direction::West => Point::new(self.point.x -1, self.point.y),
    //     };
    //     if self.plateau.is_valid(&next_point) {
    //         self.point.x = next_point.x;
    //         self.point.y = next_point.y;
    //     }
    // }
}


#[cfg(test)]
mod tests {
    use super::{Point, Direction, Rover, Plateau};

    #[test]
    // fn init_rover_success() {
    //     let plateau = Plateau::new(Point::new(0, 0), Point::new(10, 10));
    //     let rover = Rover::new(String::from("some name"), Point::new(3,4), Direction::East, plateau).unwrap();
    //     match rover.direction {
    //         Direction::East => assert!(true),
    //         _ => assert!(false),
    //     }
    //     assert_eq!(rover.name, String::from("some name"));
    //     assert_eq!(rover.point, Point::new(3, 4));
    // }

    // #[test]
    // fn step_rover() {
    //     let plateau = Plateau::new(Point::new(0, 0), Point::new(10, 10));
    //     let mut rover = Rover::new(
    //         String::from("some name"),
    //         Point::new(9,9),
    //         Direction::East,
    //         plateau
    //     ).unwrap();
    //     rover.step();
    //     assert_eq!(rover.point, Point::new(10, 9));

    //     rover.step();
    //     // end of plateau, should no longer move
    //     assert_eq!(rover.point, Point::new(10, 9));
    // }

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
        assert!(result.is_err())
    }


    // #[test]
    // fn is_point_on_plateau() {
    //     let plateau = Plateau::new(Point::new(1, 1), Point::new(100, 100));
    //     assert!(plateau.is_valid(&Point::new(1, 1)));

    //     assert!(!plateau.is_valid(&Point::new(0, 1)));
    //     assert!(!plateau.is_valid(&Point::new(1, 0)));
    //     assert!(!plateau.is_valid(&Point::new(101, 1)));
    //     assert!(!plateau.is_valid(&Point::new(1, 101)));
    // }

    // #[test]
    // fn move_collision(){
    //     let plateau = Plateau::new(Point::new(0, 0), Point::new(100, 100));
    //     let mut rover1 = Rover::new(String::from("r1"), Point::new(0, 1), Direction::East, plateau).unwrap();
    //     let mut rover2 = Rover::new(String::from("r2"), Point::new(1, 1), Direction::North, plateau).unwrap();

    //     // r1 cannot move east: r2 is blocking it
    //     rover1.step();
    //     assert_eq!(rover1.point, Point::new(0, 1));

    //     // Move r2
    //     rover2.step();

    //     // Now r1 can move
    //     rover1.step();
    //     assert_eq!(rover1.point, Point::new(1, 1));
    // }

    // #[test]
    // fn refactoring(){
    //     let plateau = Plateau::new(Point::new(0, 0), Point::new(100, 100));

    //     plateau.add_rover("r1", Point::new(0, 1), Direction::East).unwarp();
    //     plateau.add_rover("r2", Point::new(1, 1), Direction::North).unwrap();

    //     // r1 cannot move east: r2 is blocking it
    //     plateau.move_rover("r1").unwrap()
    //     assert_eq!(plateau.rover("r1").unwrap().point, Point::new(0, 1));
    //     // assert_eq!(plateau.rover_position("r1"), Point::new(0, 1));

    //     // // Move r2
    //     // rover2.step();

    //     // // Now r1 can move
    //     // rover1.step();
    //     // assert_eq!(rover1.point, Point::new(1, 1));
    // }

}
