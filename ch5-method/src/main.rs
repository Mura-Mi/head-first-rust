struct Rectangle {
  width: u32,
  height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn twice(&mut self) -> () {
        self.width = self.width * 2;
        self.height = self.height * 2;
    }

    fn square(i: u32) -> Rectangle {
        Rectangle { width: i, height: i }
    }
}

fn main() {
    let mut rect1 = Rectangle { width: 640, height: 480 };
    rect1.twice();
    println!("Rectangle's area: {}", rect1.area());

    let square = Rectangle::square(15);
    println!("Rectangle's area: {}", square.area());
}
