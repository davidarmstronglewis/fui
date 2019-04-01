# Changelog

## next

* Add `fui::skip_single_action`
* Add `fui::skip_empty_form`


## 0.9.0

* Update `cursive` to 0.9
* Add types `FieldErrors`, `FormErrors`
* Add required parameter `program_name` to `Fui::new()` - Breaking change
* Ensure `Fui` action names are unique or panic
* `Form` can be dumped to `CLI` command by `ctrl+k`
* Cancel `Fui`'s form shows back action picker
* Update to new `Cursive` `API` - Breaking change
    * Replace all `Box<AnyView>` with `ViewBox`
* First item selection improved in `Autocomplete`
* Fix empty list scrolling in view `Autocomplete`
* Improved `Autocomplete` focus change between `edit` and `select`
* `Autoselect`'s highlight color changed to light black
* `Fui` theme is now configurable (through `Fui::theme` setter)
* Add shell completion example
* Make `Fui::build_cli_app` public for generating shell completion


## 0.8.0

* `Fui` takes optionally 4 attributes: name, version, description, authors
* `FormView` has an option of setting title
* `Fui` automatically adds `CLI` feature
    * `Fui::action`'s now takes 4 arguments (previous `desc` is splited to `name` & `help`) - Breaking change
* Autocomplete/Multiselect:
    * scrolling beyond visible items load rest of them
    * keys up/down updates selected value in view
