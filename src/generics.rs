use std::cmp::Ordering;

// 10. Generics
pub fn run() {
    println!("{}", largest(&vec![2, 10, 12, 54, 12]));
    println!(
        "{:?}",
        largest(&vec![Point::new(2), Point::new(3), Point::new(4)])
    );
    println!("{}", Point::new(2.0).describe());

    let mp = MixedPoint{ x: 2.5, y: 3 };
    let mp2 = MixedPoint{ x: "siema", y: false };

    let mp3 = mp.mix(mp2);
    println!("{:?}", mp3);

}

fn largest<T>(list: &[T]) -> &T
where
    T: std::cmp::PartialOrd,
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// indicate that T is cloneable.
impl<T: Clone> Point<T> {
    fn new(val: T) -> Self {
        Point {
            x: val.clone(),
            y: val,
        }
    }
}

// implementation for specific T type.
impl Point<f32> {
    fn describe(&self) -> String {
        "if f32".to_string()
    }
}

// implement PartialOrd trait and indicate that T implements it itself.
impl<T> PartialOrd for Point<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

// implement PartialEq trait and indicate that T implements it itself.
impl<T> PartialEq<Self> for Point<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x)
    }
}

#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U
}

// types in method scope do not have to be the same as types on interface.
impl <T, U> MixedPoint<T, U> {
    fn mix<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y
        }
    }
}