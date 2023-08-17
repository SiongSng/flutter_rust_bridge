#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.81.0.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_draw_mandelbrot_impl(
    port_: MessagePort,
    image_size: impl Wire2Api<Size> + UnwindSafe,
    zoom_point: impl Wire2Api<Point> + UnwindSafe,
    scale: impl Wire2Api<f64> + UnwindSafe,
    num_threads: impl Wire2Api<i32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ZeroCopyBuffer<Vec<u8>>>(
        WrapInfo {
            debug_name: "draw_mandelbrot",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_image_size = image_size.wire2api();
            let api_zoom_point = zoom_point.wire2api();
            let api_scale = scale.wire2api();
            let api_num_threads = num_threads.wire2api();
            move |task_callback| {
                draw_mandelbrot(api_image_size, api_zoom_point, api_scale, api_num_threads)
            }
        },
    )
}
fn wire_passing_complex_structs_impl(
    port_: MessagePort,
    root: impl Wire2Api<TreeNode> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String>(
        WrapInfo {
            debug_name: "passing_complex_structs",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_root = root.wire2api();
            move |task_callback| Result::<_, ()>::Ok(passing_complex_structs(api_root))
        },
    )
}
fn wire_returning_structs_with_boxed_fields_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, BoxedPoint>(
        WrapInfo {
            debug_name: "returning_structs_with_boxed_fields",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Result::<_, ()>::Ok(returning_structs_with_boxed_fields()),
    )
}
fn wire_off_topic_memory_test_input_array_impl(
    port_: MessagePort,
    input: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, i32>(
        WrapInfo {
            debug_name: "off_topic_memory_test_input_array",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_input = input.wire2api();
            move |task_callback| Result::<_, ()>::Ok(off_topic_memory_test_input_array(api_input))
        },
    )
}
fn wire_off_topic_memory_test_output_zero_copy_buffer_impl(
    port_: MessagePort,
    len: impl Wire2Api<i32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ZeroCopyBuffer<Vec<u8>>>(
        WrapInfo {
            debug_name: "off_topic_memory_test_output_zero_copy_buffer",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_len = len.wire2api();
            move |task_callback| {
                Result::<_, ()>::Ok(off_topic_memory_test_output_zero_copy_buffer(api_len))
            }
        },
    )
}
fn wire_off_topic_memory_test_output_vec_u8_impl(
    port_: MessagePort,
    len: impl Wire2Api<i32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Vec<u8>>(
        WrapInfo {
            debug_name: "off_topic_memory_test_output_vec_u8",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_len = len.wire2api();
            move |task_callback| Result::<_, ()>::Ok(off_topic_memory_test_output_vec_u8(api_len))
        },
    )
}
fn wire_off_topic_memory_test_input_vec_of_object_impl(
    port_: MessagePort,
    input: impl Wire2Api<Vec<Size>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, i32>(
        WrapInfo {
            debug_name: "off_topic_memory_test_input_vec_of_object",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_input = input.wire2api();
            move |task_callback| {
                Result::<_, ()>::Ok(off_topic_memory_test_input_vec_of_object(api_input))
            }
        },
    )
}
fn wire_off_topic_memory_test_output_vec_of_object_impl(
    port_: MessagePort,
    len: impl Wire2Api<i32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Vec<Size>>(
        WrapInfo {
            debug_name: "off_topic_memory_test_output_vec_of_object",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_len = len.wire2api();
            move |task_callback| {
                Result::<_, ()>::Ok(off_topic_memory_test_output_vec_of_object(api_len))
            }
        },
    )
}
fn wire_off_topic_memory_test_input_complex_struct_impl(
    port_: MessagePort,
    input: impl Wire2Api<TreeNode> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, i32>(
        WrapInfo {
            debug_name: "off_topic_memory_test_input_complex_struct",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_input = input.wire2api();
            move |task_callback| {
                Result::<_, ()>::Ok(off_topic_memory_test_input_complex_struct(api_input))
            }
        },
    )
}
fn wire_off_topic_memory_test_output_complex_struct_impl(
    port_: MessagePort,
    len: impl Wire2Api<i32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, TreeNode>(
        WrapInfo {
            debug_name: "off_topic_memory_test_output_complex_struct",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_len = len.wire2api();
            move |task_callback| {
                Result::<_, ()>::Ok(off_topic_memory_test_output_complex_struct(api_len))
            }
        },
    )
}
fn wire_off_topic_deliberately_return_error_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, i32>(
        WrapInfo {
            debug_name: "off_topic_deliberately_return_error",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| off_topic_deliberately_return_error(),
    )
}
fn wire_off_topic_deliberately_panic_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, i32>(
        WrapInfo {
            debug_name: "off_topic_deliberately_panic",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Result::<_, ()>::Ok(off_topic_deliberately_panic()),
    )
}
fn wire_next_user_id_impl(port_: MessagePort, user_id: impl Wire2Api<UserId> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, UserId>(
        WrapInfo {
            debug_name: "next_user_id",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_user_id = user_id.wire2api();
            move |task_callback| Result::<_, ()>::Ok(next_user_id(api_user_id))
        },
    )
}
fn wire_test_method__method__BoxedPoint_impl(
    port_: MessagePort,
    that: impl Wire2Api<BoxedPoint> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "test_method__method__BoxedPoint",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Result::<_, ()>::Ok(BoxedPoint::test_method(&api_that))
        },
    )
}
fn wire_sum__method__SumWith_impl(
    port_: MessagePort,
    that: impl Wire2Api<SumWith> + UnwindSafe,
    y: impl Wire2Api<u32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, u32>(
        WrapInfo {
            debug_name: "sum__method__SumWith",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_y = y.wire2api();
            move |task_callback| Result::<_, ()>::Ok(SumWith::sum(&api_that, api_y))
        },
    )
}
fn wire_sum_static__static_method__SumWith_impl(
    port_: MessagePort,
    x: impl Wire2Api<u32> + UnwindSafe,
    y: impl Wire2Api<u32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, u32>(
        WrapInfo {
            debug_name: "sum_static__static_method__SumWith",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_x = x.wire2api();
            let api_y = y.wire2api();
            move |task_callback| Result::<_, ()>::Ok(SumWith::sum_static(api_x, api_y))
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<f64> for f64 {
    fn wire2api(self) -> f64 {
        self
    }
}
impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}

impl Wire2Api<u32> for u32 {
    fn wire2api(self) -> u32 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for BoxedPoint {
    fn into_dart(self) -> support::DartAbi {
        vec![self.point.into_into_dart().into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for BoxedPoint {}
impl rust2dart::IntoIntoDart<BoxedPoint> for BoxedPoint {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for Point {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.x.into_into_dart().into_dart(),
            self.y.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Point {}
impl rust2dart::IntoIntoDart<Point> for Point {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for Size {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.width.into_into_dart().into_dart(),
            self.height.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Size {}
impl rust2dart::IntoIntoDart<Size> for Size {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for TreeNode {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.name.into_into_dart().into_dart(),
            self.children.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for TreeNode {}
impl rust2dart::IntoIntoDart<TreeNode> for TreeNode {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for UserId {
    fn into_dart(self) -> support::DartAbi {
        vec![self.value.into_into_dart().into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for UserId {}
impl rust2dart::IntoIntoDart<UserId> for UserId {
    fn into_into_dart(self) -> Self {
        self
    }
}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "bridge_generated.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
