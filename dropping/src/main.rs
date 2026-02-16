struct MyStruct {
    n: i32,
}

impl MyStruct{
    fn new(n: i32) -> Self {
        println!("construcint {n}");
        Self {n}
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping {}", self.n)
    }
}

struct HasDroppables {
    x: MyStruct,
}

fn move_me(x: MyStruct) {

}

fn main() {
    let x = MyStruct::new(1);
    move_me(x);
    println!("ending main");
    let has_drop = HasDroppables {x: MyStruct::new(4)};

    let my_num = Box::new(12);
}
