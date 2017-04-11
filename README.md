Code samples for the [Borrow Checker presentation](https://fbecart.github.io/Presentations/borrow-checker.html#/).

Each code sample is a `/examples/*.rs` file.

# Getting started

    # Install rustup
    curl https://sh.rustup.rs -sSf | sh
    source $HOME/.cargo/env
    
    # Clone the repository
    git clone git@github.com:fbecart/borrow-checker-examples.git
    
    # Test your setup
    cd borrow-checker-examples
    cargo run

# Run an example

    # Run /examples/close-file.rs
    cargo run --example close-file
