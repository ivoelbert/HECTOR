# Extras

## Garbage Collection

Basura: records allocados en el heap que ya no pueden ser alcanzados. DFS.

### Algoritmos

Buscar la basura y reclamarla.

    let
        type rec = {name: string, id: int}
        var a1 : rec = rec{name = "martin", id = 1}          <---- esta referencia se pierde por completo abajo
        var a2 : rec= rec{name = "fede", id = 2}
    in
        ...
        a1 = rec{name = "martin", id = 8786}

#### Mark & Sweap

Recorrer el DFS, marcar todas las direcciones de memoria apuntadas por las variables y luego se reclaman todas las que no estan recordadas.

#### Reference Count

Se lleva cuenta de las referencias que apuntan a cada memoria. Cuando el contador llega a 0, se reclaman.

#### Copying Collection

Recorremos todo el grafo de memoria copiando en otro lugar de la memoria los datos que son alcanzados, permitiendo ademas compactar la memoria.

El lugar de memoria donde estaban allocados los se records se llama `from_space` y el lugar destino `to_space`. Luego de copiar la memoria, `from_space` es basura.

#### Generational Collection

La heuristica es que es mas probable que la memoria allocada mas recientemente sea basura que la memoria que ya sobrevivio otras etapas de GC.

#### Incremental Collection

Usar paralelismo para hacer GC.

## Lenguajes Orientados a Objetos

Al nivel de lenguaje, las nociones que se deben introducir son:

- Clases
- Herencia
- Extencion


## Lenaguajes Funcionales

Lenguajes en los que el mecanismo de computo fundamental esta basado en la aplicacion de funciones.

- FL 1er orden: micro-caml
- FL 2do orden: todos los que conocenmos

### Clausuras

Representa el llamado de una funcion con infrmacion adecional del "entorno". En general, esto es una tupla con la representacion de la funcion con las direcciones de memoria de las variables libres en el cuerpo de la funcion.



Preguntar al final como podriamos implementar manejo dinamico de memoria en Tiger.