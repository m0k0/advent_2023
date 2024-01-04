use std::fmt;

pub struct Coord2D {
    x: isize,
    y: isize
}
impl Coord2D {
    pub fn new(x: isize, y: isize) -> Self {
        return Self { x, y }
    }

    pub fn combine(&self, other: &Coord2D) -> Coord2D {
        return Coord2D::new(self.x + other.x, self.y + other.y);
    }
}
impl Clone for Coord2D {
    
    fn clone(&self) -> Self {
        return Coord2D::new(self.x, self.y);
    }

}
impl std::fmt::Display for Coord2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
impl std::ops::Add<Coord2D> for Coord2D{
    type Output = Self;

    fn add(self, rhs: Coord2D) -> Self::Output {
        return Coord2D::new(self.x + rhs.x, self.y + rhs.y);
    } 
}
impl PartialEq for Coord2D {

    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }

}

pub struct Matrix<T> {
    data: Vec<Vec<Option<T>>>,
    width: usize,
    height: usize
}

impl<T> Matrix<T> 
    where T : fmt::Display{

    pub fn get_value(&self, x: usize, y: usize) -> Option<&T> {
        let row = match self.data.get(y) {
            Some(v) => v,
            None => return None
        };

        let cell = match row.get(x) {
            Some(v) => v,
            None => return None
        };


        let value = match cell {
            Some(v) => v,
            None => return None
        };

        return Some(&value);
    }

    pub fn set_value(&mut self, x: usize, y: usize, value: T) {

        while self.data.len() <= y {
            self.data.push(Vec::new());
        }

        let row = match self.data.get_mut(y) {
            Some(v) => v,
            None => panic!("Out of bounds")
        };
         
        while row.len() <= x {
            row.push(None);
        }

        if self.width < row.len() {
            self.width = row.len();
        }
        if self.height < self.data.len() {
            self.height = self.data.len();
        }
        
        self.data[y][x] = Some(value); 
    }

    pub fn new() -> Self {
        return Self {
            data: Vec::new(),
            width: 0,
            height: 0
        }
    }

    pub fn print_grid(&self) where {

        for y in 0..self.height {
            for x in 0..self.width {

                if let Some(v) = self.get_value(x, y) {
                    print!("{}|",v);
                } else {
                    print!(" |");
                }
            }

            print!("\n");
        }
    }
}
