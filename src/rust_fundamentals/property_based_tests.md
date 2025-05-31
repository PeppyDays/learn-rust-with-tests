# Property-based Tests

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/roman).

Some companies will ask you to do the [Roman Numeral Kata](http://codingdojo.org/kata/RomanNumerals/) as part of the interview process. This chapter will show how you can tackle it with TDD.

## The First Requirement: Convert Arabic Numbers to Roman Numerals - 1

We are going to write a function which converts an [Arabic number](https://en.wikipedia.org/wiki/Arabic_numerals) (numbers 0 to 9) to a Roman Numeral. If you haven't heard of [Roman Numerals](https://en.wikipedia.org/wiki/Roman_numerals) they are how the Romans wrote down numbers.

You build them by sticking symbols together and those symbols represent numbers. So `I` is one. `III` is three. Seems easy but there's a few interesting rules. `V` means five, but `IV` is 4 (not `IIII`). `MCMLXXXIV` is 1984. That looks complicated and it's hard to imagine how we can write code to figure this out right from the start.

As this book stresses, a key skill for software developers is to try and identify "thin vertical slices" of useful functionality and then iterating. The TDD workflow helps facilitate iterative development.

So rather than 1984, let's start with 1.

### Write the Test First

```rust
#[cfg(test)]
#[allow(non_snake_case)]
mod specs_for_convert_to_roman {
    #[test]
    fn sut_converts_1_to_I() {
        // Arrange
        let arabic = 1;

        // Act
        let actual = convert_to_roman(arabic);

        // Assert
        let expected = "I";
        assert_eq!(expected, actual);
    }
}
```

If you've got this far in the book this is hopefully feeling very boring and routine to you. That's a good thing. TDD is a skill converting anxiety into routine.

We added `#[allow(non_snake_case)]` to the test module because clippy warns that the function name should be lowercases. We are using `sut_converts_1_to_I` as a test name, which is not in snake case. By adding this attribute we are telling clippy to ignore this warning for this module.

### Try to Run the Test

```bash
error[E0425]: cannot find function `convert_to_roman` in this scope
  --> src/v1.rs:22:22
   |
22 |         let actual = convert_to_roman(arabic);
   |                      ^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function
   |
3  +     use crate::convert_to_roman;
```

### Write the Minimal Amount of Code

Create our function but don't make the test pass yet, always make sure the tests fails how you expect.

```rust
pub fn convert_to_roman(arabic: usize) -> String {
    "".to_string()
}
```

It should run now.

```bash
thread 'v1::specs_for_convert_to_roman::sut_converts_1_to_I' panicked at src/v1.rs:20:9:
assertion `left == right` failed
  left: "I"
  right: ""
```

### Write Enough Code to Make It Pass

```rust
pub fn convert_to_roman(arabic: usize) -> String {
    "I".to_string()
}
```

### Refactor

Not much to refactor yet.

I know it feels weird just to hard-code the result but with TDD we want to stay out of red for as long as possible. It may feel like we haven't accomplished much but we've defined our API and got a test capturing one of our rules; even if the real code is pretty dumb.

Now use that uneasy feeling to write a new test to force us to write slightly less dumb code.

## The Second Requirement: Convert Arabic Numbers to Roman Numerals - 2

### Write the Test First

Let's add a new test to convert 2 to `II`.

```rust,ignore
#[test]
fn sut_converts_2_to_II() {
    // Arrange
    let arabic = 2;

    // Act
    let actual = convert_to_roman(arabic);

    // Assert
    let expected = "II";
    assert_eq!(expected, actual);
```

### Try to Run the Test

```bash
thread 'v1::specs_for_convert_to_roman::sut_converts_2_to_II' panicked at src/v1.rs:33:9: assertion `left == right` failed
  left: "II"
  right: "I"
```

Not much surprise there.

### Write Enough Code to Make It Pass

```rust
pub fn convert_to_roman(arabic: usize) -> String {
    if arabic == 2 {
        return "II".to_string();
    }
    "I".to_string()
}
```

Yup, it still feels like we're not actually tackling the problem. So we need to write more tests to drive us forward.

### Refactor

We have some repetition in our tests. When you're testing something which feels like it's a matter of "given input X, we expect Y" you should probably use parametrized tests.

You can use the `rstest` crate to do this. Add it to your `Cargo.toml`, and merge the two tests into one.

```toml
[dev-dependencies]
rstest = "0.25"
```

```rust
#[cfg(test)]
mod specs_for_convert_to_roman {
    use super::convert_to_roman;

    #[rstest::rstest]
    #[case(1, "I")]
    #[case(2, "II")]
    fn sut_converts_arabic_to_roman_correctly(#[case] arabic: usize, #[case] expected: &str) {
        // Act
        let actual = convert_to_roman(arabic);

        // Assert
        assert_eq!(expected, actual);
    }
}
```

We can now easily add more cases without having to write any more test boilerplate.

Let's push on and go for 3

## The Third Requirement: Convert Arabic Numbers to Roman Numerals - 3

### Write the Test First

Add the following to our cases.

```rust
#[case(3, "III")]
```

### Try to Run the Test

```bash
thread 'v2::specs_for_convert_to_roman::sut_converts_arabic_to_roman_correctly::case_3' panicked at src/v2.rs:21:9:
assertion `left == right` failed
  left: "III"
  right: "I"
```

### Write Enough Code to Make It Pass

```rust
pub fn convert_to_roman(arabic: usize) -> String {
    if arabic == 3 {
        return "III".to_string();
    }
    if arabic == 2 {
        return "II".to_string();
    }
    "I".to_string()
}
```

### Refactor

OK, so I'm starting to not enjoy these if statements and if you look at the code hard enough you can see that we're building a string of `I` based on the size of `arabic`.

We know that for more complicated numbers we will be doing some kind of arithmetic and string concatenation.

Let's try a refactor with these thoughts in mind, it might not be suitable for the end solution but that's OK. We can always throw our code away and start afresh with the tests we have to guide us.

```rust
pub fn convert_to_roman(arabic: usize) -> String {
    let mut roman = String::new();
    for _ in 0..arabic {
        roman.push('I');
    }
    roman
}
```

The code looks better to me and describes the domain as we know it right now.

## The Fourth Requirement: Convert Arabic Numbers to Roman Numerals - 4

Things start getting more complicated now. The Romans in their wisdom thought repeating characters would become hard to read and count. So a rule with Roman Numerals is you can't have the same character repeated more than 3 times in a row.

Instead you take the next highest symbol and then subtract by putting a symbol to the left of it. Not all symbols can be used as subtractors; only `I` (1), `X` (10) and `C` (100).

For example 5 in Roman Numerals is `V`. To create 4 you do not do `IIII`, instead you do `IV`.

### Write the Test First

```rust
#[case(4, "IV")]
```

### Try to Run the Test

```bash
thread 'v3::specs_for_convert_to_roman::sut_converts_arabic_to_roman_correctly::case_4' panicked at src/v3.rs:23:9:
assertion `left == right` failed
  left: "IV"
  right: "IIII"
```

### Write Enough Code to Make It Pass

```rust
pub fn convert_to_roman(arabic: usize) -> String {
    if arabic == 4 {
        return "IV".to_string();
    }

    let mut roman = String::new();
    for _ in 0..arabic {
        roman.push('I');
    }
    roman
}
```

### Refactor

I don't like that we have broken our string building pattern and I want to carry on with it.

```rust
pub fn convert_to_roman(arabic: usize) -> String {
    let mut roman = String::new();
    for n in (1..=arabic).rev() {
        if n == 4 {
            roman.push_str("IV");
            break;
        }
        roman.push('I');
    }
    roman
}
```

In order for 4 to fit with my current thinking I now count down from the Arabic number, adding symbols to our string as we progress. Not sure if this will work in the long run but let's see!

## The Fifth Requirement: Convert Arabic Numbers to Roman Numerals - 5

Let's make 5 work.

### Write the Test First

```rust
#[case(5, "V")]
```

### Try to Run the Test

```bash
thread 'v3::specs_for_convert_to_roman::sut_converts_arabic_to_roman_correctly::case_5' panicked at src/v3.rs:28:9:
assertion `left == right` failed
  left: "V"
  right: "IIV"
```

### Write Enough Code to Make It Pass

Just copy the approach we did for 4.

```rust
pub fn convert_to_roman(arabic: usize) -> String {
    let mut roman = String::new();
    for n in (1..=arabic).rev() {
        if n == 5 {
            roman.push('V');
            break;
        }
        if n == 4 {
            roman.push_str("IV");
            break;
        }
        roman.push('I');
    }
    roman
}
```

### Refactor

Repetition in loops like this are usually a sign of an abstraction waiting to be called out. Short-circuiting loops can be an effective tool for readability but it could also be telling you something else.

We are looping over our Arabic number and if we hit certain symbols we are calling break but what we are really doing is subtracting over i in a ham-fisted manner.

```rust
pub fn convert_to_roman(mut arabic: usize) -> String {
    let mut roman = String::new();
    while arabic > 0 {
        match arabic {
            n if n >= 5 => {
                roman.push('V');
                arabic -= 5;
            }
            n if n >= 4 => {
                roman.push_str("IV");
                break;
            }
            _ => {
                roman.push('I');
                arabic -= 1;
            }
        }
    }
    roman
}
```

- Given the signals I'm reading from our code, driven from our tests of some very basic scenarios I can see that to build a Roman Numeral I need to subtract from `arabic` as I apply symbols
- The for loop no longer relies on an `n` and instead we will keep building our string until we have subtracted enough symbols away from `arabic`

## The Sixth Requirement: Convert Arabic Numbers to Roman Numerals - 9

I'm pretty sure this approach will be valid for 6 (`VI`), 7 (`VII`) and 8 (`VIII`) too. Nonetheless add the cases in to our test suite and check (I won't include the code for brevity, check the GitHub for samples if you're unsure).

9 follows the same rule as 4 in that we should subtract `I` from the representation of the following number. 10 is represented in roman numerals with `X`; so therefore 9 should be `IX`.

### Write the Test First

```rust
#[case(9, "IX")]
```

### Try to Run the Test

```bash
thread 'v5::specs_for_convert_to_roman::sut_converts_arabic_to_roman_correctly::case_6' panicked at src/v5.rs:38:9:
  assertion `left == right` failed
    left: "IX"
    right: "VIV"
```

### Write Enough Code to Make It Pass

We should be able to adopt the same approach as before.

```rust,ignore
n if n >= 9 => {
    roman.push_str("IX");
    arabic -= 9;
}
```

### Refactor

It feels like the code is still telling us there's a refactor somewhere but it's not totally obvious to me, so let's keep going.

I'll skip the code for this too, but add to your test cases a test for 10 which should be `X` and make it pass before reading on.

Here are a few tests I added as I'm confident up to 39 our code should work.

```rust
#[case(10, "X")]
#[case(14, "XIV")]
#[case(18, "XVIII")]
#[case(20, "XX")]
#[case(39, "XXXIX")]
```

If you've ever done OO programming, you'll know that you should view switch statements with a bit of suspicion. Usually you are capturing a concept or data inside some imperative code when in fact it could be captured in a class structure instead.

Rust isn't strictly OO but that doesn't mean we ignore the lessons OO offers entirely (as much as some would like to tell you).

Our switch statement is describing some truths about Roman Numerals along with behaviour.

We can refactor this by decoupling the data from the behaviour.

```rust
struct RomanNumeral(usize, &'static str);

const ALL_ROMAN_NUMERALS: [RomanNumeral; 5] = [
    RomanNumeral(10, "X"),
    RomanNumeral(9, "IX"),
    RomanNumeral(5, "V"),
    RomanNumeral(4, "IV"),
    RomanNumeral(1, "I"),
];

pub fn convert_to_roman(mut arabic: usize) -> String {
    let mut roman = String::new();
    for RomanNumeral(value, symbol) in ALL_ROMAN_NUMERALS {
        while arabic >= value {
            roman.push_str(symbol);
            arabic -= value;
        }
    }
    roman
}
```

This feels much better. We've declared some rules around the numerals as data rather than hidden in an algorithm and we can see how we just work through the Arabic number, trying to add symbols to our result if they fit.

Does this abstraction work for bigger numbers? Extend the test suite so it works for the Roman number for 50 which is `L`.

Here are some test cases, try and make them pass.

```rust
#[case(40, "XL")]
#[case(47, "XLVII")]
#[case(49, "XLIX")]
#[case(50, "L")]
```

## The Seventh Requirement: Convert Arabic Numbers to Roman Numerals - the Rest

Here are the remaining symbols.

| Arabic | Roman |
| ------ | ----- |
| 100    | C     |
| 500    | D     |
| 1000   | M     |

Take the same approach for the remaining symbols, it should just be a matter of adding data to both the tests and our array of symbols.

Does your code work for 1984: `MCMLXXXIV` ?

Here is my final test suite.

```rust
#[cfg(test)]
mod specs_for_convert_to_roman {
    use super::convert_to_roman;

    #[rstest::rstest]
    #[case(1, "I")]
    #[case(2, "II")]
    #[case(3, "III")]
    #[case(4, "IV")]
    #[case(5, "V")]
    #[case(9, "IX")]
    #[case(10, "X")]
    #[case(14, "XIV")]
    #[case(18, "XVIII")]
    #[case(20, "XX")]
    #[case(39, "XXXIX")]
    #[case(40, "XL")]
    #[case(47, "XLVII")]
    #[case(49, "XLIX")]
    #[case(50, "L")]
    #[case(90, "XC")]
    #[case(100, "C")]
    #[case(400, "CD")]
    #[case(500, "D")]
    #[case(798, "DCCXCVIII")]
    #[case(900, "CM")]
    #[case(1000, "M")]
    #[case(1006, "MVI")]
    #[case(1984, "MCMLXXXIV")]
    #[case(2014, "MMXIV")]
    #[case(3999, "MMMCMXCIX")]
    fn sut_converts_arabic_to_roman_correctly(#[case] arabic: usize, #[case] expected: &str) {
        // Act
        let actual = convert_to_roman(arabic);

        // Assert
        assert_eq!(expected, actual);
    }
}
```

I added a few other edge cases I found just to give me a little more confidence. With table based tests this is very cheap to do.

I didn't change the algorithm, all I had to do was update the `ALL_ROMAN_NUMERALS` array.

```rust,ignore
const ALL_ROMAN_NUMERALS: [RomanNumeral; 13] = [
    RomanNumeral(1000, "M"),
    RomanNumeral(900, "CM"),
    RomanNumeral(500, "D"),
    RomanNumeral(400, "CD"),
    RomanNumeral(100, "C"),
    RomanNumeral(90, "XC"),
    RomanNumeral(50, "L"),
    RomanNumeral(40, "XL"),
    RomanNumeral(10, "X"),
    RomanNumeral(9, "IX"),
    RomanNumeral(5, "V"),
    RomanNumeral(4, "IV"),
    RomanNumeral(1, "I"),
];
```

## The Eighth Requirement: Convert Roman Numerals to Arabic Numbers - 1, 2, and 3

We're not done yet. Next we're going to write a function that converts from a Roman Numeral to an integer.

### Write the Test First

We can reuse our test cases here with a little refactoring. Move the test cases outside of the test as a template variable with [rstest_reuse](https://crates.io/crates/rstest_reuse) crate.

```toml
[dev-dependencies]
rstest = "0.25"
rstest_reuse = "0.7"
```

```rust
#[cfg(test)]
mod specs_for_convert {
    use super::convert_to_arabic;
    use super::convert_to_roman;

    #[rstest_reuse::template]
    #[rstest::rstest]
    #[case(1, "I")]
    #[case(2, "II")]
    #[case(3, "III")]
    #[case(4, "IV")]
    #[case(5, "V")]
    #[case(9, "IX")]
    #[case(10, "X")]
    #[case(14, "XIV")]
    #[case(18, "XVIII")]
    #[case(20, "XX")]
    #[case(39, "XXXIX")]
    #[case(40, "XL")]
    #[case(47, "XLVII")]
    #[case(49, "XLIX")]
    #[case(50, "L")]
    #[case(90, "XC")]
    #[case(100, "C")]
    #[case(400, "CD")]
    #[case(500, "D")]
    #[case(798, "DCCXCVIII")]
    #[case(900, "CM")]
    #[case(1000, "M")]
    #[case(1006, "MVI")]
    #[case(1984, "MCMLXXXIV")]
    #[case(2014, "MMXIV")]
    #[case(3999, "MMMCMXCIX")]
    fn conversion_cases(#[case] _arabic: usize, #[case] _roman: String) {}

    #[rstest_reuse::apply(conversion_cases)]
    fn sut_converts_arabic_to_roman_correctly(#[case] arabic: usize, #[case] expected: &str) {
        // Act
        let actual = convert_to_roman(arabic);

        // Assert
        assert_eq!(expected, actual);
    }

    #[rstest::rstest]
    #[case(1, "I")]
    fn sut_converts_roman_to_arabic_correctly(#[case] expecetd: usize, #[case] roman: &str) {
        // Act
        let actual = convert_to_arabic(roman);

        // Assert
        assert_eq!(expecetd, actual);
    }
}
```

Notice that I am not using the test case template for the `sut_converts_roman_to_arabic_correctly` test. We will add the cases one by one, and will replace to the template later.

### Try to Run the Test

```bash
error[E0432]: unresolved import `super::convert_to_arabic`
  --> src/v7.rs:36:9
   |
36 |     use super::convert_to_arabic;
   |         ^^^^^^^-----------------
   |         |      |
   |         |      help: a similar name exists in the module: `convert_to_roman`
   |         no `convert_to_arabic` in `v7`
```

### Write the Minimal Amount of Code

Add our new function definition.

```rust
pub fn convert_to_arabic(roman: &str) -> usize {
    0
}
```

The test should now run and fail.

```bash
thread 'v7::specs_for_convert::sut_converts_roman_to_arabic_correctly::case_1' panicked at src/v7.rs:85:9:
assertion `left == right` failed
  left: 1
  right: 0
```

### Write Enough Code to Make It Pass

You know what to do.

```rust
pub fn convert_to_arabic(roman: &str) -> usize {
    1
}
```

Next, add the test case for 2 and 3 in our test. Make it pass yourself with the dumbest code you can think of, continue writing dumb code (best book ever right?) for the third case too. Here's my dumb code.

```rust
pub fn convert_to_arabic(roman: &str) -> usize {
    if roman == "III" {
        return 3;
    }
    if roman == "II" {
        return 2;
    }
    1
}
```

Through the dumbness of real code that works we can start to see a pattern like before. We need to iterate through the input and build something, in this case a total.

```rust
pub fn convert_to_arabic(roman: &str) -> usize {
    let mut arabic = 0;
    for _ in 0..roman.len() {
        arabic += 1;
    }
    arabic
}
```

## The Ninth Requirement: Convert Roman Numerals to Arabic Numbers - the Rest

### Write the Test First

Next we move to the test case 4 which now fails because it gets 2 back as that's the length of the string.

### Write Enough Code to Make It Pass

```rust,ignore
pub fn convert_to_arabic(mut roman: &str) -> usize {
    let mut arabic = 0;
    for RomanNumeral(value, symbol) in ALL_ROMAN_NUMERALS {
        while roman.starts_with(symbol) {
            arabic += value;
            roman = roman.strip_prefix(symbol).unwrap();
        }
    }
    arabic
}
```

It is basically the algorithm of `convert_to_roman` implemented backwards. Here, we loop over the given roman numeral string:

- We look for roman numeral symbols taken from `ALL_ROMAN_NUMERALS`, highest to lowest, at the beginning of the string
- If we find the prefix, we add its value to `arabic` and trim the prefix

At the end, we return the sum as the arabic number.

The `starts_with` checks whether string starts with prefix and `strip_prefix` removes the prefix, so we can proceed with the remaining roman numeral symbols. It works with `IV` and all other test cases.

You can implement this as a recursive function, which is more elegant (in my opinion) but might be slower. I'll leave this up to you.

Now that we have our functions to convert an arabic number into a roman numeral and back, we can take our tests a step further.

## An Intro to Property-based Tests

There have been a few rules in the domain of roman numerals that we have worked with in this chapter.

- Can't have more than 3 consecutive symbols
- Only `I` (1), `X` (10) and `C` (100) can be subtractors
- Taking the result of `convert_to_roman` and passing it to `convert_to_arabic` should return us the original number

The tests we have written so far can be described as example-based tests where we provide examples for the tooling to verify.

What if we could take these rules that we know about our domain and somehow exercise them against our code?

Property based tests help you do this by throwing random data at your code and verifying the rules you describe always hold true. A lot of people think property based tests are mainly about random data but they would be mistaken. The real challenge about property based tests is having a good understanding of your domain so you can write these properties.

### Manual Implementation of Property-based Tests

Enough words, let's see some code. To generate random data, we will use [fake](https://github.com/cksac/fake-rs) crate. The crate is useful to generate random data for testing. It provides a lot of generators for basic types and much more. It is worth checking out the documentation to see what it can do.

```toml
[dev-dependencies]
fake = "4"
```

The idea is to generate a random Arabic number and then convert it to a Roman numeral. Then we will convert that Roman numeral back to an Arabic number and check that it matches the original Arabic number.

```rust,ignore
use fake::Fake;
use fake::Faker;

#[test]
fn convert_to_roman_and_inverse_is_identity() {
    let repeat = 100;

    (0..repeat).for_each(|_| {
        // Arrange
        let arabic = Faker.fake::<usize>();

        // Act
        let actual = convert_to_arabic(&convert_to_roman(arabic));

        // Assert
        let expected = arabic;
        assert_eq!(expected, actual);
    });
}
```

### Rationale of Property

Our first test will check that if we transform a number into Roman, when we use our other function to convert it back to a number that we get what we originally had.

- Given random number (e.g 4)
- Call `convert_to_roman` with random number (should return `IV` if 4)
- Take the result of above and pass it to `convert_to_arabic`
- The above should give us our original input (4)

This feels like a good test to build us confidence because it should break if there's a bug in either. The only way it could pass is if they have the same kind of bug; which isn't impossible but feels unlikely.

### Run the Test

Try running it; your computer may hang for a while, so kill it when you're bored :)

What's going on? Try adding some logs to see what is being generated.

```rust
#[test]
fn convert_to_roman_and_inverse_is_identity() {
    let repeat = 100;

    (0..repeat).for_each(|_| {
        // Arrange
        let arabic = Faker.fake::<usize>();
        println!("Testing with arabic: {}", arabic);

        // Act
        let actual = convert_to_arabic(&convert_to_roman(arabic));

        // Assert
        let expected = arabic;
        assert_eq!(expected, actual);
    });
}
```

```bash
> cargo nextest run --no-capture

running 1 test
Testing with arabic: 2092813200209769298
Testing with arabic: 6849766357708982977
Testing with arabic: -7028152357875163913
```

Just running this very simple property has exposed a flaw in our implementation. We used int as our input but:

- You can't do negative numbers with Roman Numerals
- Given our rule of a max of 3 consecutive symbols we can't represent a value greater than 3999 (well, kinda) and int has a much higher maximum value than 3999

This is great! We've been forced to think more deeply about our domain which is a real strength of property based tests. Clearly int is not a great type. What if we try to generate random numbers in a range?

```rust
#[test]
fn convert_to_roman_and_inverse_is_identity() {
    let repeat = 100;
    let max = 3999;

    (0..repeat).for_each(|_| {
        // Arrange
        let arabic = (1..max).fake::<usize>();

        // Act
        let actual = convert_to_arabic(&convert_to_roman(arabic));

        // Assert
        let expected = arabic;
        assert_eq!(expected, actual);
    });
}
```

If you run the test they now actually run and you can see what is being tested. You can run multiple times to see our code stands up well to the various values! This gives me a lot of confidence that our code is working how we want.

### Implementation of Property-based Tests with quickcheck

In Rust world, property-based tests are written using the [proptest](https://github.com/proptest-rs/proptest) or [quickcheck](https://github.com/BurntSushi/quickcheck) crates. They provide a more structured way to define properties and generate random data for testing. You can see the difference [here](https://github.com/BurntSushi/quickcheck?tab=readme-ov-file#alternative-rust-crates-for-property-testing).

We will use the `quickcheck` crate to write property-based tests in a more structured way. It helps testing using randomly generated input, and provides shrinking capabilities to reduce the input to a minimal failing case. With our Roman Numeral example, shrinking is not very useful, but it is a great feature for more complex tests.

Anyway, let's change our test to use `proptest`. You need to install first.

```toml
[dev-dependencies]
quickcheck = "1"
quickcheck_macros = "1"
```

```rust,ignore
#[derive(Clone, Debug)]
struct ArabicFixture(usize);

impl quickcheck::Arbitrary for ArabicFixture {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let arabic: usize = usize::arbitrary(g) % 3999 + 1;
        ArabicFixture(arabic)
    }
}

#[quickcheck_macros::quickcheck]
fn convert_to_roman_and_inverse_is_identity(arabic: ArabicFixture) -> bool {
    let actual = convert_to_arabic(&convert_to_roman(arabic.0));
    let expected = arabic.0;
    expected == actual
}
```

A test should be annotated with `#[quickcheck_macros::quickcheck]` to indicate that it is a property-based test. The functions' argument is a fixture that implements `quickcheck::Arbitrary` trait. quickcheck will generate a randomized value from the implementation of `quickcheck::Arbitrary` and put the value into the test.

It looks a bit more complex than the manual implementation, but if you read the [documentation](https://github.com/BurntSushi/quickcheck?tab=readme-ov-file#alternative-rust-crates-for-property-testing) once, you will see that it is not that hard to understand. The key is to implement the `quickcheck::Arbitrary` trait for your fixture type, which allows you to generate random data for your tests. All the rest is just a matter of writing the test function and annotating it with `#[quickcheck_macros::quickcheck]`.

## Wrapping Up

### More TDD Practice with Iterative Development

Did the thought of writing code that converts 1984 into `MCMLXXXIV` feel intimidating to you at first? It did to me and I've been writing software for quite a long time.

The trick, as always, is to get started with something simple and take small steps.

At no point in this process did we make any large leaps, do any huge refactorings, or get in a mess.

I can hear someone cynically saying "this is just a kata". I can't argue with that, but I still take this same approach for every project I work on. I never ship a big distributed system in my first step, I find the simplest thing the team could ship (usually a "Hello world" website) and then iterate on small bits of functionality in manageable chunks, just like how we did here.

The skill is knowing how to split work up, and that comes with practice and with some lovely TDD to help you on your way.

### Property-based Tests

- If you can think of ways to describe your domain rules in code, they are an excellent tool for giving you more confidence
- Force you to think about your domain deeply
- Potentially a nice complement to your test suite
