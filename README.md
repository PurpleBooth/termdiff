# termdiff

Diff a string for presentation to a user in the terminal.

## Usage

``` rust
use termdiff::{signs_theme, DrawDiff};
let old = "Double, double toil and trouble;
Fire burn and
Caldron bubble.";
let new = "Double, double toil and trouble;
Fire burn and
caldron bubble.
Cool it with a baboon's blood,
Then the charm is firm and good.";
let actual = format!("{}", DrawDiff::new(old, new, signs_theme()));

assert_eq!(
    actual,
    "--- remove | insert +++
 Double, double toil and trouble;
 Fire burn and
-Caldron bubble.
+caldron bubble.
+Cool it with a baboon's blood,
+Then the charm is firm and good.
"
);
```

Alternatively you can use this interface

``` rust
use termdiff::{arrows_theme, diff};
let old = "Double, double toil and trouble;
Fire burn and
Caldron bubble.";
let new = "Double, double toil and trouble;
Fire burn and
caldron bubble.
Cool it with a baboon's blood,
Then the charm is firm and good.";
let mut buffer: Vec<u8> = Vec::new();
diff(&mut buffer, old, new, arrows_theme()).unwrap();
let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");

assert_eq!(
    actual,
    "< left / > right
 Double, double toil and trouble;
 Fire burn and
<Caldron bubble.
>caldron bubble.
>Cool it with a baboon's blood,
>Then the charm is firm and good.
"
);
```

Read more at [Docs.rs](https://docs.rs/termdiff/)

## Themes

We have a limited number of built in themes

### Arrows

![Demo of the arrows
format](./demo_arrows.png "Demo of the arrows format")

### Signs

![Demo of the signs format](./demo_signs.png "Demo of the signs format")
