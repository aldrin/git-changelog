[![Build Status]](https://travis-ci.org/aldrin/git-changelog)

`git-changelog` is a tool to automate repository [changelog] generation.

A commit [like this](resources/sample-commit.message) generates an output [like
this](resources/sample.md).

## Motivation

Commit messages must always be meaningful and with a little extra effort we can automate the chore
of generating meaningful change logs for our users. As I finish up work on a change to a repository,
I like to pause a while, consider what the change means to the end-user and reorganize the message a
bit. If you follow the (easy) conventions described below and tag lines in your commit message
appropriately, this tool will help you generate an *accurate* and *presentable* change log.

A little time spent at commit, when the context and impact of the change is fresh in mind, saves a
lot of time at release milestones.

## Installation

```bash
> cargo install git-changelog
```

If you use a Mac with [Homebrew], you may prefer the following:

```bash
> brew tap aldrin/tap
> brew install git-changelog
```

## Usage

Just write your commits as you [normally] do. When it looks like a particular commit includes a
change that the "user" may be interested in, tag its lines appropriately.

Concretely, instead of writing this:

```
Add support for filtering responses

UI gets a bit cluttered when the response contains too many
items. Added a simple filtering scheme to reduce the result
set to a more relevant subset. Clients using v1.2 need to
upgrade to accomodate the new request parameter.
```

Write this:

```
Add support for filtering responses

- feature: UI gets a bit cluttered when the response
  contains too many items. Added a simple filtering scheme
  to reduce the result set to a more relevant subset.

- break: Clients using v1.2 need to upgrade to accommodate
  the new request parameter.
```

The two commit messages are almost the same but the latter tags *user visible* changes a bit more
diligently. Eventually, this diligence helps the tool to identify lines, aggregate similar things
(e.g. breaking changes) across commits, order them, and give you a report that you can share, as is,
with users. Or, you can use the output as the starting draft, make editorial changes and then share
it with users. Either way, it saves you some time.

Of course, you don't need to tag **every** commit (`git commit -m` is perfectly fine, where you
think it is). You just need to tag the changes you want your users to know about. The quality of the
tool output depends on the quality of *your* input.

## Generate reports

When `git-changelog` is on the path, `git changelog` works just like `git log` and takes similar
arguments. Concretely, it takes a [commit range] and looks for all commits in that range and uses
the tags it finds in their messages to generate a report. Simple. :)

```
> git changelog -h
git-changelog (v0.2.0)
Aldrin J D'Souza <mail@aldrin.co>
A tool to automate project changelog generation.

USAGE:
    git-changelog [FLAGS] [OPTIONS] [revision-range]...

FLAGS:
    -d, --debug      Prints debug logs
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>    Configuration file

ARGS:
    <revision-range>...    The revision range, defaults to HEAD...<last-tag>
```

The default revision range picks all commits made since the *last* tag.

## Customization

*Tags*: You can specify a configuration file to define the tags and scopes you want to use for your
project.  See the [default configuration file](resources/config.yml) for a starting example.

*Template*: You can specify a [Handlebars] template to control the rendered report format. The
[default template](resources/report.handlebars) generates a Markdown document that renders well on
Github.

[normally]:https://chris.beams.io/posts/git-commit/
[changelog]: http://keepachangelog.com/
[Build Status]: https://travis-ci.org/aldrin/git-changelog.svg?branch=master
[commit range]: https://git-scm.com/book/en/v2/Git-Tools-Revision-Selection#_commit_ranges
[Handlebars]: http://handlebarsjs.com/
[Homebrew]: https://brew.sh/
