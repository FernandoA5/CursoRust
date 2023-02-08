Patterns that Bind to Values

En el episodio anterior vimos la característica Match, la cual nos permite controlar
el flujo de ejecución de nuestro programa dependiendo de el valor que adquiriera una variable, o
un enumerador como fue realmente el caso del episodio.

Utilizando el mismo código, nosotros podemos modificar nuestro enumerador, para que
las variantes puedan alojar un enumerador. Por ejemplo, podemos hacer que cada billete
tenga dentro de si, un enumerador que indica el año de emisión. 

Para hacer esto creamos un nuevo enumerador, el cual llamaremos Year, así
evitamos problemas con el compilador al no utilizar la Ñ. Y cómo ejemplo, pondremos algunos años
como 1964, 1988, y 2012. 
*Hacer código*

Tambien debemos modificar nuestro enumerador de billetes, para que puedan alojar esta variante de
fecha de emisión. Aunque para efectos prácticos solo lo haremos con el buen Nezahualcoyotl.
*hacer código*

Tambien, ahora que sabemos que Neza puede tener tres variantes, podemos modificar nuestra estructura
Match, podemos crear un Match anidado, al cual le podríamos pedir que haga diferentes cosas
dependiendo de el año de emisión. Pero no quiero hacerlo, entonces será tarea.

Lo que si haremos, es mostrar en pantalla, el tipo de Neza que es. O sea, su año de emisión. Y para
eso debemos decirle al compilador que nos deje imprimir la información interna de las variables.
Esto lo vimos en un episodio anterior, el cual nos mostraba los derive traits. Pero, para que eso
funcione, solo debemos añadir una linea de código al comienzo de nuestro programa: #[derive(debug)]
*Agregar linea*
*Modificar Match*

Y esto nos permitirá mostrar en pantalla el año de emisión de el billete de Nezahualcoyotl. Y por
supuesto, debemos retornar el valor de el billete.

Para ver esto funcionar, debemos crear un nuevo billete.
*crear nuevo billete*

Por supuesto, cómo les dije: Utilizando un Match anidado podrían personalizar el código para que
dependiendo de el año de emisión el código muestre distintos mensajes o haga distintas cosas.

Y eso es todo por este episodio, espero les haya resultado interesante. Y nos vemos en el siguiente
video.


