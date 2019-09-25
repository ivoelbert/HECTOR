# Optimizacion de Programas

Es mejorar alguna caracteristica del programa. Ej: tiempo o espacio.

Nos concentraremos en optimizar tiempo. Una de las maneras de optimizar tiempo es reemplazar las instrucciones "caras" por instrucciones "baratas". Ejemplo: potencias de dos:

    2^n x = x << n
    x / 2^n = x >> n, para x > 0.

Para x < 0, x >> n da el cociente resto a -inf. pero podemos arreglarlo si tomamos

    x / 2^n = x >> n + A con A =/= si x < 0 y x mod 2 =/= 0

