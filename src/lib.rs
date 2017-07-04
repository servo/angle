#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod ffi {
    include!("bindings.rs");  

    use std::os::raw::{c_char, c_int, c_uint};
    extern {
        pub fn GLSLangInitialize() -> c_int;
        pub fn GLSLangFinalize() -> c_int;
        pub fn GLSLangInitBuiltInResources(res: *mut ShBuiltInResources);
        pub fn GLSLangConstructCompiler(_type: c_uint, spec: c_int, output: c_int, resources_ptr: *const ShBuiltInResources) -> ShHandle;
        pub fn GLSLangDestructCompiler(handle: ShHandle);
        pub fn GLSLangCompile(handle: ShHandle, strings: *const *const c_char, num_strings: usize, compile_options: c_int) -> c_int;
        pub fn GLSLangClearResults(handle: ShHandle);
        pub fn GLSLangGetShaderVersion(handle: ShHandle) -> c_int;
        pub fn GLSLangGetShaderOutputType(handle: ShHandle) -> c_int;
        pub fn GLSLangGetObjectCode(handle: ShHandle) -> *const c_char;
        pub fn GLSLangGetInfoLog(handle: ShHandle) -> *const c_char;
    } 
}
pub mod hl;

#[cfg(test)]
mod tests {
    use ffi::*;
    use hl::*;
    use std::sync::{ONCE_INIT, Once};
    static GLSLANG_INITIALIZATION: Once = ONCE_INIT;

    fn init() {
        GLSLANG_INITIALIZATION.call_once(|| initialize().unwrap());
    }


    #[test]
    fn test_linkage() {
        init();
    }

    #[test]
    fn test_translation_complex() {
        init();
        const FRAGMENT_SHADER: u32 = 0x8B30;
        let source = "
   precision mediump float;
   varying vec2 vTextureCoord;

   uniform sampler2D uSampler;

   void main() {
      gl_FragColor = texture2D(uSampler, vTextureCoord);
   }
";
        let resources = BuiltInResources::default();
        let compiler = ShaderValidator::for_webgl(FRAGMENT_SHADER,
                                                  Output::Glsl,
                                                  &resources).unwrap();

        assert!(compiler.compile_and_translate(&[source]).is_ok());
    }

    #[test]
    fn test_translation() {
        const SHADER: &'static str = "void main() {
    gl_FragColor = vec4(0, 1, 0, 1);  // green
}";
        const EXPECTED: &'static str = "void main(){
(gl_FragColor = vec4(0.0, 1.0, 0.0, 1.0));
}\n";
        const FRAGMENT_SHADER: u32 = 0x8B30;

        init();

        let resources = BuiltInResources::default();
        let compiler = ShaderValidator::for_webgl(FRAGMENT_SHADER,
                                                  Output::Glsl,
                                                  &resources).unwrap();

        let result = compiler.compile_and_translate(&[SHADER]).unwrap();
        println!("{:?}", result);
        assert!(result == EXPECTED);
    }

    #[test]
    fn test_translation_essl() {
        const SHADER: &'static str = "void main() {
    gl_FragColor = vec4(0, 1, 0, 1);  // green
}";
        const EXPECTED: &'static str = "void main(){
(gl_FragColor = vec4(0.0, 1.0, 0.0, 1.0));
}\n";
        const FRAGMENT_SHADER: u32 = 0x8B30;

        init();

        let compiler = ShaderValidator::for_webgl(FRAGMENT_SHADER,
                                                  Output::Essl,
                                                  &BuiltInResources::default()).expect("Failed to create a validator for essl");

        let result = compiler.compile_and_translate(&[SHADER]).unwrap();
        println!("{:?}", result);
        assert!(result == EXPECTED);
    }
}
