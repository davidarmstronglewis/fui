# Changelog

## Next2

* In general: redesigned `Field` with its consequenses.

<details><summary>Click here for ugly details</summary>

* `Form` changes:
    * Remove
        * `pub fn clap_arg_matches2value(&self, arg_matches: &ArgMatches) -> Value`
            * `Fui` itself handles this feature
        * `fn show_errors(&mut self, form_errors: &FormErrors)`
            * now each Field sets it implicity during validation

    * Add:
        * `FormData` type
        * `pub fn for_each` which calls function on each field.
        * `pub fn for_each_mut` which calls function on each field (mutable variant).

* `FormField` changes:
    * `FormField` must also implements `Cursive::View`
    * Remove:
        * `fn build_widget(&self) -> Box<AnyView>;`
        * `fn get_widget_manager(&self) -> &WidgetManager;`
        * `fn clap_arg(&self) -> Arg;`
        * TODO::: rm it for sure
        * `fn clap_args2str(&self, args: &ArgMatches) -> String;`
    * Replace:
        * `fn validate(&self, data: &str) -> Result<Value, String>;`
        * with:
        * `fn validate(&mut self) -> Result<Value, FieldErrors>;`
    * Add:
        * `fn set_value(&mut self, value: &Value);`
        * TODO::: rm it 
        * `fn as_clap_arg(&self) -> Arg;`

* `Field` changes:
    * Remove `label_with_help_layout` (`Field` itself handles that)
    * Replace:
        * `pub struct Field<W: WidgetManager, T>`
        * with
        * `pub struct Field`
    * Replace
        * `pub fn new<IS: Into<String>>(label: IS, widget_manager: W, initial: T) -> Self`
        * with
        * `pub fn new<VM: WidgetManager + 'static, IS: Into<String>>(label: IS, mut widget_manager: VM) -> Field`
    * Replace
        * `pub fn initial(self, value: bool) -> Self`
        * with
        * `pub fn initial<IS: Into<Value>>(self, initial: IS) -> Self`
    * Replace
        * `pub fn help<IS: Into<String>>(self, msg: IS) -> Self`
        * with
        * `pub fn help(self, msg: &str) -> Self`
    * Add:
        * `pub fn set_help(&mut self, msg: &str)`
        * `pub fn set_error(&mut self, msg: &str)`
    * Add `FieldErrors` type
    * Implement `FormField` for `Field`
    * Implement `ViewWrapper` for `Field`

    * `Checkbox` changes:
        * Replace:
            * `pub fn new<IS: Into<String>>(label: IS) -> Field<CheckboxManager, bool>`
            * with
            * `pub fn new<IS: Into<String>>(label: IS) -> Field`
    * `Text` changes:
        * Replace:
            * `pub fn new<IS: Into<String>>(label: IS) -> Field<TextManager, String>`
            * with
            * `pub fn new<IS: Into<String>>(label: IS) -> Field`
    * `Autocomplete` changes:
        * Replace:
            * `pub fn new<IS: Into<String>, F: Feeder>(label: IS, feeder: F) -> Field<AutocompleteManager, String>`
            * with
            * `pub fn new<IS: Into<String>, F: Feeder>(label: IS, feeder: F) -> Field`
    * `Multiselect` changes:
        * Replace:
            * `pub fn new<IS: Into<String>, F: Feeder>(label: IS, feeder: F) -> Field<MultiselectManager, Vec<String>>`
            * with
            * `pub fn new<IS: Into<String>, F: Feeder>(label: IS, feeder: F) -> Field`

* `WidgetManager` changes:
    * Remove:
        * `fn build_widget(&self, label: &str, help: &str, initial: &str) -> Box<AnyView>;`
        * `fn set_error(&self, view: &mut AnyView, error: &str);`
        * `fn build_value_view(&self, value: &str) -> Box<AnyView>;`

    * Replace:
        * `fn get_value(&self, view: &AnyView) -> String;`
        * with
        * `fn as_string(&self, view_box: &ViewBox) -> String;`

    * Add:
        * `fn take_view(&mut self) -> ViewBox;`
        * `fn set_value(&self, view_box: &mut ViewBox, value: &Value);`
        * `fn as_value(&self, view_box: &ViewBox) -> Value;`

* `Views` changes:
    * Add `set_value` to `Autocomplete`

</details>


## Next

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
