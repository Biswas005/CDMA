function __init__()
    a = 0
    b = 0
    codeA = Vector{Int64}()
    codeB = Vector{Int64}()
    As = Vector{Int64}()
    Bs = Vector{Int64}()
    Cs = Vector{Int64}()

    print("Enter data A : ")
    A = parse(Int64, readline())
    if A == 0
        A = -1
    end

    print("Enter data B : ")
    B = parse(Int64, readline())
    if B == 0
        B = -1
    end

    println("Enter codeA : ")
    for i = 1:8
         ele = parse(Int64, readline())
         append!(codeA, [ele])
         if codeA[i] == 0
            codeA[i] = -1
         end
    end

    println("Enter codeB : ")
    for i = 1:8
        ele = parse(Int64, readline())
         append!(codeB, [ele])
         if codeB[i] == 0
            codeB[i] = -1
         end
    end

    for i = 1:8
        append!(As, [A * codeA[i]])        
        append!(Bs, [B * codeB[i]])
        append!(Cs, [As[i] + Bs[i]])
        a = a + codeA[i] * Cs[i]
        b = b + codeB[i] * Cs[i]
    end

    println("\n\nCodeA : ", codeA)
    println("CodeB : ", codeB)
    println("As    : ", As)
    println("Bs    : ", Bs)
    println("Cs    : ", Cs)

    if a > 0
        println("\n\nA    : 1")
    else
        println("\n\nA    : 0")
    end

    if b > 0
        println("B    : 1")
    else
        println("B    : 0")
    end
end

__init__()