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

    print(" Enter DATA B : ")
    B = parse(Int64, readline())
    if B == 0
        B = -1
    end

    println(" Enter CodeA : ")
    for i = 1:8
         ele = parse(Int64, readline())
         append!(codeA, [ele])
         if codeA[i] == 0
            codeA[i] = -1
         end
    end
    println(codeA)

    println(" Enter CodeB : ")
    for i = 1:8
        ele = parse(Int64, readline())
         append!(codeB, [ele])
         if codeB[i] == 0
            codeB[i] = -1
         end
    end
    println(codeB)

    for i = 1:8
        x = A * codeA[i]
        append!( As, [x])

        y = B * codeB[i]
        append!(Bs, [y])

        z = As[i] + Bs[i]
        append!(Cs, [z])
    end

    println(" As : ")
    for i = 1:8
        print( " ", As[i])
    end

    println(" Bs : ")
    for i = 1:8
        print( " ", Bs[i])
    end

    println("\n Cs : ")
    for i = 1:8
        print( " ", Cs[i])
    end

    for i = 1:8
        a = a + codeA[i] * Cs[i]
        b = b + codeB[i] * Cs[i]
    end


    println("\n \n A: ",A, "\n B:", B, "\n \n codeA: " , codeA, "\n codeB: " , codeB," \n\n As:",  As," \n Bs:", Bs, " \n Cs:", Cs," \n \n a:", a, " \n b:", b, "\n \n")

    if a > 0
        println(" A: 1")
    else
        println(" A: 0")
    end

    if b > 0
        println(" B: 1")
    else
        println(" B: 0")
    end
end

__init__()
