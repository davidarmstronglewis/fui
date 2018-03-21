# Changelog

## Changed

* `Fui` takes optionally 4 attributes: name, version, description, authors
* `FormView` has an option of setting title
* `Fui` automatically adds `CLI` feature
    * `Fui::action`'s now takes 4 arguments (previous `desc` is splited to `name` & `help`) - Breaking change
* Autocomplete/Multiselect:
    * scrolling beyond visible items load rest of them
    * keys up/down updates selected value in view
