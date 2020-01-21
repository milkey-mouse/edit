# `edit`

`edit` is a Rust library that lets you open and edit something in a text editor, regardless of platform. (Think `git commit`.)

It works on Windows, Mac, and Linux, and knows about lots of different text editors to fall back upon in case standard environment variables such as `VISUAL` and `EDITOR` aren't set.

    let template = "Fill in the blank: Hello, _____!";
    let edited = edit::edit(template)?;
    println!("after editing: '{}'", edited);
    // after editing: 'Fill in the blank: Hello, world!'
