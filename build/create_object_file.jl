using PackageCompiler:
    NATIVE_CPU_TARGET,
    PackageCompiler,
    check_packages_in_project,
    create_pkg_context,
    create_sysimg_object_file
using Pkg: Pkg

# Project which should be active when the sysimage is created.
project = joinpath(dirname(@__DIR__), "library")

# The list of packages that should be included.
packages = ["Library"]

# This is the list of transitive dependencies that are loaded based on the above `packages` list.
packages_sysimg = ""

function _create_object_file(
        project::String,
        packages::Vector{String};
        base_sysimage::Union{Nothing,String}=nothing,
        cpu_target::String=NATIVE_CPU_TARGET,
        incremental::Bool=true
    )
    ctx = create_pkg_context(project)
    check_packages_in_project(ctx, packages)

    @info "Instantiating project at $(repr(project))"
    Pkg.instantiate(ctx; verbose=true, allow_autoprecomp=false)

    if !incremental
        if base_sysimage !== nothing
            error("cannot specify `base_sysimage`  when `incremental=false`")
        end
        sysimage_stdlibs = filter_stdlibs ? gather_stdlibs_project(ctx) : stdlibs_in_sysimage()
        base_sysimage = create_fresh_base_sysimage(sysimage_stdlibs; cpu_target)
    else
        base_sysimage = something(base_sysimage, unsafe_string(Base.JLOptions().image_file))
    end

    packages_sysimg = Set{Base.PkgId}()

    if true # include_transitive_dependencies
        # We are not sure that packages actually load their dependencies on `using`
        # but we still want them to end up in the sysimage. Therefore, explicitly
        # collect their dependencies, recursively.

        frontier = Set{Base.PkgId}()
        deps = ctx.env.project.deps
        for pkg in packages
            # Add all dependencies of the package
            if ctx.env.pkg !== nothing && pkg == ctx.env.pkg.name
                push!(frontier, Base.PkgId(ctx.env.pkg.uuid, pkg))
            else
                uuid = ctx.env.project.deps[pkg]
                push!(frontier, Base.PkgId(uuid, pkg))
            end
        end
        copy!(packages_sysimg, frontier)
        new_frontier = Set{Base.PkgId}()
        while !(isempty(frontier))
            for pkgid in frontier
                deps = if ctx.env.pkg !== nothing && pkgid.uuid == ctx.env.pkg.uuid
                    ctx.env.project.deps
                else
                    ctx.env.manifest[pkgid.uuid].deps
                end
                pkgid_deps = [Base.PkgId(uuid, name) for (name, uuid) in deps]
                for pkgid_dep in pkgid_deps
                    if !(pkgid_dep in packages_sysimg) #
                        push!(packages_sysimg, pkgid_dep)
                        push!(new_frontier, pkgid_dep)
                    end
                end
            end
            copy!(frontier, new_frontier)
            empty!(new_frontier)
        end
    end

    precompile_execution_file=vcat(joinpath(@__DIR__, "precompile_execution_file.jl"))

    create_sysimg_object_file(
        "julia.o",
        packages,
        packages_sysimg;
        base_sysimage,
        cpu_target,
        project,
        precompile_execution_file,
        precompile_statements_file=String[],
        script=nothing,
        sysimage_build_args=``,
        incremental=false,
        extra_precompiles=""
    )
end

_create_object_file(project, packages)
