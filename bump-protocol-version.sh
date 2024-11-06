#!/bin/sh
if [[ ! -z "$(git status --porcelain)" ]]; then
	echo "worktree is dirty"
	exit 1
fi

PREV=$(grep "const Version" protocol/version.go | sed 's/.*"\(\w*\)"/\1/')
NEXT=$(git rev-parse HEAD)

FILES=$(fd -e py -e go -t f)
echo $FILES | xargs sed -i "s/$PREV/$NEXT/"
echo $FILES | xargs git add
git diff
git commit -em "bump protocol version"
