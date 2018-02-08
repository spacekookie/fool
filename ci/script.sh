# This script takes care of testing your crate

set -ex

# TODO This is the "test phase", tweak it as you see fit
main() {
    cargo build # --target $TARGET
    cargo build # --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cargo test # --target $TARGET
    cargo test # --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG] && [ "$TARGET" != "x86_64-apple-darwin" ]; then
    main
fi
