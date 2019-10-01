# Conceptos Previos a la Generacion de CI

## Frames Dinamicos

Es una region de memoria que toda funcion activa (he entrado a su codigo y no he hecho return) tiene para disponer de ella. En esta memoria va:

- Argumentos que no han pasado por registros
- Direcciones de retorno
- Variables que no van en regitros
- Registros spilleados
- Registros calle-saves
- etc, etc

Se administra de forma LIFO.

En los 70 los programas pasaban todos los argumentos por stack.

En los 60 era comun disponer disponer de memoria estatica para esto: trafico innecesario y no recursivo.

## Frames Estaticos

Es una estructura que maneja el compilador (una por funcion) para poder generar el codigo para el frame dinamico.

**Nota**: el frame dinamico depende de c/lenguaje y procesador.

En tigerfram.sml:

    datatype access = InReg of tigertemp.temp
                    | InMem of int

En tigertemp.sml:

    type temp = string
    local
        val tip = ref 0
    in
        fun memtemp()= "T"^IntToString(!tmp) before tmp:= !tmp + 1

Definimos registros con funcionalidades.

    val rv = "%rax" // Return value
    val sp = "%rsp" // Stack Pointer
    val fp = "%rbp" // Frame Pointer
    val rargs = ["%rdi", "%rsi", ... ]
    val ov = "%rdx" // Overflow

## Como acceder a variables escapadas

Ej:

    let
        functon f(i: int) =
            let
                var j := 10
                function g() =
                    print(chr(ord("A") + i + j))
            in g() end
    in f() end

Toda funcion tiene un parametro extra, implicito, llamado static link (sl) y que apunta al fp anterior del ultimo fram de la funcion anidante.

    Nivel_llamante < nivel llamada => Nivel_llamante + 1 = Nivel_llamada
        SL_llamada = fp_llamante
    Nivel_llamante = nivel llamada
        SL_llamada = SL_llamante
    Nivel_llamante > Nivel_llamado; Si Nivel__llamante - Nivel_llamado = n
        SL_llamada = *(SL + offset) n veces

# Codigo Intermedio: Tree

En tigertree.sml:

    datatype exp = CONST of int
                 | NAME of temp.label
                 | TEMP of temp.temp
                 | BINOP of binop * exp * exp
                 | MEM of exp
                 | CALL of exp * exp list
                 | ESEQ of stm * exp
    and stm = MOVE of exp * exp
            | EXP of  exp
            | JUMP of exp * temp.label list
            | CJUMP of relop * exp * exp * temp.label * temp.label
            | LABEL of temp.label
    and binop = PLUS | MINUS | MUL | DIV
              | AND | OR | LSHIFT | RSHIFT
              | ARSHIFT | XOR
    and relop = EQ | NE | LT | CT | | LE | GE
              | ULT | ULE | UGT | UGE

En translate:

    datatype exp = Ex of tree.exp
                 | Nx of tree.stm
                 | Cx of temp.label * temp.label .> tre.stm

    fun unEx(Ex e) = e
    | unEx(Nx s) = ESEQ(S, CONST 0)
    | unCx(genstm) =
        let val (r, t, f) = (newLabel(), newLabel(), newLabel())
        in
            ESEQ(
                seq[
                    MOVE(TEMP r, CONST 0),
                    genstm(t, f),
                    LABEL t,
                    MOVE (TEMP r, CONST 0),
                    LABEL f]
                ]),
                TEMP r)
