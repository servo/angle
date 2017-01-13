project(angle)
cmake_minimum_required(VERSION 2.6)

set(DUMMY ${CMAKE_BUILD_TYPE})

set(ANGLE_SOURCES
  src/glslang-c.cpp
  src/common/Float16ToFloat32.cpp
  src/common/MemoryBuffer.cpp
  src/common/angleutils.cpp
  src/common/debug.cpp
  src/common/event_tracer.cpp
  src/common/mathutil.cpp
  src/common/string_utils.cpp
  src/common/tls.cpp
  src/common/utilities.cpp
  src/compiler/preprocessor/DiagnosticsBase.cpp
  src/compiler/preprocessor/DirectiveHandlerBase.cpp
  src/compiler/preprocessor/DirectiveParser.cpp
  src/compiler/preprocessor/ExpressionParser.cpp
  src/compiler/preprocessor/Input.cpp
  src/compiler/preprocessor/Lexer.cpp
  src/compiler/preprocessor/Macro.cpp
  src/compiler/preprocessor/MacroExpander.cpp
  src/compiler/preprocessor/Preprocessor.cpp
  src/compiler/preprocessor/Token.cpp
  src/compiler/preprocessor/Tokenizer.cpp
  src/compiler/translator/BuiltInFunctionEmulator.cpp
  src/compiler/translator/BuiltInFunctionEmulatorGLSL.cpp
  src/compiler/translator/Cache.cpp
  src/compiler/translator/CallDAG.cpp
  src/compiler/translator/CodeGen.cpp
  src/compiler/translator/Compiler.cpp
  src/compiler/translator/Diagnostics.cpp
  src/compiler/translator/DirectiveHandler.cpp
  src/compiler/translator/EmulatePrecision.cpp
  src/compiler/translator/ExtensionGLSL.cpp
  src/compiler/translator/FlagStd140Structs.cpp
  src/compiler/translator/ForLoopUnroll.cpp
  src/compiler/translator/InfoSink.cpp
  src/compiler/translator/Initialize.cpp
  src/compiler/translator/InitializeDll.cpp
  src/compiler/translator/InitializeParseContext.cpp
  src/compiler/translator/InitializeVariables.cpp
  src/compiler/translator/IntermNode.cpp
  src/compiler/translator/IntermTraverse.cpp
  src/compiler/translator/Intermediate.cpp
  src/compiler/translator/LoopInfo.cpp
  src/compiler/translator/Operator.cpp
  src/compiler/translator/OutputESSL.cpp
  src/compiler/translator/OutputGLSL.cpp
  src/compiler/translator/OutputGLSLBase.cpp
  src/compiler/translator/ParseContext.cpp
  src/compiler/translator/PoolAlloc.cpp
  src/compiler/translator/PruneEmptyDeclarations.cpp
  src/compiler/translator/RecordConstantPrecision.cpp
  src/compiler/translator/RegenerateStructNames.cpp
  src/compiler/translator/RemovePow.cpp
  src/compiler/translator/RewriteDoWhile.cpp
  src/compiler/translator/ScalarizeVecAndMatConstructorArgs.cpp
  src/compiler/translator/SearchSymbol.cpp
  src/compiler/translator/ShaderLang.cpp
  src/compiler/translator/ShaderVars.cpp
  src/compiler/translator/SymbolTable.cpp
  src/compiler/translator/TranslatorESSL.cpp
  src/compiler/translator/TranslatorGLSL.cpp
  src/compiler/translator/Types.cpp
  src/compiler/translator/UnfoldShortCircuitAST.cpp
  src/compiler/translator/ValidateGlobalInitializer.cpp
  src/compiler/translator/ValidateLimitations.cpp
  src/compiler/translator/ValidateMaxParameters.cpp
  src/compiler/translator/ValidateOutputs.cpp
  src/compiler/translator/ValidateSwitch.cpp
  src/compiler/translator/VariableInfo.cpp
  src/compiler/translator/VariablePacker.cpp
  src/compiler/translator/VersionGLSL.cpp
  src/compiler/translator/blocklayout.cpp
  src/compiler/translator/depgraph/DependencyGraph.cpp
  src/compiler/translator/depgraph/DependencyGraphBuilder.cpp
  src/compiler/translator/depgraph/DependencyGraphOutput.cpp
  src/compiler/translator/depgraph/DependencyGraphTraverse.cpp
  src/compiler/translator/glslang_lex.cpp
  src/compiler/translator/glslang_tab.cpp
  src/compiler/translator/intermOut.cpp
  src/compiler/translator/timing/RestrictFragmentShaderTiming.cpp
  src/compiler/translator/timing/RestrictVertexShaderTiming.cpp
  src/compiler/translator/util.cpp
  src/libANGLE/Platform.cpp
  src/third_party/compiler/ArrayBoundsClamper.cpp
  )

add_definitions(
  -DANGLE_TRANSLATOR_STATIC
  -DANGLE_ENABLE_ESSL
  -DANGLE_ENABLE_GLSL
  -DLIBGLESV2_EXPORT_H
  -DLIBANGLE_IMPLEMENTATION
  -DCOMPONENT_BUILD)

# This is only relevant if we're building libGLESv2, which
# we don't.  When we do, we'll need to modify include/export.h
# to add a #define for a static build instead of always using
# DLL goop on Windows.
#add_definitions(-DANGLE_EXPORT="")

include_directories(include src)

if(NOT WIN32 OR MINGW)
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=gnu++11")
endif()

if(NOT WIN32)
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fPIC -Wall")
endif()

if(MSVC)
  add_definitions(
    -EHsc
    -D_HAS_EXCEPTIONS=0
    -DNOMINMAX
    -D_CRT_SECURE_NO_DEPRECATE
    -D_SCL_SECURE_NO_WARNINGS)
endif()

add_library(angle STATIC ${ANGLE_SOURCES})
install(TARGETS angle ARCHIVE DESTINATION .)
