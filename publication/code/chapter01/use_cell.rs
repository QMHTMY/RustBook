// use_cell.rs

use std::cell::Cell;

struct Fields {
    regular_field: u8,
    special_field: Cell<u8>,
}

fn main() {
    let fields = Fields {
        regular_field: 0,
        special_field: Cell::new(1),
    };

    let value = 10;
    // fields.regular_field = value;  错误：Fields 是不可变的

    fields.special_field.set(value);
    // 尽管 Fields 不可变，
    // 但 special_field 是一个 Cell, 而 Cell 内部值可被修改

    println!("special: {}", fields.special_field.get());
}
