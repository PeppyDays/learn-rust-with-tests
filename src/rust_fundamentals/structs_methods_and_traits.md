# Structs, Methods and Traits

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/structs).

Suppose that we need some geometry code to calculate the perimeter of a rectangle given a height and width. We can write a `Perimeter(width: f64, height: f64)` function, where `f64` is for floating-point numbers like 123.45.

The TDD cycle should be pretty familiar to you by now.

## The First Requirement: Perimeter and Area of a Rectangle

### Write the Test First

```rust
#[cfg(test)]
mod specs_for_perimeter {
    #[test]
    fn sut_returns_perimeter_correctly() {
        // Arrange
        let width = 10.0;
        let height = 10.0;

        // Act
        let actual = perimeter(width, height);

        // Assert
        let expected = 40.0;
        assert_eq!(expected, actual);
    }
}
```

### Try to Run the Test

```bash
error[E0425]: cannot find function `perimeter` in this scope
  --> src/lib.rs:10:22
   |
10 |         let actual = perimeter(width, height);
   |                      ^^^^^^^^^ not found in this scope
```

### Write the Minimal Amount of Code

```rust
pub fn perimeter(width: f64, height: f64) -> f64 {
    0.0
}

#[cfg(test)]
mod specs_for_perimeter {
    use super::perimeter;

    #[test]
    fn sut_returns_perimeter_correctly() {
        // Arrange
        let width = 10.0;
        let height = 10.0;

        // Act
        let actual = perimeter(width, height);

        // Assert
        let expected = 40.0;
        assert_eq!(expected, actual);
    }
}
```

Results is as expected.

```bash
thread 'specs_for_perimeter::sut_returns_perimeter_correctly' panicked at src/lib.rs:20:9:
assertion `left == right` failed
  left: 40.0
 right: 0.0
```

### Write Enough Code to Make It Pass

```rust
pub fn perimeter(width: f64, height: f64) -> f64 {
    2.0 * (width + height)
}
#
# #[cfg(test)]
# mod specs_for_perimeter {
#     use super::perimeter;
#
#     #[test]
#     fn sut_returns_perimeter_correctly() {
#         // Arrange
#         let width = 10.0;
#         let height = 10.0;
#
#         // Act
#         let actual = perimeter(width, height);
#
#         // Assert
#         let expected = 40.0;
#         assert_eq!(expected, actual);
#     }
# }
```

So far, so easy. Now let's create a function called `area(width: f64, height: f64)` which returns the area of a rectangle.

Try to do it by yourself, following the TDD cycle.

You should end up with tests like this.

```rust
pub fn perimeter(width: f64, height: f64) -> f64 {
    2.0 * (width + height)
}

pub fn area(width: f64, height: f64) -> f64 {
    width * height
}

#[cfg(test)]
mod specs_for_perimeter {
    use super::perimeter;

    #[test]
    fn sut_returns_perimeter_correctly() {
        // Arrange
        let width = 10.0;
        let height = 10.0;

        // Act
        let actual = perimeter(width, height);

        // Assert
        let expected = 40.0;
        assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod specs_for_area {
    use super::area;

    #[test]
    fn sut_returns_area_correctly() {
        // Arrange
        let width = 12.0;
        let height = 6.0;

        // Act
        let actual = area(width, height);

        // Assert
        let expected = 72.0;
        assert_eq!(expected, actual);
    }
}
```

### Refactor

Our code does the job, but it doesn't contain anything explicit about rectangles. An unwary developer might try to supply the width and height of a triangle to these functions without realising they will return the wrong answer.

We could just give the functions more specific names like `rectangle_area`. A neater solution is to define our own type called `Rectangle` which encapsulates this concept for us.

