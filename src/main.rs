#![allow(dead_code)]
mod test_option;
mod test_result;
mod test_iter;
mod test_destruect;
mod test_strings;
mod test_ownship;
mod test_trait;
mod test_box;
mod test_generic;
mod test_closure;
mod test_datastruct;
mod  test_buffer;

use test_option::test_option_all;
use test_result::test_result_all;
use test_iter::test_iter_all;
use test_destruect::test_destruect_all;
use test_strings::test_string_all;
use test_ownship::test_ownship_all;
use test_trait::test_trait_all;
use test_box::test_box_all;
use test_generic::test_generic_all;
use test_closure::test_closure_all;
use test_datastruct::test_datastruct_all;
use test_buffer::test_buffer_all;


fn main() {
    // test_option_all();
    // test_result_all();
    // test_iter_all();
    // test_destruect_all();
    // test_string_all();
    // test_ownship_all();
    // test_trait_all();
    // test_box_all();
    // test_generic_all();
    // test_closure_all();
    // test_datastruct_all();
    test_buffer_all();
    println!("the end!");
}
