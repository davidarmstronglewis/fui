# Changelog

## Next

* Improved selecting first item in `Autocomplete`
* Fix scrolling view `Autocomplete` when list empty
* Improved focus change between `edit` and `select` in `Autocomplete`
* `Autoselect` highlight color changed to light black
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
