fn main() {
    let string = format!(
        "Hello world!\nYou are editing this file in '{}'.",
        edit::get_editor()
            .expect("can't find an editor")
            .to_str()
            .unwrap()
    );
    let edited = edit::edit(string).expect("editing failed");
    println!("after editing:\n{}", edited);
}
