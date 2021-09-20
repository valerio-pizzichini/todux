pub struct KeyBinding {
    pub key: String,
    pub description: String
}

pub fn get_key_bindings() -> Vec<KeyBinding> {
    let mut key_bindings = vec![];

    let toggle = KeyBinding {
        key: String::from("T"),
        description: String::from("Toggle done/undone")
    };
    let delete = KeyBinding {
        key: String::from("D"),
        description: String::from("Delete")
    };
    let quit = KeyBinding {
        key: String::from("Q"),
        description: String::from("Quit")
    };

    key_bindings.push(toggle);
    key_bindings.push(delete);
    key_bindings.push(quit);

    return key_bindings;
}

