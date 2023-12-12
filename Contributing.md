# Contributing

If you want to contribute to the project pick an issue with [open for contribution](https://github.com/nmeylan/rust-ro/issues?q=is%3Aopen+is%3Aissue+label%3A%22Open+for+contribution%22) label

If there is no issue created with this label and you want to contribute on something, open an issue and detail what you plan to implement and we will discuss about it.

## Design required
For now a short design will be required for any contribution. In this design describe how you plan to implement the feature.

The reason for this is simple: currently the architecture of the project is not fully defined and i would like to control how thing are implemented. When the basis would be put in place, the requirement for design will be removed or atleast not required for every feature

# Pull request content
- Any new feature should be covered by tests following the principle present in existing tests.
- Newly added code should follow principle already in place in the code base (code structure, event loop, etc...)
- Avoid reformatting code not related with your change
- Follow indentation in place
- `cargo test` should be passing on the pull request
