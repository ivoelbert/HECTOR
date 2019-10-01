# Implementacion de funciones anidadas

No podemos encontrar en general cunatos marcos de activacion atras estará una variable definida en una funcion anidante en tiempo de compilacion.

## Clausuras

Clausuras: una estructura con direccion de la funcion y valores y/o referencias a todas las variables libres en ella

Ventajas: se puede acceder rapido a todo lo necesario

Desventajas: no llegue a copiar

Es mucho para tiger. No vamos a usar clausuras.

## Lambda Lifting

Lambda lifting: crear las funciones anidadas de donde estan y agregar argumentos extra.

Tenemos que pasar como argumentos las variables que se usan en la funcion asi como las que se pueden usar en las funciones que puedan ser llamadas por ella.

Desventaja: es muy ineficiente si no se usan todas las variables escapadas.

## Displays

Displays: implementar una pila con los marcos de activacion de las funciones de diferentes niveles que estan activas.

Como lo implementamos?

- Recurrimos al heap. Malloc al principio del programa.
- Sustraemos una cte al SP para hacer lugar a la pila.

Acceso de variables: en codigo intermedio asumimos que tenemos infinitos registros. Cuando empezamos a generar codigo tenemos por ejemplolas variables escapadas que nos conviene ponerlas en memoria

En tigerframe

    datatype access = InReg of temp
                    | InFrame of int


Nosotros somos libres de implementar Static Links o Display