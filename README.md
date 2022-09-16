# TimeSpent

<img src="https://i.imgur.com/6YpjZ0K.png" height="650">

**A simple GUI rust application that keeps track of how much time you spend on each application.**

## Usage
The Daemon binary tracks the amount of time you used an application. Make sure it's running in the background.

In the Gui, You can right click the names of applications to open a Context menu.

## Build from Source
Install [Rust](https://www.rust-lang.org/tools/install) if you don't have it already

```
git clone https://github.com/slackedlime/TimeSpent.git
cd TimeSpent/
cargo build --bin daemon --release
cargo build --bin gui --release
```

You should find you executables in the /target/release/ directory

Be sure to make the daemon binary a start up application.

## Todo
- Add Groups Feature.
- Add a bargraph to Status menu.
- Fix Context Menu going out of screen.
