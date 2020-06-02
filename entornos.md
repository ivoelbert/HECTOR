# Ambientes

## End-user

Alguien quiere escribir su libreria en tiger para importarla en su javascript.

El resultado de hector tiene que ser un paquete importable, con el .wasm y un .js con las interfaces.
Tenemos que poder correr `./hector algo.tig -o paquete` y que eso deje en paquete algo que este listo para importarse desde JavaScript.

## Como libreria para la app

- La aplicacion de react, el compilador, el runtime y el interop.
- Escribis tiger en el editor de la app
- hector solo arma el .wasm
- la aplicacion instancia ese .wasm como un modulo javascript
  - Tiene 1 solo export, tigermain_wrapper: () -> numero
  - Tiene imports con cada funcion del runtime
- la aplicacion llama a la funcion del interop

## Ambientes de Test

### garco test

Testea solo las estructuras intermedias. No hace tests semanticos sobre el resultado.
Fijarse que todos los good compilen sin explotar.

### jest

Tiene que hacer los tests semanticos sobre el resultado en wasm.
Levanta hector compilado a webassembly, compila los sources, los carga como modulos wasm y los testea.
