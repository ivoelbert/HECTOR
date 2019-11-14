# Optimizaciones

Veamos algunas optimizaciones que podemos hacer viendo el flujo de los datos del programa.

- Eliminacion de subexpresiones comunes.
- Eliminacion de codigo muerto.
- Propagacion de constantes.

## Eliminacion de subexpresiones comunes

No nos va a alcanzar con los src y dst de definidos en tigerassem.inst. Vamos a tener que hacer algo de esto:

- Cmabiar instruccion.
- Usar codigo intermedio.
- Usar una representacion entre estados.

## Expresiones disponibles

Una expresion x (+) y esta disponible en el nodo n si:

- Por todo camino desde el comienzo a n, x (+) y se computa al menos una vez.
- No hay definiciones de x o y entre el ultimo computo de n.

Podemos calcular estod e manera similar a liveness.