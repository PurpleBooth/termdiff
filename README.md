# termdiff

Diff a string for presentation to a user in the terminal.

## Usage

``` rust
use termdiff::{signs_theme, DrawDiff};
let old = "The quick brown fox and\njumps over the sleepy dog";
let new = "The quick red fox and\njumps over the lazy dog";
let theme = signs_theme();
let actual = format!("{}", DrawDiff::new(old, new, &theme));

assert_eq!(
    actual,
    "--- remove | insert +++
-The quick brown fox and
-jumps over the sleepy dog
+The quick red fox and
+jumps over the lazy dog
"
);
```

Alternatively you can use this interface

``` rust
use termdiff::{arrows_theme, diff};
let old = "The quick brown fox and\njumps over the sleepy dog";
let new = "The quick red fox and\njumps over the lazy dog";
let theme = arrows_theme();
let mut buffer: Vec<u8> = Vec::new();
diff(&mut buffer, old, new, &theme).unwrap();
let actual: String = String::from_utf8(buffer).expect("Not valid UTF-8");

assert_eq!(
    actual,
    "< left / > right
<The quick brown fox and
<jumps over the sleepy dog
>The quick red fox and
>jumps over the lazy dog
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

### Custom

``` rust
use termdiff::DrawDiff;
use termdiff::Theme;
use crossterm::style::Stylize;

let my_theme = Theme {
    header: format!("{}\n", "Header"),
    highlight_insert: crossterm::style::Stylize::stylize,
    highlight_delete: crossterm::style::Stylize::stylize,
    equal_prefix: "=".to_string(),
    equal_content: crossterm::style::Stylize::stylize,
    delete_prefix: "!".to_string(),
    delete_content: crossterm::style::Stylize::stylize,
    insert_prefix: "|".to_string(),
    insert_line: crossterm::style::Stylize::stylize,
    line_end: "\n".into(),
};

let old = "The quick brown fox and\njumps over the sleepy dog";
let new = "The quick red fox and\njumps over the lazy dog";
let actual = format!("{}", DrawDiff::new(old, new, &my_theme));

assert_eq!(
    actual,
    "Header
!The quick brown fox and
!jumps over the sleepy dog
|The quick red fox and
|jumps over the lazy dog
"
);
```
