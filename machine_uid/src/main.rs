use machine_uid;

fn main() {
    let id: String = machine_uid::get().unwrap();
    println!("{}", id);
}
