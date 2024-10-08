# Inlet

<p align="center">
    <img src="./documentation/images/LogoInlet.png" style="width: 200px; max-width: 100%"/>
</p>

## Introduction
Inlet is a simple language that implements capability safety by tagging every value with an origin. It doesn't compile or run though - only semantic analysis is performed.

## Important Notes

* Right now, it doesn't make sure you actually have return statements at the end of your functions! That's coming up shortly. For the time being, just don't forget to add a return statement at the end of each function. Otherwise, you may get false positives.

## Example
```
fn bar(x: {boo::baz}) -> {boo::baz} {
    return x;
}
```

The function `baz` must take a value claimed (created or modified by) the `boo` crate in the `baz` module. Any other call will result in a compiler error. It returns a value that is also claimed by `bar::baz`, and since `x` is the only value passed in, we know it must be returned untouched! This makes the function the **identity function**.

## Frequently Asked Questions
### Why is it called Inlet?
Because crabs probably live in Inlets, and Inlet is based on Rust, which has a crab as its mascot. Wasn't it obvious?