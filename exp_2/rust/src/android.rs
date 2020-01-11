use jni::objects::{JClass, JString};
use jni::sys::{jlong, jobject, jstring, JNIEnv};
use std::panic;

use log::{error, info};

fn set_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        error!("{}", panic_info.to_string());
    }));
}

struct AndroidActivityProxy {}
impl AndroidActivityProxy {
    fn new(env: *mut JNIEnv, activity: jobject) -> Self {
        AndroidActivityProxy {}
    }
    fn on_start(&self) {
        info!("on_start");
    }
    fn on_resume(&self) {
        info!("on_resume");
    }
    fn on_pause(&self) {
        info!("on_pause");
    }
    fn on_stop(&self) {
        info!("on_stop");
    }
    fn on_destroy(&self) {
        info!("on_destroy");
    }

    fn surface_created(&mut self, env: *mut JNIEnv, surface: jobject) {
        info!("surface_created");
    }
    fn surface_changed(&mut self, env: *mut JNIEnv, surface: jobject) {
        info!("surface_changed");
    }
    fn surface_destroyed(&mut self) {
        info!("surface_destroyed");
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_myapplication_LibRust_onCreate(
    env: *mut JNIEnv,
    _: jobject,
    activity: jobject,
) -> jlong {
    crate::init_logger();
    set_panic_hook();
    let activity_proxy = Box::new(AndroidActivityProxy::new(env, activity));
    Box::into_raw(activity_proxy) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_myapplication_LibRust_onStart(
    _: *mut JNIEnv,
    _: jobject,
    activity_proxy: jlong,
) {
    let activity_proxy = (activity_proxy as *mut AndroidActivityProxy)
        .as_mut()
        .unwrap();
    activity_proxy.on_start();
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_myapplication_LibRust_onResume(
    _: *mut JNIEnv,
    _: jobject,
    activity_proxy: jlong,
) {
    let activity_proxy = (activity_proxy as *mut AndroidActivityProxy)
        .as_mut()
        .unwrap();
    activity_proxy.on_resume();
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_myapplication_LibRust_onPause(
    _: *mut JNIEnv,
    _: jobject,
    activity_proxy: jlong,
) {
    let activity_proxy = (activity_proxy as *mut AndroidActivityProxy)
        .as_mut()
        .unwrap();
    activity_proxy.on_pause();
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_myapplication_LibRust_onStop(
    _: *mut JNIEnv,
    _: jobject,
    activity_proxy: jlong,
) {
    let activity_proxy = (activity_proxy as *mut AndroidActivityProxy)
        .as_mut()
        .unwrap();
    activity_proxy.on_stop();
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_myapplication_LibRust_onDestroy(
    _: *mut JNIEnv,
    _: jobject,
    activity_proxy: jlong,
) {
    let activity_proxy = (activity_proxy as *mut AndroidActivityProxy)
        .as_mut()
        .unwrap();
    activity_proxy.on_destroy();
    Box::from_raw(activity_proxy);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_myapplication_LibRust_surfaceCreated(
    env: *mut JNIEnv,
    _: jobject,
    activity_proxy: jlong,
    surface: jobject,
) {
    let activity_proxy = (activity_proxy as *mut AndroidActivityProxy)
        .as_mut()
        .unwrap();
    activity_proxy.surface_created(env, surface);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_myapplication_LibRust_surfaceChanged(
    env: *mut JNIEnv,
    _: jobject,
    activity_proxy: jlong,
    surface: jobject,
) {
    let activity_proxy = (activity_proxy as *mut AndroidActivityProxy)
        .as_mut()
        .unwrap();
    activity_proxy.surface_changed(env, surface);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_myapplication_LibRust_surfaceDestroyed(
    _: *mut JNIEnv,
    _: jobject,
    activity_proxy: jlong,
) {
    let activity_proxy = (activity_proxy as *mut AndroidActivityProxy)
        .as_mut()
        .unwrap();
    activity_proxy.surface_destroyed();
}
