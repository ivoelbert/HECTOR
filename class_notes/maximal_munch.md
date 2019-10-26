# Maximal Munch en Tiger

Como representamos una instruccion en assembler?
Principalmente, por una cadena con el assembler.

## En tigerassem.sml

    datatype inst OPER of {
        assem: string,
    }

    max codigo, verlo en el repo

Aparte de asem, tenemos campos con los temporarios que se definen o se usan en cada instruccion y para poder reconstruir el flujo del programa, el campo jump que indica los posible saltos que se hagan en una instruccion.

Vamos a usar OPER para la mayoria de las instrucciones. LABEL quedará para la definicion de Labels y MOVE solo para el caso de una copia de registro a registro. Este ultimo caso, si ponemos la fuente y el destino en el mismo registro fisico, podemos eliminarlo.


En tree tenemos dos tipos mutuamente recursivos: exp y stm. Vamos a definir dos funciones munch_exp y munch_stm.

Diferencias:

- munchStm nos data una lista de instrucciones que implemente un stm. muchExp, nara una lista de instrucciones que genera un valor en registro, y nos devolvera el registro para poder usarlo.

## CALLs

Despues del canonizado, CALL aparece solo en:

    EXP(CALL(f, args))
    o
    MOVE(temp t, CALL(f, args))

Aca tenemos que pensar en la convencion de llamad de la arquitectura elegida.

    codigo

Vamos a tener uan lista constante con los registros que la funcion llamante debe guardar segun al convenciond e llamda de la arquitectura elegida.

div en x68 guarda el resultado en rax y el resto en rdx

## Ejemplos

Los casos mas espeficificos, por ejemplo lea en x86 deberian ir primero en la definicion de la funcion

Despues de esto, nos queda escribir el prologo y el epilogo de cada funcion. Tendriamos que tener:

1. pseudo instrucciones necesaias para indica que comienza una funcion
2. el label
3. instrucciones para modificar el stack pointer
4. instrucciones para guardar los argumentos en resitros si no escapan, en memoria se escapan.
5. guardar los resitros caller-saves a registros nuevos.
6. el cuerpo de la funcion
7. una instruccion para devolver el valr en el registro que corresponde
8. cargar los registros callee-saves
9. restaurar el stack pointer
10. una instruccion para retornar
11. pseudo-instrucciones para indicar el fin de la funcion

Quien hace todo esto? Las funciones procEntryExit en tigerfrmae:

- procEntryExit1 se encarga de mover los argumentos a su lugar, save/restore de calle saves

procEntryExit2 - agrega algunas instrucciones para poder calcular bien la vida (mas adelante vemos que es esto) de los registros


## Como implementamos Strings ?

Llamado a las funciones checkIndexArray y checkNil evitamos que nuestro programa genere segfaults. Nos queda division por cero.

## Livenes analysis

Este es el primer paso para determinar el numero de registros necesarios (y stack). Para esto necesitaremos saver que variables (registros en nuestra maquina infinita) se necesitan al mismo tiempo y cuales no. Si dos variables no se usan al mismo tiempo, podemos en principio ponerlas en el mismo registro fisico.

Vamos a usar el grado de flujo de programa. Tiene un nodo por instruccion y una arista de p a q si la ejecucion del programa a partir de p puede continuar con q. Es un grago dirigido.

En un momento dado, se dice que una variable esta "viva" si su valor actual puede ser utilizado en el futuro.