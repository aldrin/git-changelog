[![Build Status](https://travis-ci.org/aldrin/git-changelog.svg?branch=master)](https://travis-ci.org/aldrin/git-changelog)
[![Build status](https://ci.appveyor.com/api/projects/status/ixcfop3nhjmx3s5v/branch/master?svg=true)](https://ci.appveyor.com/project/aldrin/git-changelog/branch/master)
[![Crates.io](https://img.shields.io/crates/v/git-changelog.svg)](https://crates.io/crates/git-changelog)
[![GitHub release](https://img.shields.io/github/release/aldrin/git-changelog.svg)](https://github.com/aldrin/git-changelog/releases)
[![codecov](https://codecov.io/gh/aldrin/git-changelog/branch/master/graph/badge.svg)](https://codecov.io/gh/aldrin/git-changelog)

`git-changelog` is a tool to generate [change logs] (a.k.a release notes) that are typically
distributed at project release milestones. Unlike other tools that do the same, this one does not
require you to follow any particular git workflow conventions. All it assumes is that you'll pick a
few keywords (or use the built-in ones) to annotate lines in your commit messages. Concretely, a
commit [like this](src/assets/sample-commit.message) generates an output [like
this](src/assets/sample.md).

When you wish to record a *user visible* change (e.g. new feature, bug fix, breaking change, etc.)
you write a normal commit message and annotate some lines in it with your chosen keywords. The
annotated lines are used at report generation time to organize changes into *categories* and
*scopes*. The organized changes are then rendered as a pretty and accurate change log. 

Commit messages without tags are quietly ignored and you are free to tag as little or as much as you
want. Here's a quick demo:

[![asciicast](https://asciinema.org/a/Jk8A5UEJGkhlalL4gl3HevC7e.png)](https://asciinema.org/a/Jk8A5UEJGkhlalL4gl3HevC7e)

## Motivation

Commit messages must always be meaningful and with a little extra effort we can automate the chore
of generating meaningful change logs for users. As I finish up work on a change, I like to pause,
consider what the change means to the end-user and reorganize the message a bit. If you follow the
(easy) conventions described below and tag lines in your commit message appropriately, this tool
will help you generate an *accurate* and *presentable* change log.

A little time spent, when the context and impact of the change is fresh in mind, saves a lot of time
at release milestones.

## Installation

The recommended way to install the tool is:

```bash
> cargo install git-changelog
```

This compiles the tool for your environment from the sources. If you just need the executables, see
[releases].

If you use a Mac with [Homebrew], you can get the latest binaries with the following:

```bash
> brew tap aldrin/tap
> brew install git-changelog
```

## Usage

Write your commits as you normally do (or [should]🙂). When it looks like a particular commit
includes a change that the "user" may be interested in, tag its lines appropriately. Concretely,
instead of writing this:

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

You don't need to tag **every** commit (`git commit -m` is perfectly fine, where you think it is).
You just need to tag the changes you want your users to know about. 

> The quality of the tool output depends on the quality of *your* input.

## Generate reports

Once on `PATH`, the tool works like a usual git sub-command (e.g. like `git log`) and takes a
[revision range] as input. It looks at all commits in the range and uses the keywords it finds in
their messages to generate the report. Simple. 🙂

```bash
$ git changelog v0.1.1..v0.2.0
```

If you don't provide a revision range, `<last-tag>..HEAD` is used. If no tags are defined, just the
last commit is picked.

```bash
$ git changelog -d
INFO: Reading file '/Users/aldrin/Code/git-changelog/.changelog.yml'
INFO: Using revision range 'v0.2.0..HEAD (15 commits)'
...
```

Note that using `-d` gives you some insight into the tool operations. 

For more complex range selections you can use `git log` arguments as shown below:

```bash
$ git changelog -- --author aldrin --reverse --since "1 month ago"
```

Note the `--` before you start the `git log` arguments.

## Customization

Each project is different and you may want to customize the tags and output to suit your
requirements. 

**Conventions**: You can define change categories and scopes tags and titles relevant to your
project. Add a [.changelog.yml] file to your repository root (or use the `--config` option).  See
the [default configuration file](src/assets/changelog.yml) for a starting example.

**Templates**: You can specify your own [Handlebars] template if the output doesn't work for
you. Add a `.changelog.hbs` to your repository root or use the `--template` command line option. See
the [default template](src/assets/changelog.hbs) for a starting example and the [library
documentation] for details on the input data-structure.

**JSON**: You can skip Markdown completely and ask for a JSON output with the `--json` flag.

**Post Processors**: You can add line post-processors to tweak the output. I use these to simplify
adding links to bug-tracking systems. For example, the commit message can simply state the ticket
number:

```
Fixes: JIRA-1234
```

Then, with a post-processor like the following in the configuration file:

```yml
output:
  post_processors:
    - {lookup: "JIRA-(?P<id>\\d+)", replace: "[JIRA-$id](https://jira.company.com/view/JIRA-$id)"}
```

the tool replaces it with:

```
Fixes: [JIRA-1234](https://jira.company.com/view/JIRA-1234)
```

[should]:https://chris.beams.io/posts/git-commit/
[library documentation]: https://docs.rs/git-changelog/0.3.1/changelog/struct.ChangeLog.html
[change logs]: http://keepachangelog.com/
[revision range]: https://git-scm.com/book/en/v2/Git-Tools-Revision-Selection#_commit_ranges
[Handlebars]: http://handlebarsjs.com/
[Homebrew]: https://brew.sh/
[CHANGELOG.md]: CHANGELOG.md
[.changelog.yml]: .changelog.yml
[releases]:https://github.com/aldrin/git-changelog/releases
