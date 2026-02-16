fn borrow<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    i
}

// impl CatFeeder<'a> {
//     fn feed(&mut self) {
//         self.cat.feed();
//     }
// }

impl Cat {
    fn feed(&mut self) {
        self.0 = format!("{} (purring", self.0)
    }
}

fn main() {
    let n = 12;
    borrow(&n, &n);
    let cats = vec![
        Cat("Fordo".to_string()),
        Cat("Bilbo".to_string())
    ];

    let mut feeders = Vec::new();
    for cat in cats.iter() {
        feeders.push(CatFeeder{cat})
    }

}

struct Cat(String);

struct CatFeeder<'a> {
    cat: &'a Cat,
}
