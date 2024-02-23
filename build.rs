fn main() {
    glib_build_tools::compile_resources(
        &["src/resources"],
        "csrc/resources/resources.gresource.xml",
        "todores.gresource",
    );
}

// todo
    glib_build_tools::compile_resources(
        &["todo/1/resources"],
        "todo/1/resources/resources.gresource.xml",
        "todo_1.gresource",
    );