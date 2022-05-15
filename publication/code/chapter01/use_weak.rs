// use_weak.rs

use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Car {
    name: String,
    wheels: RefCell<Vec<Weak<Wheel>>>, // 引用 Wheel
}

struct Wheel {
    id: i32,
    car: Rc<Car>, // 引用 Car
}

fn main() {
    let car: Rc<Car> = Rc::new(
        Car {
            name: "Tesla".to_string(),
            wheels: RefCell::new(vec![]),
        }
    );
    let wl1 = Rc::new(Wheel { id:1, car: Rc::clone(&car) });
    let wl2 = Rc::new(Wheel { id:2, car: Rc::clone(&car) });

    let mut wheels = car.wheels.borrow_mut();

    // downgrade 得到 Weak
    wheels.push(Rc::downgrade(&wl1));
    wheels.push(Rc::downgrade(&wl2));

    for wheel_weak in car.wheels.borrow().iter() {
        let wl = wheel_weak.upgrade().unwrap(); // Option
        println!("wheel {} owned by {}", wl.id, wl.car.name);
    }
}
