// move2tower.rs

// p : pole 杆
fn move2tower(height: u32, src_p: &str, des_p: &str, mid_p: &str) {
    if height >= 1 {
        move2tower(height - 1, src_p, mid_p, des_p);
        println!("moving disk[{height}] from {src_p} to {des_p}");
        move2tower(height - 1, mid_p, des_p, src_p);
    }
}

fn main() {
    move2tower(1, "A", "B", "C");
    move2tower(2, "A", "B", "C");
    move2tower(3, "A", "B", "C");
    move2tower(4, "A", "B", "C");
}
