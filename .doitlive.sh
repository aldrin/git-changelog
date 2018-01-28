#doitlive commentecho: true

#You use "git log" to walk through the git history, for example
#to see commits between release v0.1.1 and v0.2.1 of this repository
git log v0.1.1..v0.2.1
#
#To get a categorized change-log for the same revision range replace 'log' with 'changelog'
git changelog v0.1.1..v0.2.1
#
#The output is Markdown that renders nicely in a CHANGELOG.md on Github.
#If you don't like Markdown, you can ask for JSON and render as you see best
git changelog v0.1.1..v0.2.1 -j | jq '.scopes[] | .categories [] | select(.title | contains("Breaking")) | .'
clear
#That's the gist. :)
git changelog -h
#See the README for other details




