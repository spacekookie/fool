#!/bin/bash

TARGET=x86_64-apple-darwin
CRATE_NAME=fool

# Only run when doing a Mac build
if [ "$TARGET" == "x86_64-apple-darwin" ]; then

    # Add the github public key to avoid warnings
    GITHUB_PUBKEY="github.com ssh-rsa AAAAB3NzaC1yc2EAAAABIwAAAQEAq2A7hRGmdnm9tUDbO9IDSwBK6TbQa+PXYPCPy6rbTrTtw7PHkccKrpp0yVhp5HdEIcKr6pLlVDBfOLX9QUsyCOV0wzfjIJNlGEYsdlLJizHhbn2mUjvSAHQqZETYP81eFzLQNnPHt4EVVUh7VfDESU84KezmD5QlWpXLmvU31/yMf+Se8xhHTvKSCZIFImWwoG6mbUoWf9nzpIoaSjB+weqqUUmpaaasXVal72J+UX2B+2RPW3RcT0eOzQgqlJL3RKrTJvdsjE3JEAvGq3lGHSZXy28G3skua2SmVi/w4yCE6gbODqnTWlg7+wC604ydGXA8VJiS5ap43JXiUFFAaQ=="
    echo $GITHUB_PUBKEY > ~/.ssh/known_hosts

    # Copy the private key
    cp homebrew.priv ~/.ssh/id_rsa
    chmod 400 ~/.ssh/id_rsa

    # Clone the stuff
    git clone git@github.com:spacekookie/homebrew-kookie.git

    sed -i -e 's/url.*/url \"https:\/\/github.com\/spacekookie\/fool\/releases\/download\/'"$TRAVIS_TAG"'\/'"$CRATE_NAME"'-'"$TRAVIS_TAG"'-'"$TARGET"'.tar.gz\"/g' homebrew-kookie/fool.rb
    PACKAGESHASUM=`shasum -a 256 "$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz" | sed 's/ .*//'`
    sed -i -e 's/sha256.*/sha256 \"'"$PACKAGESHASUM"'\"/g' homebrew-kookie/fool.rb

    cd homebrew-kookie
    git config --global user.email "ci@spacekookie.de"
    git config --global user.name "Kookie CI"
    git add fool.rb
    git commit -m "Bumping version to $TRAVIS_TAG"
    git push
fi
