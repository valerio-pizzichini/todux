# TODUX
> Simple CLI todo manager

 Quick links:
* [How to use](#how-to-use)
* [Commands list](#commands-list)
* [Installation](#installation)

## How to use
![How_to_use](docs/todux_how_to_use.gif)

## Commands list

### Add item
````
$ todux add "Schedule new meeting for tomorrow at 10.30"

"Schedule new meeting for tomorrow at 10.30" added ✔

````

### Display list
````
$ todux list
````
Interactive keybindings during list view:
* [T] Toggle/Untoggle item
* [D] Delete item
* [Q] Quit app

### Change workspace
````
$ todux workspace set new_workspace

Workspace set to "new_workspace" ✔
````

### Unset workspace
````
$ todux workspace unset
````

### Delete workspace
````
$ todux workspace remove new_workspace

Workspace "new_workspace" removed 🗑
````

## Installation

### From source

With Rust's package manager [cargo](https://github.com/rust-lang/cargo), you can install *todux* via:
```
cargo install todux
```
Note that rust version *1.42.0* or later is required.

`make` is also needed for the build.

### From binaries

The [release page](https://github.com/sf3ris/todux/releases) includes precompiled binaries for Linux.
