module Library

export increment64

Base.@ccallable function increment64(count::Clong)::Clong
    count += 1
    println("Incremented count: $count (Clong)")
    return count
end

end # module
