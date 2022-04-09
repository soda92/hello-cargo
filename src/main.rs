#[doc(inline)]
pub use bar::Bar;

/// bar docs
mod bar {
    /// the docs for Bar
    pub struct Bar;
}

/// A human being is represented here
pub struct Person {
    /// A person must have a name, no matter how much Juliet may hate it
    name: String,
}
impl Person {
    /// Returns a person with the name given them
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the person
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// Gives a friendly hello!
    ///
    /// Says "Hello, \[name\]" to the `Person` it is called on.
    pub fn hello(&self) {
        println!("Hello, {}!", self.name);
    }
}

extern crate crossbeam;

fn main() {
    let x = 1;
    let y = x + 1;
    println!("{}", y);
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("{}", months[0]);
    println!("Hello, world!");

    let mut array = [1, 2, 3];
    #[allow(unused_must_use)]
    {
        crossbeam::scope(|scope| {
            for x in &mut array {
                scope.spawn(move |_| {
                    *x += 1;
                });
            }
        });
    }

    println!("{:?}", array);
}
