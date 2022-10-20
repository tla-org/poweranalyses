using PackageCompiler:
    PackageCompiler,
    bundle_julia_libraries,
    bundle_artifacts,
    bundle_headers,
    bundle_cert,
    create_sysimage_workaround,
    create_pkg_context,
    default_app_cpu_target,
    get_library_filename,
    try_rm_dir
using Pkg: Pkg

const ROOT_DIR = dirname(dirname(@__DIR__))::String
const PROJECT_DIR = joinpath(ROOT_DIR, "library")
const TARGET_DIR = joinpath(ROOT_DIR, "_library")

println("Creating library in $TARGET_DIR")

# Based on PackageCompiler.jl's src.
function create_static_library(
    package_dir::String,
    dest_dir::String;
    lib_name=nothing,
    precompile_execution_file::Union{String, Vector{String}}=String[],
    precompile_statements_file::Union{String, Vector{String}}=String[],
    incremental::Bool=false,
    filter_stdlibs::Bool=false,
    force::Bool=false,
    header_files::Vector{String} = String[],
    julia_init_c_file::String=String(PackageCompiler.DEFAULT_JULIA_INIT),
    version::Union{String,VersionNumber,Nothing}=nothing,
    compat_level::String="major",
    cpu_target::String=default_app_cpu_target(),
    include_lazy_artifacts::Bool=false,
    sysimage_build_args::Cmd=``,
    include_transitive_dependencies::Bool=true
)
    PackageCompiler.warn_official()

    julia_init_h_file = String(PackageCompiler.DEFAULT_JULIA_INIT_HEADER)

    if !(julia_init_h_file in header_files)
        push!(header_files, julia_init_h_file)
    end

    if version isa String
        version = parse(VersionNumber, version)
    end

    ctx = create_pkg_context(package_dir)
    ctx.env.pkg === nothing && error("expected package to have a `name` and `uuid`")
    Pkg.instantiate(ctx, verbose=true, allow_autoprecomp=false)

    lib_name = something(lib_name, ctx.env.pkg.name)
    try_rm_dir(dest_dir; force)
    mkpath(dest_dir)
    bundle_julia_libraries(dest_dir)
    bundle_artifacts(ctx, dest_dir; include_lazy_artifacts)
    bundle_headers(dest_dir, header_files)
    bundle_cert(dest_dir)

    lib_dir = Sys.iswindows() ? joinpath(dest_dir, "bin") : joinpath(dest_dir, "lib")

    sysimg_file = get_library_filename(lib_name; version)
    sysimg_path = joinpath(lib_dir, sysimg_file)
    compat_file = get_library_filename(lib_name; version, compat_level)
    soname = (Sys.isunix() && !Sys.isapple()) ? compat_file : nothing

    create_sysimage_workaround(ctx, sysimg_path, precompile_execution_file,
        precompile_statements_file, incremental, filter_stdlibs, cpu_target;
        sysimage_build_args, include_transitive_dependencies, julia_init_c_file, version,
        soname)

    return nothing
end

create_static_library(
    PROJECT_DIR,
    TARGET_DIR;
    lib_name="mylib",
    precompile_execution_file=joinpath(@__DIR__, "precompile_execution_file.jl"),
    incremental=false,
    filter_stdlibs=true,
    force=true,
    header_files=[joinpath(PROJECT_DIR, "src", "mylib.h")]
)
