[![Build Status](https://travis-ci.org/aldrin/git-changelog.svg?branch=master)](https://travis-ci.org/aldrin/git-changelog)

`git-changelog`

`git-changelog` is a tool to automate repository changelog generation without enforcing any git
workflow conventions. When developers wish to record a "user visible" change to the repository
(e.g. new feature, bug fix, breaking change, etc.) they can tag lines in their commit message with a
few keywords. These tags are then used to organize changes made to the repository into *scopes* and
*categories* and these organized changes can then be presented as pretty change-logs or release
notes. Commits messages without tags are quietly ignored and developers are free to tag as little or
as much as they want.

A commit [like this](sample-commit.message) generates an output [like this](sample.md)

