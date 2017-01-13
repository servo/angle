#![feature(cstr_to_str)]

extern crate libc;

pub mod ffi;
pub mod hl;

#[cfg(test)]
mod tests {
    use ffi::*;
    use hl::*;

    #[test]
    fn test_linkage() {
        assert_eq!(unsafe { GLSLangInitialize() }, 1);
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

        initialize().unwrap();

        let compiler = ShaderValidator::for_webgl(FRAGMENT_SHADER,
                                                  Output::Glsl,
                                                  &BuiltInResources::default()).unwrap();

        let result = compiler.compile_and_translate(&[SHADER.to_owned().as_bytes()]).unwrap();
        println!("{:?}", result);
        assert!(result == EXPECTED);
    }
}
