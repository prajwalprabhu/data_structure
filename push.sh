#!/bin/bash

git add -A &&
git commit -a &&
git checkout main &&
git merge test &&
git push origin &&
git checkout test 
 