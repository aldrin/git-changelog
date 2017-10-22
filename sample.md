## CHANGELOG (2017-11-24)

### Summary

-  Demonstrate tagging conventions for commit messages


#### Features

-  **Categorized Changes:** Lines that begin with a `- <tag>:`
      can be used to categorize changes. Everything that follow a the prefix
      is tagged with the chosen category. For example, this text describes
      the tool feature that categorizes changes. Among others, `feature` is
      an out-of-the-box tag. You can define tags that make sense for your
      project.
    

-  **Scoped Changes**: For larger projects, you can introduce
    another level of organization using *scopes* (e.g. `API`,
    `Documentation`, `Benchmarks`, etc.) To specify a scope, use the `-
    tag(scope):` prefix as done by the next item. Lines that don't specify a
    `scope` like this or the previous item fall into the `default` scope. As
    with tags, you can define scopes that make sense for your project (or
    not use them at all).
    


#### Notes

- This commit message is long to demonstrate the tool features. Follow the
    usual [guidance](https://chris.beams.io/posts/git-commit/) on writing
    commit messages for *real* commits. The only additional (optional)
    conventions this tool introduces are tagging prefixes described here.
    




### API
#### Breaking Changes

-  This line and all that follow will show up under a
    "Breaking Changes" section in the "API" scope of the project. Once you
    tag a line, all lines that follow use the same categorization. You're
    free to write whatever you like.  Markdown is fine too. For example, for
    a breaking change, I can itemize things like:
    
     - **Why** I broke the API.
     - **What** you should do about it.
    
    Or even put code snippets like this:
    
    ```rust
    /// A single line in the change message
    pub struct Line {
        /// The scope
        pub scope: Option<String>,
        /// The category
        pub category: Option<String>,
        /// The text
        pub text: Option<String>,
    }
    ```
