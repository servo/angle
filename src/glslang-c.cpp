#include "GLSLANG/ShaderLang.h"

extern "C"
int GLSLangInitialize() {
    if ( ShInitialize() )
        return 1;
    return 0;
}

extern "C"
int GLSLangFinalize() {
    if ( ShFinalize() )
        return 1;
    return 0;
}

extern "C"
void GLSLangInitBuiltInResources(ShBuiltInResources *resources) {
    ShInitBuiltInResources(resources);
}

extern "C"
const char* GLSLangGetBuiltInResourcesString(const ShHandle handle) {
    return ShGetBuiltInResourcesString(handle).c_str();
}

extern "C"
ShHandle GLSLangConstructCompiler(unsigned int type,
                                  int spec,
                                  int output,
                                  const ShBuiltInResources *resources) {
    return ShConstructCompiler(static_cast<sh::GLenum>(type),
                               static_cast<ShShaderSpec>(spec),
                               static_cast<ShShaderOutput>(output),
                               resources);
}

extern "C"
void GLSLangDestructCompiler(ShHandle handle) {
    ShDestruct(handle);
}

extern "C"
int GLSLangCompile(const ShHandle handle,
               const char* const shaderStrings[],
               size_t numStrings,
               int compileOptions) {
    if ( ShCompile(handle, shaderStrings, numStrings, compileOptions) )
        return 1;

    return 0;
}

extern "C"
void GLSLangClearResults(const ShHandle handle) {
    ShClearResults(handle);
}

extern "C"
int GLSLangGetShaderVersion(const ShHandle handle) {
    return ShGetShaderVersion(handle);
}

extern "C"
int GLSLangGetShaderOutputType(const ShHandle handle) {
    return ShGetShaderOutputType(handle);
}

extern "C"
const char* GLSLangGetInfoLog(const ShHandle handle) {
    return ShGetInfoLog(handle).c_str();
}

// Returns null-terminated object code for a compiled shader.
// Parameters:
// handle: Specifies the compiler
extern "C"
const char* GLSLangGetObjectCode(const ShHandle handle) {
    return ShGetObjectCode(handle).c_str();
}
