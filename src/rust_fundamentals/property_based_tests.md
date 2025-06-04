# Property-based Tests

You can find all the code for this chapter [here](https://github.com/PeppyDays/learn-rust-with-tests/tree/main/examples/roman).

Some companies include the [Roman Numeral Kata](http://codingdojo.org/kata/RomanNumerals/) in their interview process. This chapter demonstrates how to tackle it using TDD and introduces property-based testing as a powerful complement to traditional example-based tests.

## The First Requirement: Convert Arabic Numbers to Roman Numerals - 1

We'll write a function that converts [Arabic numbers](https://en.wikipedia.org/wiki/Arabic_numerals) (numbers 0 to 9) to Roman Numerals. [Roman Numerals](https://en.wikipedia.org/wiki/Roman_numerals) represent how Romans wrote numbers.

Roman numerals use symbols representing values: `I` is one, `III` is three. However, there are interesting rules: `V` means five, but `IV` is 4 (not `IIII`). `MCMLXXXIV` represents 1984. This looks complex, making it hard to imagine writing code for such cases from the start.

As emphasized throughout this book, developers should identify "thin vertical slices" of useful functionality and iterate. The TDD workflow facilitates this iterative development.

Rather than starting with 1984, let's begin with 1.

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

If you've reached this point in the book, this should feel routine. That's excellent - TDD is a skill that converts anxiety into routine.

We added `#[allow(non_snake_case)]` to the test module because clippy warns that function names should be lowercase. We're using `sut_converts_1_to_I` as a test name, which isn't snake case. This attribute tells clippy to ignore this warning for this module.

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

Hard-coding the result feels strange, but with TDD we want to stay out of the red state as briefly as possible. While it seems we haven't accomplished much, we've defined our API and captured one rule in a test, even if the implementation is simplistic.

Now use that uncomfortable feeling to write a new test that forces us to write slightly more sophisticated code.

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

This still feels like we're avoiding the real problem. We need more tests to drive us forward.

### Refactor

Our tests contain repetition. When testing scenarios that follow "given input X, expect Y" patterns, parameterized tests are ideal.

The `rstest` crate provides this functionality. Add it to your `Cargo.toml` and merge the two tests into one.

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

These if statements are becoming unwieldy. Looking closely at the code, we're building a string of `I` characters based on the `arabic` value.

For more complex numbers, we'll need arithmetic and string concatenation.

Let's refactor with this in mind. This approach might not suit the final solution, but that's acceptable. We can always discard code and start fresh with our tests as guides.

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

Complexity increases here. Romans recognized that repeating characters becomes difficult to read and count. Therefore, Roman numerals forbid repeating the same character more than 3 times consecutively.

Instead, you use the next highest symbol and subtract by placing a symbol to its left. Not all symbols can be subtractors; only `I` (1), `X` (10), and `C` (100) qualify.

For example, 5 in Roman numerals is `V`. To create 4, you don't use `IIII`; instead, you use `IV`.

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

I dislike breaking our string-building pattern and want to maintain it.

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

To accommodate 4 within our current approach, I now count down from the Arabic number, adding symbols to our string as we progress. This approach may not work long-term, but let's explore it!

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

Loop repetition like this typically signals a missing abstraction. Short-circuiting loops can improve readability but may indicate deeper issues.

We're looping over our Arabic number and calling break for certain symbols, but we're essentially performing clumsy subtraction.

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

- Based on signals from our code, driven by tests of basic scenarios, building a Roman numeral requires subtracting from `arabic` as we apply symbols
- The for loop no longer relies on an `n`; instead, we continue building our string until we've subtracted enough symbols from `arabic`

## The Sixth Requirement: Convert Arabic Numbers to Roman Numerals - 9

This approach should work for 6 (`VI`), 7 (`VII`), and 8 (`VIII`) as well. Add these cases to our test suite to verify (see GitHub for complete code samples).

9 follows the same rule as 4: subtract `I` from the next number's representation. Since 10 is represented as `X` in Roman numerals, 9 becomes `IX`.

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

The code suggests another refactor opportunity, though it's not entirely clear. Let's continue.

Add a test case for 10 (should be `X`) and make it pass before proceeding.

Here are additional tests I added, confident our code works up to 39:

```rust
#[case(10, "X")]
#[case(14, "XIV")]
#[case(18, "XVIII")]
#[case(20, "XX")]
#[case(39, "XXXIX")]
```

In object-oriented programming, switch statements warrant suspicion. They often capture concepts or data in imperative code when class structures would be more appropriate.

While Rust isn't strictly object-oriented, we shouldn't ignore OO lessons entirely.

Our switch statement describes Roman numeral truths alongside behavior.

We can refactor by decoupling data from behavior.

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

This feels much better. We've declared numeral rules as data rather than hiding them in algorithms, clearly showing how we work through the Arabic number, adding symbols that fit.

Does this abstraction handle larger numbers? Extend the test suite for 50, which is `L` in Roman numerals.

Here are test cases to make pass:

```rust
#[case(40, "XL")]
#[case(47, "XLVII")]
#[case(49, "XLIX")]
#[case(50, "L")]
```

## The Seventh Requirement: Convert Arabic Numbers to Roman Numerals - the Rest

Here are the remaining symbols:

| Arabic | Roman |
| ------ | ----- |
| 100    | C     |
| 500    | D     |
| 1000   | M     |

Apply the same approach for remaining symbols by adding data to both tests and our symbol array.

Does your code work for 1984: `MCMLXXXIV`?

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

I added additional edge cases for extra confidence. Table-based tests make this very efficient.

The algorithm remained unchanged; I only updated the `ALL_ROMAN_NUMERALS` array.

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

We're not finished yet. Next, we'll write a function that converts Roman numerals back to integers.

### Write the Test First

We can reuse our test cases with minor refactoring. Move test cases outside the test as template variables using the [rstest_reuse](https://crates.io/crates/rstest_reuse) crate.

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

Note that I'm not using the test case template for `sut_converts_roman_to_arabic_correctly`. We'll add cases individually and replace with the template later.

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

Next, add test cases for 2 and 3. Make them pass with the simplest code possible, continuing with minimal implementations for the third case. Here's my simple code:

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

Through this working simplistic code, we can see a pattern emerging. We need to iterate through the input and build something - in this case, a total.

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

Next, we move to test case 4, which fails because it returns 2 (the string length).

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

This is essentially the `convert_to_roman` algorithm in reverse. We loop over the Roman numeral string:

- Look for Roman numeral symbols from `ALL_ROMAN_NUMERALS` (highest to lowest) at the string's beginning
- If we find a prefix, add its value to `arabic` and trim the prefix

Finally, we return the sum as the Arabic number.

`starts_with` checks if the string begins with a prefix, and `strip_prefix` removes it, allowing us to process remaining symbols. This works for `IV` and all other test cases.

You could implement this recursively for elegance, though it might be slower.

Now that we have functions to convert Arabic numbers to Roman numerals and back, we can advance our tests further.

## An Intro to Property-based Tests

We've worked with several Roman numeral domain rules in this chapter:

- No more than 3 consecutive symbols
- Only `I` (1), `X` (10), and `C` (100) can be subtractors
- Converting to Roman and back should return the original number

Our tests so far are example-based, where we provide specific examples for verification.

What if we could exercise these known domain rules against our code directly?

Property-based tests accomplish this by testing random data against your code, verifying that described rules always hold true. Many people think property-based tests are primarily about random data, but they're mistaken. The real challenge is understanding your domain well enough to write meaningful properties.

### Manual Implementation of Property-based Tests

Let's see some code. We'll use the [fake](https://github.com/cksac/fake-rs) crate to generate random data for testing. This crate provides many generators for basic types and more. The documentation shows its full capabilities.

```toml
[dev-dependencies]
fake = "4"
```

The concept: generate a random Arabic number, convert it to a Roman numeral, convert back to Arabic, and verify it matches the original.

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

Our first test verifies that transforming a number to Roman and back returns the original value.

- Given random number (e.g 4)
- Call `convert_to_roman` with random number (should return `IV` if 4)
- Take the result of above and pass it to `convert_to_arabic`
- The above should give us our original input (4)

This provides strong confidence because it should break if either function has bugs. It could only pass if both functions share identical bugs, which is unlikely.

### Run the Test

Try running it; your computer may hang, so kill the process when needed.

What's happening? Add logging to see what's being generated.

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

This simple property has exposed implementation flaws. Using `int` as input creates problems:

- Roman numerals don't support negative numbers
- Given the 3-consecutive-symbol rule, we can't represent values above 3999, while `int` has much higher maximum values

This is excellent! We're forced to think more deeply about our domain - a key strength of property-based tests. Clearly `int` isn't ideal. Let's try generating random numbers within a specific range.

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

Running the test now works, and you can see what's being tested. Multiple runs show our code handles various values well, giving strong confidence in our implementation.

### Implementation of Property-based Tests with quickcheck

In Rust, property-based tests use the [proptest](https://github.com/proptest-rs/proptest) or [quickcheck](https://github.com/BurntSushi/quickcheck) crates. They provide structured ways to define properties and generate random test data. See the differences [here](https://github.com/BurntSushi/quickcheck?tab=readme-ov-file#alternative-rust-crates-for-property-testing).

We'll use `quickcheck` for more structured property-based tests. It tests with randomly generated input and provides shrinking capabilities to reduce failing cases to minimal examples. While shrinking isn't particularly useful for Roman numerals, it's valuable for complex tests.

Let's change our test to use `quickcheck`. First, install it:

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

Tests should be annotated with `#[quickcheck_macros::quickcheck]` to indicate property-based testing. The function's argument is a fixture implementing the `quickcheck::Arbitrary` trait. QuickCheck generates randomized values from this implementation and injects them into tests.

This looks more complex than manual implementation, but the [documentation](https://github.com/BurntSushi/quickcheck?tab=readme-ov-file#alternative-rust-crates-for-property-testing) shows it's straightforward. The key is implementing `quickcheck::Arbitrary` for your fixture type, enabling random data generation. The rest involves writing the test function and adding the annotation.

## Wrapping Up

### More TDD Practice with Iterative Development

Did writing code to convert 1984 into `MCMLXXXIV` feel intimidating initially? It did to me, despite years of software development experience.

The trick is always starting with something simple and taking small steps.

Throughout this process, we never made large leaps, performed huge refactorings, or created messes.

Someone might cynically say "this is just a kata." While true, I apply this same approach to every project. I never ship large distributed systems initially; instead, I find the simplest deliverable (usually a "Hello world" website) and iterate on small functionality chunks in manageable pieces, exactly as we did here.

The skill lies in knowing how to split work effectively - something that comes with practice and TDD guidance.

### Property-based Tests

- If you can describe domain rules in code, they're excellent confidence-building tools
- They force deep domain thinking
- They potentially complement your test suite nicely
