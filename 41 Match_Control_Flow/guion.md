En episodios anteriores vimos bastante acerca de los enumeradores, pero la forma de utilizarlos sigue siendo un poco ambigua. En este episodio hablaremos de una caracteristica de rust llamada match, con la cual podremos hacer que nuestro código se comporte de una forma o de otra, en función de el estado que tenga un enum.

*Hacer Enumerador*
Si estás confundido no te preocupes, si deje la explicación confusa es porque en esta ocación si veremos código.
Vamos a crear un enum llamado billete, el cual tendrá dentro de sí las variables Benito, Morelos, Nezahualcoyotl, o Neza para los amigos, y Sor Juana.

*Hacer función main*
Ahora crearemos la función main que se encargará de ejecutar el programa, dentro de la función main, llamaremos una función a la que se llamará valorBillete, quien recibirá una variable a la que llamaremos billete, y será de tipo Billete.


*Hacer función valorBillete*
Vamos a crear la función valorBillete, la cual como dijimos recibirá una variable con el nombre de billete, y de tipo Billete. Esta función retornará un valor de tipo u16. Dentro de esta función, se encontrará esta característica match. La cual recibe el enum Billete, y si Billete es Benito, devuelve el valor 20. Si el billete es Morelos, devuelve el valor 50. Si el billete es Nezahualcóyotl devuelve 100, y si es Sor-Juana, devuelve 200.

Esta función sabemos que retorna un valor, por lo que tendríamos que llamarla dentro de un println! para ver el valor que devuelve. 

Tambien podemos poner llaves en el match, justo donde antes pusimos la asignación de valor, y dentro de estas llaves, podemos poner el bloque de código que queremos ejecutar.

Y así de sencillo podemos comenzar a utilizar esta caracterísitca de Rust, los más veteranos en esto de la programación habrán notado que esto es como un switch en otros lenguajes, una estructura que si bien, no es la favorita de los desarrolladores, es bastante útil, y aquí es más intuitiva de utilizar.

*Escena post-creditos*
P: Oiga tengo una pregunta.
F: *Asustado* ¿Quién es?¿Cómo entró aquí?
P: Pos la mera verdad ni sé, para que le hecho mentiras.
F: *confundido* Okay, ¿puedo ayudarlo de alguna forma?
P: No ni lo mande Dios, La última vez me disque me ayudó un tal Omar chaparro y mire nomás como me fue.
F: *confundido* ¿Omar Chaparro? *se da cuenta* Es usted...
P: Eso no es importante, mejor digame para que sirve todo eso que estaba explicando y quien es aquel que está ahí nomas parado.
F: El es el productor, y lo que expliqué tiene muchas funciones, con Rust se pueden hacer cosas como servidores, blockchain, Internet of Things y cosas así que requieren velocidad.
P: No pos no le entendí nada. ¿La gente si ve su programa?
F: Pues algunas personas, Entre 10 y 70 personas diariamente. Es un canal pequeño.
P: ¿Cómo? ¿Tiene su propio canal de televisión?
F: No, o sea es YouTube.
P: ¿Y eso con qué se come oiga?
F: Em.. Pues no se como explicarselo.
P: Sí ya lo estuve viendo, esto de explicar cosas como que no se le da muy bien que digamos.
F: Pues tampoco es como que me paguen por esto.
P: Ahí ni cómo discutirle. Mejor voy a ver como están las cosas por estos rumbos. Ahí nos vemos. *sale de cuadro*
F: *Confundido* O-okay, Adiós.
P: Estaría bueno que me prepare un auto rápido con ese tal Rust. *voz*
F: ¿Un auto rápido?
J: Usted dijo que Rust servía para hacer cosas rápidas.
F: *confundido*







