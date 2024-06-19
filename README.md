# dollars
A simple dollar value in Rust

# Overview
If you need a simple representation of a dollar value backed by a single integer, consider `dollars`. It doesn't do much, and it doesn't need to.

To get a `Dollars` value, you can:
* Construct it directly from an integer cent value
* Parse it from a string
    * With or without a `+`/`-` sign
    * With or without a `$` in front
    * With or without a cents portion

Given one, you can:
* Inspect its component dollar and cent parts
* Retrieve its value in cents
* Do basic arithmetic

