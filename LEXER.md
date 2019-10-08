# Consideraciones:

## Máquina de estados

El lexer puede estar en 3 estados posibles (cada estado tiene sus "argumentos"):

#### 1- Generando tokens.

Estado default. Estoy consumiendo caracteres tratando de generar tokens que serán 'emitidos' por mi iterador.

Argumento interno: `currentRawToken` (string) representa el "pedazo" de token que vengo construyendo con los caracteres que consumo.

Ejemplo: Puede ser `"funct"`, posiblemente genere el token `function`

#### 2- Consumiendo comentario de bloque.

Estoy consumiendo comentarios del tipo

```rust
/*
    comentario...
*/
```

Argumento interno: `level` (i64) representa el nivel de anidación.

#### 3- Consumiendo comentario de linea.

Estoy consumiendo un comentario del tipo

```rust
// comentario...
```

Sin argumento interno.

#### 4- Terminado

No hay más que consumir. Fin del lexer.

## Estructura de un estado

Un estado debe proveer un método que consume un caracter y devuelve `Option<CharConsumption>`

La estructura `CharConsumption` es

```rust
enum CharConsumpion {
    token: Option<Token>,
    stateTransition: Option<State>,
}
```

Si hay un `token`, se debe emitir por el iterador para ser consumido por el parser

Si hay un `stateTransition` la máquina de estados debe transicionar al estado especificado

## Usando la máquina de estados

La máquina de estados provee un método que consume un caracter y devuelve `Option<Token>`.

Este `Option<Token>` se corresponde con el `Option<Self::Item>` del iterador del lexer de lalrpop.

A nivel implementación, la máquina de estados consume caracteres llamando al consumidor del estado actual y emitiendo tokens cuando corresponda.