using PackageCompiler

const ROOT_DIR = dirname(dirname(@__DIR__))::String
const PROJECT_DIR = joinpath(ROOT_DIR, "library")
const TARGET_DIR = joinpath(ROOT_DIR, "_library")

println("Creating library in $TARGET_DIR")

PackageCompiler.create_library(
    PROJECT_DIR,
    TARGET_DIR;
    lib_name="mylib",
    precompile_execution_file=joinpath(@__DIR__, "precompile_execution_file.jl"),
    incremental=false,
    filter_stdlibs=true,
    force=true
)
