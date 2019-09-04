# Declaraciones

Principio de minimo asombro: Cuando haya alguna ambiguedad la resolveremos de la manera mas racional y "natural" que podamos.

¿ Que hacemos con este codigo?

    let
      type A = int
      var v := 0
      type B = A
      type A = string

Para tener minimo asombro, tiene que comportarse igual que:

    let
      type A = int
      var v := 0
      type A = string
      type B = A

¿ Ocurre algo horrible en este codigo?

    type A = B
    type B = A

No, solo no se puede tener valores de estos tipos.
    ML lo permite y no da error.

Sort Topologico: dado un conjunto finito de elemtos e_i y un finito de pares (e_i, e_) (un orden parcial), trata de encontrar una secuencia de los e_i, que cumple que si e_k_1, e_k_2, ..., e_k_i, e_k_j, ...
no existe un par (e_k_j, e_k_i). Normalmente hay mas de una secuencia.

    type A = B                          --->      (B, A)
    type A = array of B                 --->      (B, A)
    type A = {a: B_1, b: B_2, c: B_3, ..}   --->      (B_1, A), (B_2, A), (B_3, A) Siempre y cuando B_1 no sea un record que definimos en el batch (para poder tener por ejemplo listas enlazadas)
