#![allow(dead_code)]
#![allow(unused_imports)]

use libc::{ c_int, c_void, c_char };
use ll::{ WINDOW, c_bool, chtype };

pub type FORM = *mut i8;
pub type FIELD = *mut i8;
pub type FIELDTYPE = *mut i8;
pub type FieldOptions = c_int;

#[cfg(feature="form")] #[link(name="form")]
extern {
    pub fn set_current_field(_:FORM, _:FIELD) -> c_int;
    pub fn current_field(_:FORM) -> FIELD;
    pub fn unfocus_current_field(_:FORM) -> c_int;
    pub fn set_form_page(_:FORM, _:c_int) -> c_int;
    pub fn form_page(_:FORM) -> c_int;
    pub fn field_index(_:FIELD) -> c_int;

    pub fn data_ahead(_:FORM) -> c_int;
    pub fn data_behind(_:FORM) -> c_int;

    pub fn new_field(_:c_int, _:c_int, _:c_int, _:c_int, _:c_int, _:c_int) -> FIELD;
    pub fn dup_field(_:FIELD, _:c_int, _:c_int) -> FIELD;
    pub fn link_field(_:FIELD, _:c_int, _:c_int) -> FIELD;
    pub fn free_field(_:FIELD) -> c_int;

    pub fn field_info(_:FIELD, _:*mut c_int, _:*mut c_int, _:*mut c_int, _:*mut c_int, _:*mut c_int, _:*mut c_int) -> c_int;
    pub fn dynamic_field_info(_:FIELD, _:*mut c_int, _:*mut c_int, _:*mut c_int) -> c_int;

    // pub fn int set_field_type(FIELD *field, FIELDTYPE *type, ...); TODO
    pub fn field_type(_:FIELD) -> FIELDTYPE;
    // pub fn void *field_arg(const FIELD *field); TODO

    pub fn set_field_buffer(_:FIELD, _:c_int, _:*const c_char) -> c_int;
    pub fn field_buffer(_:FIELD, _:c_int) -> *const c_char;
    pub fn set_field_status(_:FIELD, _:c_bool) -> c_int;
    pub fn field_status(_:FIELD) -> c_bool;
    pub fn set_max_field(_:FIELD, _:c_int) -> c_int;

    pub fn set_form_fields(_:FORM, _:*mut FIELD) -> c_int;
    pub fn form_fields(_:FORM) -> *mut FIELD;
    pub fn field_count(_:FORM) -> c_int;
    pub fn move_field(_:FIELD, _:c_int, _:c_int) -> c_int;

    pub fn set_field_fore(_:FIELD, _:chtype) -> c_int;
    pub fn field_fore(_:FIELD) -> chtype;
    pub fn set_field_back(_:FIELD, _:chtype) -> c_int;
    pub fn field_back(_:FIELD) -> chtype;
    pub fn set_field_pad(_:FIELD, _:c_int) -> c_int;
    pub fn field_pad(_:FIELD) -> c_int;

    // TODO
    // pub fn int set_field_init(FORM *form, Form_Hook func);
    // pub fn Form_Hook field_init(const FORM *form);
    // pub fn int set_field_term(FORM *form, Form_Hook func);
    // pub fn Form_Hook field_term(const FORM *form);
    // pub fn int set_form_init(FORM *form, Form_Hook func);
    // pub fn Form_Hook form_init(const FORM *form);
    // pub fn int set_form_term(FORM *form, Form_Hook func);
    // pub fn Form_Hook form_term(const FORM *form);

    pub fn set_field_just(_:FIELD, _:c_int) -> c_int;
    pub fn field_just(_:FIELD) -> c_int;

    pub fn set_field_opts(_:FIELD, _:FieldOptions) -> c_int;
    pub fn field_opts_on(_:FIELD, _:FieldOptions) -> c_int;
    pub fn field_opts_off(_:FIELD, _:FieldOptions) -> c_int;
    pub fn field_opts(_:FIELD) -> FieldOptions;

    pub fn form_driver(_:FORM, _:c_int) -> c_int;
    // pub fn form_driver_w(_:FORM, _:c_int, wchar_t wch) -> c_int; TODO wchar ?

    pub fn set_form_opts(_:FORM, _:FieldOptions) -> c_int;
    pub fn form_opts_on(_:FORM, _:FieldOptions) -> c_int;
    pub fn form_opts_off(_:FORM, _:FieldOptions) -> c_int;
    pub fn form_opts(_:FORM) -> FieldOptions;

    pub fn form_request_name(_:c_int) -> *const c_char;
    pub fn form_request_by_name(_:*const c_char) -> c_int;

    pub fn set_form_win(_:FORM, _:WINDOW) -> c_int;
    pub fn form_win(_:FORM) -> WINDOW;
    pub fn set_form_sub(_:FORM, _:WINDOW) -> c_int;
    pub fn form_sub(_:FORM) -> WINDOW;
    pub fn scale_form(_:FORM, _:*mut c_int, _:*mut c_int) -> c_int;

    // TODO
    // FIELDTYPE *new_fieldtype(bool (* const field_check)(FIELD *, const void *), bool (* const char_check)(int, const void *));
    // int free_fieldtype(FIELDTYPE *fieldtype);
    // int set_fieldtype_arg(FIELDTYPE *fieldtype, void *(* const make_arg)(va_list *),
    //     void *(* const copy_arg)(const void *),
    //     void  (* const free_arg)(void *));
    // int set_fieldtype_choice(
    //     FIELDTYPE *fieldtype,
    //     bool (* const next_choice)(FIELD *, const void *),
    //     bool (* const prev_choice)(FIELD *, const void *));
    // FIELDTYPE *link_fieldtype(FIELDTYPE *type1,
    //                           FIELDTYPE *type2);

    pub fn new_form(_:*mut FIELD) -> FORM;
    pub fn free_form(_:FORM) -> c_int;

    pub fn set_new_page(_:FIELD, _:c_bool) -> c_int;
    pub fn new_page(_:FIELD) -> c_bool;

    pub fn pos_form_cursor(_:FORM) -> c_int;

    pub fn post_form(_:FORM) -> c_int;
    pub fn unpost_form(_:FORM) -> c_int;





}
