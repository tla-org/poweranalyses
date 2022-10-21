# Based on PackageCompile.jl
Base.reinit_stdio()
@eval Sys BINDIR = ccall(:jl_get_julia_bindir, Any, ())::String
@eval Sys STDLIB = (repr(abspath(Sys.BINDIR, "../share/julia/stdlib", string('v', VERSION.major, '.', VERSION.minor))))
copy!(LOAD_PATH, [repr(project)]) # Only allow loading packages from current project

Base.init_depot_path()
Base.init_load_path()

@show LOAD_PATH

using Pkg

empty!(LOAD_PATH)
empty!(DEPOT_PATH)
