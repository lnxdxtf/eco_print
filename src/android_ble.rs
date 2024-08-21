use jni::objects::GlobalRef;
use jni::{JNIEnv, JavaVM};
use once_cell::sync::OnceCell;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AndroidError {
    #[error("JNI {0}")]
    Jni(#[from] jni::errors::Error),

    #[error("Cannot initialize CLASS_LOADER")]
    ClassLoader,

    #[error("Java vm not initialized")]
    JavaVM,

    #[error("Btleplug error: {0}")]
    Btleplug(#[from] btleplug::Error),
}

pub static JAVAVM: OnceCell<JavaVM> = OnceCell::new();

pub fn attach_current_thread() -> Result<(), AndroidError> {
    log::info!("Attempting to attach current thread to JVM");
    let vm = JAVAVM.get().ok_or(AndroidError::JavaVM)?;
    log::info!("JVM found, attempting to attach thread");
    let env = vm.attach_current_thread().map_err(|e| {
        log::error!("Failed to attach current thread: {:?}", e);
        AndroidError::Jni(e)
    })?;
    log::info!("Thread attached successfully");
    Ok(())
}

pub fn setup_class_loader(env: &JNIEnv) -> Result<GlobalRef, AndroidError> {
    let thread = env
        .call_static_method(
            "java/lang/Thread",
            "currentThread",
            "()Ljava/lang/Thread;",
            &[],
        )?
        .l()?;
    let class_loader = env
        .call_method(
            thread,
            "getContextClassLoader",
            "()Ljava/lang/ClassLoader;",
            &[],
        )?
        .l()?;

    Ok(env.new_global_ref(class_loader)?)
}

#[no_mangle]
pub extern "C" fn JNI_OnLoad(vm: jni::JavaVM, _res: *const std::os::raw::c_void) -> jni::sys::jint {
    log::info!("JNI_OnLoad");
    let env = vm.get_env().unwrap();
    jni_utils::init(&env).unwrap();
    btleplug::platform::init(&env).unwrap();
    let _ = JAVAVM.set(vm);
    log::info!("JNI_OnLoad done");
    jni::JNIVersion::V6.into()
}
