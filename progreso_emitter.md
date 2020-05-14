# Emitter

- parity-wasm sera, ya fue
- No entiendo los accesos a memoria. Como es eso del exponente?
- meto un hashtable para nombre->id y me solucione el mambo de lelvar indices en la traduccion

- Que vida tienen los entornos?
  - Un local puede aparecer en cualquier lado de una funcion, asi que no tendria sentido matar el entorno cuando termino un munch de un stm, porque el proximo acceso no tendria el indice.
  - En algun momento voy a tener que hacer el .with_locals() que es sencillo, son I32 len veces, pero necesito el env cuando termine de traducir todo el body. Lo tengo que ir pasando y devolviendo?

- store con offset = index * WORD_SIZE y allign 32 bits. buscar un ejemplo


## Programa 42

- / Move a MEM
- / BinOp, al menos suma.
- / exp LOCAL
- / JUMP a done: return
- / LABEL done : la podemos descartar
- / LABEL blockfirst: ???
- / agregar memoria
- / mainness
- / SL de _tigermain
