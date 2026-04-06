use std::fmt::Debug;

use std::collections::HashMap;

struct HashMapBucket<K, V> {
    map: HashMap<K, Vec<V>>
}

impl <K,V> HashMapBucket<K,V>
where K: Eq + std::hash::Hash
{
    fn new() -> Self {
        HashMapBucket { map: HashMap::new() }
    }

    fn insert(&mut self, key: K, value: V) {
        let values = self.map.entry(key).or_insert(Vec::new());
        values.push(value);
    }
}

struct HashMapBucketIter<'a, K, V> {
    key_iter: std::collections::hash_map::Iter<'a, K, Vec<V>>,
    current_map_entry: Option<(&'a K, &'a Vec<V>)>,
    current_vec_index: usize,
}

impl <K,V> HashMapBucket<K, V> {
    fn iter(&self) -> HashMapBucketIter<K, V> {
        let mut key_iter = self.map.iter();
        let current_map_entry = key_iter.next();
        HashMapBucketIter {
            key_iter,
            current_map_entry,
            current_vec_index: 0,
        }
    }
}

// Specify 'a - the lifetime, and K,V on both sides.
// If you wanted to change how the iterator acts for a given type of key or
// value you could cange the left-hand side.
impl <'a, K, V> Iterator for HashMapBucketIter<'a, K, V> {
    // Define `Item` - a type used inside the trait - to be a reference to a key and value.
    // This specifies the type that the iterator will return.
    type Item = (&'a K, &'a V);

    // You use Item to specify the type returned by `Next`. It's always an option of the type.
    fn next(&mut self) -> Option<Self::Item> {
        // If there is a current map entry, and a current vec index
        if let Some((key, values)) = self.current_map_entry {
            // If the index is less than the length of the vector
            if self.current_vec_index < values.len() {
                // Get the value at the current index
                let value = &values[self.current_vec_index];
                // Increment the index
                self.current_vec_index += 1;
                // Return the key and value
                return Some((key, value));
            } else {
                // We're past the end of the vector, next key
                self.current_map_entry = self.key_iter.next();
                self.current_vec_index = 0;

                if let Some((key, values)) = self.current_map_entry {
                    // If the index is less than the length of the vector
                    if self.current_vec_index < values.len() {
                        // Get the value at the current index
                        let value = &values[self.current_vec_index];
                        // Increment the index
                        self.current_vec_index += 1;
                        // Return the key and value
                        return Some((key, value));
                    }
                }
            }
        }

        None
    }
}

struct Degrees(f32);
struct Radians(f32);

impl From<Radians> for Degrees {
    fn from(rad: Radians) -> Self {
        Degrees(rad.0 * 180.0 / std::f32::consts::PI)
    }
}
impl From<Degrees> for Radians {
    fn from(deg: Degrees) -> Self {
        Radians(deg.0 * std::f32::consts::PI / 100.0)
    }
}

fn sin(angle: impl Into<Radians>) -> f32 {
    let angle: Radians = angle.into();
    angle.0.sin()
}


fn print_it<T: ToString>(x: T) {
    println!("{}", x.to_string());
}
// fn print_it<T>(x: T)
// where T: ToString + Debug,
// {
//     println!("{}", x.to_string());
// }

fn main() {
    print_it("Hello");
    print_it(5);

    let behind_you = Degrees(180.0);
    let behind_you_radians = Radians::from(behind_you);
    let behind_you_radians2: Radians = Degrees(180.0).into();

    let mut bucket = HashMapBucket::new();
    bucket.insert("Hello", 1);
    bucket.insert("Hello", 2);

    for (key, value) in bucket.iter() {
        println!("{key}, {value}");
    }
}
