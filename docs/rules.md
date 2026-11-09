# Normas del Evaluador

## Definiciones
   `payload:` variables a tener en cuenta en la solicitud de permisos enviadas en formato de objeto Json;
   `accion:` accion definida por el usuario del gestor permisos (ejemplo: realizar facturas) se identifican por id
   `condicion:` objeto  evaluable formado por una condicion Common Expresión Language y variables. 
   `premisa:` colección evaluable de condiciones.
   `restrición:` premisa cuya evaluacion tiene los efectos descritos en Restricciones.
   `permiso:` premisa utilizada en la evaluación de criterios.
   `activa`: estado en el cual  una premisa  ejerce su efecto
   `grupo`: colección de grupos de acciones o de acciones.
   `raiz`: grupo que cumple que para cualquier otro grupo existe una cadena de grupos cuyo elemento superior está en raiz. (forman un arbol)
    
## Normas sobre grupos

  | identificación | norma |
--

  | GRU-001 | las acciones pertecen a un solo grupo
  | GRU-002 | las acciones pertenecen a almenos un grupo
  | GRU-003 | la predencia de una premisa  es el par (n,k) tal que: n es el numero de grupos desde grupo en el cual se ha definido dicha premisa hasta la raiz, y k es el número de premisa que le ha asignado el usuario al definirla diremos que (n,k)< (m,l) cuando n>m y k<l y diremos que son iguales si los dos valores coinciden. 


## Criterios

  | identificación |                 condicion                 | resultado | reporte
  --
  | CRI-001        | `payload` no llega                        |    deny   | Error :No payload supplied
  | CRI-002        | la `accion` reportada no existe o es nula |    deny   | Error :Action not defined 
  | CRI-003        | no aplica ningun ` permiso `              |    deny   | No rules apply y  errores detectados
  | CRI-004        | no aplica ningun ` permiso ` `activo`     |    deny   | Se muestran todas las ` restricciones ` y los `permisos ` que deshabilitan y errores detectados
  | CRI-005        | aplica  algún `permiso` `activo`          |    allow  | ` restricciones `  aplicadas  , `permiso` que aplica y errores detectados.

##  Restricciones

| identificación |                   norma                 |
--
| RES-001         | las restricciones `activas`  `inactivan`  cualquier ` premisa ` que tenga una precedencia menor a la misma cuando aplica.
| RES-002         | una restriccion **nunca** ` inactiva ` ninguna  `premisa` de precedencia igual o superior a la misma.

## Evaluacción de premisas

| identificacion |                   condicion                 | resultado |
--
| EVP-001| todas las `condiciones` de una `premisa` compilan y aplican | aplica
| EVP-999| cualquier caso no mencionado previamente                    | no aplica

## Evaluacion de condiciones

| identificación |                  condicion                   | resultado |
--
| EVC-001 | falta alguna variable en el `payload` o no es valida | no aplica
| EVC-002 | la norma C.E.L. que define la condición no compila | no compila
| EVC-003 | al sutituir la variables del `payload` la condición C.E.L compila y devuelve true | aplica |
| EVC-999 | cualquier caso no mencionado previamente           | no aplica





  
