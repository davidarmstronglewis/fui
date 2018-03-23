# Fui Examples

Here are example programs using `Fui` to help you getting familiar with the
various aspects of the library.

To run an example, use `cargo run --example EXAMPLE_NAME`.

## App examples

These examples shows the most common cases for the library.
Which is creating a action-form interface for a CLI program.

Artificial examples showing that a program can be run in two modes 

* [`app_basic`](app_basic.rs) runs action once and exitS
* [`app_basic_looped`](app_basic_looped.rs) runs actions in a loop until user press *ctrl+c*

Paritally copied interfaces of real world programs: `ln` and `tar`

* [`app_ln_like`](app_ln_like.rs) parital copy of `ln`
* [`app_tar_like`](app_tar_like.rs) parital copy of `tar`


## Form example

`Form` is container for `Fields`. `Form` also includes two events:

* on_submit
* on_cancel

These examples show more details about `Form` and available `Fields`.

* [`form`](form.rs) shows `Form` with available `Fields`

## Field examples

`Fields` stores user input. `Fields` are building blocks for `Forms`.
These examples show more details about `Fields`.

* [`field_checkbox`](field_checkbox.rs) covers switches like `--debug`,  `--verbose`, etc.
* [`field_text`](field_text.rs) covers free text like inputs
* [`field_autocomplete`](field_autocomplete.rs) covers single input which require assistance, like paths, selections, etc.
* [`field_multiselect`](field_multiselect.rs) covers multiple input which require assistance, like paths, selections, etc.


## View examples

`Views` handle user interaction. `Views` are building blocks for `Fields`.
These examples show more details about `Views`.

* [`view_autocomplete`](view_autocomplete.rs) 
* [`view_multiselect`](view_multiselect.rs)

More `Views` are already defined in `fui` dependency crate [Cursive](http://docs.rs/cursive)



## Misc examples

* [`feeders`](feeders.rs) building block for `Fields` with completition, like `Autocomplete` and `Multiselect`

* shell completions (based on [clap example](https://kbknapp.github.io/clap-rs/clap/struct.App.html#method.gen_completions))

    Generating:

    ```bash
    cd examples/shell_completions
    cargo build
    cat clap_completition.bash
    ```
