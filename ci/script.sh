# This script takes care of testing your crate

set -ex

TARGET="x86_64-unknown-linux-gnu"

# TODO This is the "test phase", tweak it as you see fit
main() {
    cargo build --target $TARGET
    cargo build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cargo test --target $TARGET
    cargo test --target $TARGET --release

    cargo run --target $TARGET
    cargo run --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
