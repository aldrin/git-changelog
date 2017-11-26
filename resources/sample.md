### CHANGELOG (2017-11-25)

#### Summary

- [#4] Demonstrate tagging conventions for commit messages


##### Features

-  *Categorized Changes:* Lines that begin with a `- <tag>:` can
      be used to categorize changes. Everything that follows the prefix is
      tagged with the chosen category. For example, this text describes the
      tool feature that categorizes changes. Among others, `feature` is an
      out-of-the-box tag defined in the [configuration](config.yml). You can
      define tags that make sense for your project.
    

-  *Scoped Changes:* For larger projects, the tool supports
    another level of organization using *scopes*. For example, projects can
    organize API changes under a `API` scope. To specify a scope, use the
    *`- tag(scope):`* prefix. When you don't specify a scope the item goes
    into the `default` scope. As with tags, you can define scopes that make
    sense for your project like `docs`, `benchmark`, etc. You can also
    ignore them entirely and just use simple category tags.
    


##### Notes

- 
    This commit message is long to demonstrate the tool features. *Real*
    commits should follow the standard
    [guidance](https://chris.beams.io/posts/git-commit/) on writing commit
    messages. The only additional (and optional) conventions this tool
    introduces are the tagging prefixes described here.
    




#### API
##### Breaking Changes

-  This item is an example of a scoped *and* categorized
    change. The text here and all that follows will show up under a
    "Breaking Changes" section in the "API" scope of the project. You can
    use this to describe the change as you wish. For example, for a breaking
    change, you may want to itemize things like:
    
     - **Why** the breaking change was necessary.
     - **What** the users should do about it.
    
    You can even put code snippets like this:
    
    ```rust
    /// A single line in the change message
    #[derive(Default, Serialize)]
    pub struct Line {
        /// The scope
        pub scope: Option<String>,
        /// The category
        pub category: Option<String>,
        /// The text
        pub text: Option<String>,
    }
    ```
    
    All text that follows a tagged line is implicitly categorized under the
    currently active tags.
