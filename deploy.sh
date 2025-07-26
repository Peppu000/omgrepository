#!/bin/sh -ex
target_branch="ghp_deploy"
git config --global user.name "CircleCI deployer"
git config --global user.email "<>"
git checkout -b $target_branch
git reset --hard origin/main

./target/release/omgrepository > output.txt
git add output.txt
git commit -m "Update Github Pages"
git push -f origin $target_branch