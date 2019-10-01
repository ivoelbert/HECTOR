# Seleccion de instrucciones

Entramos al backend del compilador.

En general un compilador se divide en frontend y backend. El frontend pasa el codigo fuente a lenguaje intermedio y el backend del lenguaje intermedio al lenguaje final.

Esta arquitectura te permite mantener mas facilmente una familia de compiladores.

Hay que encontrar una forma de cubrir cada arbolito (recuerden que canonizando nos queda una lista de arbolitos por cada funcion) con los patrones que nos da l assembler de la maquina de destino.

## Ejemplo: arquitectura de juguete

ADD r_i <- r_j + r_k
MUL r_i <- r_j * r_k
SUB r_i <- r_j - r_k
DIV r_i <- r_j / r_k

Ser mas prolijo al generar el codigo intermedio (por ejemplo si hay constantes ponerla siempre al final) te puede ahorrar casos en la generacion de codigo final.

Los cubrimientos que encontremos no seran unicos. Podemos asociarle un costo a cada instruccion.

    (aca en el medio hay unaparte que no copie porque estaba mirando los escapes)

Existen cubrimientos optimos (de acuerdo a la funcion de costo que hayamos elegido) y cubrimientos optimales (no existe un cubrimiento similar, es decir, cambiando solo un par de tiles, que tenga un costo menor).

Elegiremos algun algoritmo que nos provea un cubrimiento optimal, para no hacer que lleve demasiado tiempo la compilacion.

## Maximal Munch

- Genera las instrucciones "al reves"
- Si existe un patron para cada tipo de nodo de tree que sea simple, no va a quedar atascado.

 VAMOS A USAR MUNCH

## Programacion dinamica

Podemos usarla para obtener el tiling optimo. ¿ Cual es el costo de utilizar un tile deteerminado en un nodo?

El costo optimo de los subarboles que quedan despues de colocar el tile t mas el costo asociado a a t.

¿Cual es el costo optimo en un nodo?

El menor costo entre los obtenidos al poner todos los tiles que matcheaban.

Vamos asignandole costos a los nodos desde las hojas hasta la raiz.


## Gramaticas

Se pueden definir gramaticas que quedan re ambiguas y es un quilombo, en resumen.