//

// compile-flags: -C no-prepopulate-passes

#![crate_type = "lib"]

#![feature(repr_simd, platform_intrinsics)]
#![allow(non_camel_case_types)]

#[repr(simd)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec2<T>(pub T, pub T);

#[repr(simd)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec4<T>(pub T, pub T, pub T, pub T);

extern "platform-intrinsic" {
    fn simd_load<T, P, M>(values: T, pointer: P, mask: M) -> T;
}

// CHECK-LABEL: @load_f32x2
#[no_mangle]
pub unsafe fn load_f32x2(pointer: *const f32, mask: Vec2<i32>,
                         values: Vec2<f32>) -> Vec2<f32> {
    // CHECK: call <2 x float> @llvm.masked.load.v2f32.p0(ptr {{.*}}, i32 {{.*}}, <2 x i1> {{.*}}, <2 x float> {{.*}})
    simd_load(values, pointer, mask)
}

// CHECK-LABEL: @load_pf32x2
#[no_mangle]
pub unsafe fn load_pf32x2(pointer: *const *const f32, mask: Vec2<i32>,
                          values: Vec2<*const f32>) -> Vec2<*const f32> {
    // CHECK: call <2 x ptr> @llvm.masked.load.v2p0.p0({{.*}}, i32 {{.*}}, <2 x i1> {{.*}}, <2 x ptr> {{.*}})
    simd_load(values, pointer, mask)
}