We can create a simple type using a **struct**. A [struct](https://doc.rust-lang.org/stable/book/ch05-01-defining-structs.html) is just a named collection of fields where you can store data.

Declare a struct in `lib.rs` like this.

```rust
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}
```

Now let's refactor the tests to use `Rectangle` instead of plain `f64` values, and modify the test name to specify that we are testing the `Rectangle` struct.

```rust
# pub struct Rectangle {
#     pub width: f64,
#     pub height: f64,
# }
#
# pub fn perimeter(width: f64, height: f64) -> f64 {
#     2.0 * (width + height)
# }
#
# pub fn area(width: f64, height: f64) -> f64 {
#     width * height
# }
#
#[cfg(test)]
mod specs_for_perimeter {
    use super::Rectangle;
    use super::perimeter;

    #[test]
    fn sut_returns_perimeter_of_rectangle_correctly() {
        // Arrange
        let rectangle = Rectangle {
            width: 10.0,
            height: 10.0,
        };

        // Act
        let actual = perimeter(rectangle);

        // Assert
        let expected = 40.0;
        assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod specs_for_area {
    use super::Rectangle;
    use super::area;

    #[test]
    fn sut_returns_area_of_rectangle_correctly() {
        // Arrange
        let rectangle = Rectangle {
            width: 12.0,
            height: 6.0,
        };

        // Act
        let actual = area(rectangle);

        // Assert
        let expected = 72.0;
        assert_eq!(expected, actual);
    }
}
```

Remember to run your tests before attempting to fix. The tests should show a helpful error like this.

```bash
15 | pub fn perimeter(width: f64, height: f64) -> f64 {
   |        ^^^^^^^^^ ----------  -----------
help: provide the argument
   |
37 -         let actual = perimeter(rectangle);
37 +         let actual = perimeter(/* f64 */, /* f64 */);
```

You can access the fields of a struct with the syntax `rectangle.width` and `rectangle.height`. Change the two functions to fix the test.

```rust
# pub struct Rectangle {
#     pub width: f64,
#     pub height: f64,
# }
#
pub fn perimeter(rectangle: Rectangle) -> f64 {
    2.0 * (rectangle.width + rectangle.height)
}

pub fn area(rectangle: Rectangle) -> f64 {
    rectangle.width * rectangle.height
}
#
# #[cfg(test)]
# mod specs_for_perimeter {
#     use super::Rectangle;
#     use super::perimeter;
#
#     #[test]
#     fn sut_returns_perimeter_of_rectangle_correctly() {
#         // Arrange
#         let rectangle = Rectangle {
#             width: 10.0,
#             height: 10.0,
#         };
#
#         // Act
#         let actual = perimeter(rectangle);
#
#         // Assert
#         let expected = 40.0;
#         assert_eq!(expected, actual);
#     }
# }
#
# #[cfg(test)]
# mod specs_for_area {
#     use super::Rectangle;
#     use super::area;
#
#     #[test]
#     fn sut_returns_area_of_rectangle_correctly() {
#         // Arrange
#         let rectangle = Rectangle {
#             width: 12.0,
#             height: 6.0,
#         };
#
#         // Act
#         let actual = area(rectangle);
#
#         // Assert
#         let expected = 72.0;
#         assert_eq!(expected, actual);
#     }
# }
```

I hope you'll agree that passing a `Rectangle` to a function conveys our intent more clearly, but there are more benefits of using structs that we will cover later.

## The Second Requirement: Perimeter and Area of a Circle

Our next requirement is to write an `area` function for circles.

### Write the Test First

```rust
# pub struct Rectangle {
#     pub width: f64,
#     pub height: f64,
# }
#
# pub fn perimeter(rectangle: Rectangle) -> f64 {
#     2.0 * (rectangle.width + rectangle.height)
# }
#
# pub fn area(rectangle: Rectangle) -> f64 {
#     rectangle.width * rectangle.height
# }
#
# #[cfg(test)]
# mod specs_for_perimeter {
#     use super::Rectangle;
#     use super::perimeter;
#
#     #[test]
#     fn sut_returns_perimeter_of_rectangle_correctly() {
#         // Arrange
#         let rectangle = Rectangle {
#             width: 10.0,
#             height: 10.0,
#         };
#
#         // Act
#         let actual = perimeter(rectangle);
#
#         // Assert
#         let expected = 40.0;
#         assert_eq!(expected, actual);
#     }
# }
#
#[cfg(test)]
mod specs_for_area {
    use super::Rectangle;
    use super::area;

    #[test]
    fn sut_returns_area_of_rectangle_correctly() {
        // Arrange
        let rectangle = Rectangle {
            width: 12.0,
            height: 6.0,
        };

        // Act
        let actual = area(rectangle);

        // Assert
        let expected = 72.0;
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_returns_area_of_circle_correctly() {
        // Arrange
        let circle = Circle { radius: 10.0 };

        // Act
        let actual = area(circle);

        // Assert
        let expected = 314.1592653589793;
        assert_eq!(expected, actual);
    }
}
```

### Try to Run the Test

```bash
error[E0422]: cannot find struct, variant or union type `Circle` in this scope
  --> src/lib.rs:69:22
   |
69 |         let circle = Circle { radius: 10.0 };
   |                      ^^^^^^ not found in this scope
```

### Write the Minimal Amount of Code

We need to define our `Circle` struct.

```rust
# pub struct Rectangle {
#     pub width: f64,
#     pub height: f64,
# }
#
pub struct Circle {
    pub radius: f64,
}
#
# pub fn perimeter(rectangle: Rectangle) -> f64 {
#     2.0 * (rectangle.width + rectangle.height)
# }
#
# pub fn area(rectangle: Rectangle) -> f64 {
#     rectangle.width * rectangle.height
# }
#
# #[cfg(test)]
# mod specs_for_perimeter {
#     use super::Rectangle;
#     use super::perimeter;
#
#     #[test]
#     fn sut_returns_perimeter_of_rectangle_correctly() {
#         // Arrange
#         let rectangle = Rectangle {
#             width: 10.0,
#             height: 10.0,
#         };
#
#         // Act
#         let actual = perimeter(rectangle);
#
#         // Assert
#         let expected = 40.0;
#         assert_eq!(expected, actual);
#     }
# }
#
# #[cfg(test)]
# mod specs_for_area {
#     use super::Rectangle;
#     use super::area;
#
#     #[test]
#     fn sut_returns_area_of_rectangle_correctly() {
#         // Arrange
#         let rectangle = Rectangle {
#             width: 12.0,
#             height: 6.0,
#         };
#
#         // Act
#         let actual = area(rectangle);
#
#         // Assert
#         let expected = 72.0;
#         assert_eq!(expected, actual);
#     }
#
#     #[test]
#     fn sut_returns_area_of_circle_correctly() {
#         // Arrange
#         let circle = Circle { radius: 10.0 };
#
#         // Act
#         let actual = area(circle);
#
#         // Assert
#         let expected = 314.1592653589793;
#         assert_eq!(expected, actual);
#     }
# }
```

Now try to run the tests again.

```bash
error[E0308]: mismatched types
  --> src/lib.rs:77:27
   |
77 |         let actual = area(circle);
   |                      ---- ^^^^^^ expected `Rectangle`, found `Circle`
   |                      |
   |                      arguments to this function are incorrect
```

Some programming languages allow you to do something like this.

```rust,ignore
fn area(circle: Circle) -> f64 {}
fn area(rectangle: Rectangle) -> f64 {}
```

But you cannot in Rust.

```bash
23 | pub fn area(rectangle: Rectangle) -> f64 {
   | ---------------------------------------- previous definition of the value `area` here
...
27 | pub fn area(circle: Circle) -> f64 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `area` redefined here
   |
   = note: `area` must be defined only once in the value namespace of this module
```

We have two choices:

- We can have functions with the same name declared in different packages
  - We could create our `area(circle: Circle)` function in a new package
  - Feels overkill here
- We can define [methods](https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html) on our newly defined types instead

### What are Methods?

So far we have only been writing functions.

A method is a function with a receiver. A method declaration binds an identifier, the method name, to a method, and associates the method with the receiver's base type.

Methods are very similar to functions but they are called by invoking them on an instance of a particular type. Where you can just call functions wherever you like, such as `area(rectangle)` you can only call methods on "things".

An example will help, so let's change our tests first to call methods instead and then fix the code.

```rust
# pub struct Rectangle {
#     pub width: f64,
#     pub height: f64,
# }
#
# pub struct Circle {
#     pub radius: f64,
# }
#
# pub fn perimeter(rectangle: Rectangle) -> f64 {
#     2.0 * (rectangle.width + rectangle.height)
# }
#
# pub fn area(rectangle: Rectangle) -> f64 {
#     rectangle.width * rectangle.height
# }
#
# #[cfg(test)]
# mod specs_for_perimeter {
#     use super::Rectangle;
#     use super::perimeter;
#
#     #[test]
#     fn sut_returns_perimeter_of_rectangle_correctly() {
#         // Arrange
#         let rectangle = Rectangle {
#             width: 10.0,
#             height: 10.0,
#         };
#
#         // Act
#         let actual = perimeter(rectangle);
#
#         // Assert
#         let expected = 40.0;
#         assert_eq!(expected, actual);
#     }
# }
#
#[cfg(test)]
mod specs_for_area {
    use super::Circle;
    use super::Rectangle;

    #[test]
    fn sut_returns_area_of_rectangle_correctly() {
        // Arrange
        let rectangle = Rectangle {
            width: 12.0,
            height: 6.0,
        };

        // Act
        let actual = rectangle.area();

        // Assert
        let expected = 72.0;
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_returns_area_of_circle_correctly() {
        // Arrange
        let circle = Circle { radius: 10.0 };

        // Act
        let actual = circle.area();

        // Assert
        let expected = 314.1592653589793;
        assert_eq!(expected, actual);
    }
}
```

If we try to run the tests, we get:

```bash
error[E0599]: no method named `area` found for struct `Circle` in the current scope
  --> src/lib.rs:76:29
   |
15 | pub struct Circle {
   | ----------------- method `area` not found for this struct
...
76 |         let actual = circle.area();
   |                             ^^^^ method not found in `Circle`
```

I would like to reiterate how great the compiler is here. It is so important to take the time to slowly read the error messages you get, it will help you in the long run.

### Write the Minimal Amount of Code

Let's add some methods to our types.

```rust
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn area(&self) -> f64 {
        0.0
    }
}

pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn area(&self) -> f64 {
        0.0
    }
}
#
# pub fn perimeter(rectangle: Rectangle) -> f64 {
#     2.0 * (rectangle.width + rectangle.height)
# }
#
# #[cfg(test)]
# mod specs_for_perimeter {
#     use super::Rectangle;
#     use super::perimeter;
#
#     #[test]
#     fn sut_returns_perimeter_of_rectangle_correctly() {
#         // Arrange
#         let rectangle = Rectangle {
#             width: 10.0,
#             height: 10.0,
#         };
#
#         // Act
#         let actual = perimeter(rectangle);
#
#         // Assert
#         let expected = 40.0;
#         assert_eq!(expected, actual);
#     }
# }
#
# #[cfg(test)]
# mod specs_for_area {
#     use super::Circle;
#     use super::Rectangle;
#
#     #[test]
#     fn sut_returns_area_of_rectangle_correctly() {
#         // Arrange
#         let rectangle = Rectangle {
#             width: 12.0,
#             height: 6.0,
#         };
#
#         // Act
#         let actual = rectangle.area();
#
#         // Assert
#         let expected = 72.0;
#         assert_eq!(expected, actual);
#     }
#
#     #[test]
#     fn sut_returns_area_of_circle_correctly() {
#         // Arrange
#         let circle = Circle { radius: 10.0 };
#
#         // Act
#         let actual = circle.area();
#
#         // Assert
#         let expected = 314.1592653589793;
#         assert_eq!(expected, actual);
#     }
# }
```

The syntax for declaring methods is almost the same as functions and that's because they're so similar. Methods are declared inside an `impl` block, and the first argument is the receiver type `&self`. The `&` means that the method takes a reference to the receiver. The receiver type can be changed to `&mut self` or `self` depending on the context you want to pass in.

When your method is called on a variable of that type, you get your reference to its data via the `self` variable. In many other programming languages this is done implicitly and you access the receiver via this.

If you try to re-run the tests, they should now compile and give you some failing output.

### Write Enough Code to Make It Pass

Now let's make our rectangle tests pass by fixing our new method.

```rust
use std::f64::consts::PI;

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}

pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}
#
# pub fn perimeter(rectangle: Rectangle) -> f64 {
#     2.0 * (rectangle.width + rectangle.height)
# }
#
# #[cfg(test)]
# mod specs_for_perimeter {
#     use super::Rectangle;
#     use super::perimeter;
#
#     #[test]
#     fn sut_returns_perimeter_of_rectangle_correctly() {
#         // Arrange
#         let rectangle = Rectangle {
#             width: 10.0,
#             height: 10.0,
#         };
#
#         // Act
#         let actual = perimeter(rectangle);
#
#         // Assert
#         let expected = 40.0;
#         assert_eq!(expected, actual);
#     }
# }
#
# #[cfg(test)]
# mod specs_for_area {
#     use super::Circle;
#     use super::Rectangle;
#
#     #[test]
#     fn sut_returns_area_of_rectangle_correctly() {
#         // Arrange
#         let rectangle = Rectangle {
#             width: 12.0,
#             height: 6.0,
#         };
#
#         // Act
#         let actual = rectangle.area();
#
#         // Assert
#         let expected = 72.0;
#         assert_eq!(expected, actual);
#     }
#
#     #[test]
#     fn sut_returns_area_of_circle_correctly() {
#         // Arrange
#         let circle = Circle { radius: 10.0 };
#
#         // Act
#         let actual = circle.area();
#
#         // Assert
#         let expected = 314.1592653589793;
#         assert_eq!(expected, actual);
#     }
# }
```

If you re-run the tests, they should now pass.

We can do the same thing for the `perimeter` function. Do this by yourself, and you should end up with something like this.

```rust
use std::f64::consts::PI;

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

#[cfg(test)]
mod specs_for_perimeter {
    use super::Circle;
    use super::Rectangle;

    #[test]
    fn sut_returns_perimeter_of_rectangle_correctly() {
        // Arrange
        let rectangle = Rectangle {
            width: 10.0,
            height: 10.0,
        };

        // Act
        let actual = rectangle.perimeter();

        // Assert
        let expected = 40.0;
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_returns_perimeter_of_circle_correctly() {
        // Arrange
        let circle = Circle { radius: 10.0 };

        // Act
        let actual = circle.perimeter();

        // Assert
        let expected = 62.83185307179586;
        assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod specs_for_area {
    use super::Circle;
    use super::Rectangle;

    #[test]
    fn sut_returns_area_of_rectangle_correctly() {
        // Arrange
        let rectangle = Rectangle {
            width: 12.0,
            height: 6.0,
        };

        // Act
        let actual = rectangle.area();

        // Assert
        let expected = 72.0;
        assert_eq!(expected, actual);
    }

    #[test]
    fn sut_returns_area_of_circle_correctly() {
        // Arrange
        let circle = Circle { radius: 10.0 };

        // Act
        let actual = circle.area();

        // Assert
        let expected = 314.1592653589793;
        assert_eq!(expected, actual);
    }
}
```

### Refactor

Nothing to do here.

## The Third Requirement: Summing Up Area of Shapes

Now we have two types, `Rectangle` and `Circle`, and we have to sum up their areas when we have a collection of them. Wait, what? Do we have to sum up the areas of different types? How can we do that?

With Rust, we can codify this with [traits](https://doc.rust-lang.org/stable/book/ch10-02-traits.html). Storing different types in a single collection can be also done by using enumeration in Rust, but we will not cover that in this chapter.

Traits are similar to interfaces in other languages. They allow you to define shared behavior across different types. In this case, we can define a trait called `Shape` that has a method `area` and implement it for both `Rectangle` and `Circle`.

Let's introduce this.

### Write the Test First

```rust
# use std::f64::consts::PI;
#
# pub struct Rectangle {
#     pub width: f64,
#     pub height: f64,
# }
#
# impl Rectangle {
#     pub fn area(&self) -> f64 {
#         self.width * self.height
#     }
#
#     pub fn perimeter(&self) -> f64 {
#         2.0 * (self.width + self.height)
#     }
# }
#
# pub struct Circle {
#     pub radius: f64,
# }
#
# impl Circle {
#     pub fn area(&self) -> f64 {
#         PI * self.radius * self.radius
#     }
#
#     pub fn perimeter(&self) -> f64 {
#         2.0 * PI * self.radius
#     }
# }
#
#[cfg(test)]
mod specs_for_sum_areas {
    #[test]
    fn sut_returns_sum_of_areas_if_rectangle_and_circle_are_given() {
        // Arrange
        let rectangle = super::Rectangle {
            width: 10.0,
            height: 10.0,
        };
        let circle = super::Circle { radius: 10.0 };
        let shapes: Vec<&dyn Shape> = vec![&rectangle, &circle];

        // Act
        let actual = sum_areas(&shapes);

        // Assert
        let expected = 414.1592653589793;
        assert_eq!(expected, actual);
    }
}
```

You might wondering why we are using `&dyn Shape` instead of `Shape`. Rust compiler allows variable's size to be determined at compile time. The implementation of `Shape` can have different sizes, so that Rust doesn't allow setting type as `Vec<Shape>`.

As a result, we have to use `&dyn Shape` or `Box<dyn Shape>` to store different types in a single collection. `&dyn Shape` is a reference to a trait object, and `Box<dyn Shape>` is a heap-allocated trait object.

If you need to store a variable with ownership, you can use `Box<dyn Shape>`. If you don't need ownership, you can use `&dyn Shape`. In this case, we only need a reference because we only get the generated value of trait object's properties. So we can use `&dyn Shape`.

All details for this are covered [here](https://doc.rust-lang.org/stable/book/ch18-02-trait-objects.html?highlight=dyn#trait-objects-perform-dynamic-dispatch).

### Try to Run the Test

You can see compiler errors, and let's fix them.

### Write the Minimal Amount of Code

Declare the `Shape` trait and implement it for both `Rectangle` and `Circle`. After that, implement the `sum_areas` function with empty body.

```rust
use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

pub fn sum_areas(shapes: &[&dyn Shape]) -> f64 {
    0.0
}
#
# #[cfg(test)]
# mod specs_for_sum_areas {
#     use super::Shape;
#     use super::sum_areas;
#
#     #[test]
#     fn sut_returns_sum_of_areas_if_rectangle_and_circle_are_given() {
#         // Arrange
#         let rectangle = super::Rectangle {
#             width: 10.0,
#             height: 10.0,
#         };
#         let circle = super::Circle { radius: 10.0 };
#         let shapes: Vec<&dyn Shape> = vec![&rectangle, &circle];
#
#         // Act
#         let actual = sum_areas(&shapes);
#
#         // Assert
#         let expected = 414.1592653589793;
#         assert_eq!(expected, actual);
#     }
# }
```

You might be accustomed to writing a function with multiple inputs like `&[T]` because we did it in [the previous chapter](arrays_and_slices.md). The only different is that the `T` is now a trait object with a reference `&dyn Shape`.

### Write Enough Code to Make It Pass

```rust
# use std::f64::consts::PI;
#
# pub trait Shape {
#     fn area(&self) -> f64;
#     fn perimeter(&self) -> f64;
# }
#
# pub struct Rectangle {
#     pub width: f64,
#     pub height: f64,
# }
#
# impl Shape for Rectangle {
#     fn area(&self) -> f64 {
#         self.width * self.height
#     }
#
#     fn perimeter(&self) -> f64 {
#         2.0 * (self.width + self.height)
#     }
# }
#
# pub struct Circle {
#     pub radius: f64,
# }
#
# impl Shape for Circle {
#     fn area(&self) -> f64 {
#         PI * self.radius * self.radius
#     }
#
#     fn perimeter(&self) -> f64 {
#         2.0 * PI * self.radius
#     }
# }
#
pub fn sum_areas(shapes: &[&dyn Shape]) -> f64 {
    let mut total_area = 0.0;
    for shape in shapes {
        total_area += shape.area();
    }
    total_area
}
#
# #[cfg(test)]
# mod specs_for_sum_areas {
#     use super::Shape;
#     use super::sum_areas;
#
#     #[test]
#     fn sut_returns_sum_of_areas_if_rectangle_and_circle_are_given() {
#         // Arrange
#         let rectangle = super::Rectangle {
#             width: 10.0,
#             height: 10.0,
#         };
#         let circle = super::Circle { radius: 10.0 };
#         let shapes: Vec<&dyn Shape> = vec![&rectangle, &circle];
#
#         // Act
#         let actual = sum_areas(&shapes);
#
#         // Assert
#         let expected = 414.1592653589793;
#         assert_eq!(expected, actual);
#     }
# }
```

If you run the tests, they should pass.

### Refactor

Nothing to do here for `sum_areas`, but the test modules `specs_for_perimeter` and `specs_for_area` can be combined into one module because we introduced the `Shape` trait. This is a good time to refactor the tests to use the `Shape` trait.

Let's see the two tests we wrote.

```rust
#[test]
fn sut_returns_perimeter_of_rectangle_correctly() {
    // Arrange
    let rectangle = Rectangle {
        width: 10.0,
        height: 10.0,
    };

    // Act
    let actual = rectangle.perimeter();

    // Assert
    let expected = 40.0;
    assert_eq!(expected, actual);
}

#[test]
fn sut_returns_perimeter_of_circle_correctly() {
    // Arrange
    let radius = 10.0;
    let circle = Circle { radius };

    // Act
    let actual = circle.perimeter();

    // Assert
    let expected = 62.83185307179586;
    assert_eq!(expected, actual);
}
```

Don't you think the two tests are similar because we are testing the same method on `Shape` trait object?

It's time to introduce parameterised tests. Parameterised tests are a way to run the same test with different inputs. To do that, we will use the `rstest` crate. This crate enables us to write advances tests including parameterised tests. We'll keep using this crate and learning more advanced features in the following chapters.

To use `rstest`, add the following to your `Cargo.toml` file.

```toml
[package]
name = "structs"
version = "0.1.0"
edition = "2024"

[dependencies]

[dev-dependencies]
rstest = "0.25"
```

Check the [documentation for parameterised tests](https://docs.rs/rstest/latest/rstest/#creating-parametrized-tests) with examples first, and try to simplify our tests. You might end up with following.

```rust
# use std::f64::consts::PI;
#
# pub trait Shape {
#     fn area(&self) -> f64;
#     fn perimeter(&self) -> f64;
# }
#
# pub struct Rectangle {
#     pub width: f64,
#     pub height: f64,
# }
#
# impl Shape for Rectangle {
#     fn area(&self) -> f64 {
#         self.width * self.height
#     }
#
#     fn perimeter(&self) -> f64 {
#         2.0 * (self.width + self.height)
#     }
# }
#
# pub struct Circle {
#     pub radius: f64,
# }
#
# impl Shape for Circle {
#     fn area(&self) -> f64 {
#         PI * self.radius * self.radius
#     }
#
#     fn perimeter(&self) -> f64 {
#         2.0 * PI * self.radius
#     }
# }
#
# pub fn sum_areas(shapes: &[&dyn Shape]) -> f64 {
#     let mut total_area = 0.0;
#     for shape in shapes {
#         total_area += shape.area();
#     }
#     total_area
# }
#
# #[cfg(test)]
# mod specs_for_sum_areas {
#     use super::Shape;
#     use super::sum_areas;
#
#     #[test]
#     fn sut_returns_sum_of_areas_if_rectangle_and_circle_are_given() {
#         // Arrange
#         let rectangle = super::Rectangle {
#             width: 10.0,
#             height: 10.0,
#         };
#         let circle = super::Circle { radius: 10.0 };
#         let shapes: Vec<&dyn Shape> = vec![&rectangle, &circle];
#
#         // Act
#         let actual = sum_areas(&shapes);
#
#         // Assert
#         let expected = 414.1592653589793;
#         assert_eq!(expected, actual);
#     }
# }
#
#[cfg(test)]
mod specs_for_shape {
    use rstest::rstest;

    use super::Circle;
    use super::Rectangle;
    use super::Shape;

    #[rstest]
    #[case(Rectangle {width: 10.0, height: 10.0}, 40.0)]
    #[case(Circle {radius: 10.0}, 62.83185307179586)]
    fn sut_returns_perimeter_of_shape_correctly(#[case] shape: impl Shape, #[case] expected: f64) {
        // Act
        let actual = shape.perimeter();

        // Assert
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(Rectangle {width: 12.0, height: 6.0}, 72.0)]
    #[case(Circle {radius: 10.0}, 314.1592653589793)]
    fn sut_returns_area_of_shape_correctly(#[case] shape: impl Shape, #[case] expected: f64) {
        // Act
        let actual = shape.area();

        // Assert
        assert_eq!(expected, actual);
    }
}
```

Excited to see the tests are super simplified!

## Wrapping Up

This was more TDD practice, iterating over our solutions to basic mathematic problems and learning new language features motivated by our tests.

- Declaring structs to create your own data types which lets you bundle related data together and make the intent of your code clearer
- Declaring traits so you can define functions that can be used by different types, called [parametric polymorphism](https://en.wikipedia.org/wiki/Parametric_polymorphism)
- Adding methods so you can add functionality to your data types, and implementing traits for your data types
- Parameterised tests to make your tests more readable and maintainable
