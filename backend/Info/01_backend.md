
# Responsiblity of Backend Apis: 

* A Backend Api is 6 handed Daemon 
* He does 1 thing with each of his hand
> Hands: 
    - 1.Starting Server
    - 2.Accept request 
    - 3.Route req => function
    - 4.Run business logics
    - 5.Talk to db
    - 6.Send Respnse

its like a dmart shop 
> you start shop 
> costumer comes with request (accept request) 
> send him to proper shelve where his required thing esists (proper function) 
> when product(funtion) found scan the tag and calculate the business logics 
> once the calculations done you write into log book for intry (talk to db)
> and you complete transation with giving the product + recept (response)


so there are multyple wokkers and ther assistants. 
> so main is the owner who starts the shop  
> app build the router (whome do i connect this one with? =>one who send users to shelves?)
> handlers - Request revievers => counter of waiting list. they took entry of who came and handels the request ? something missing here.
> services -> no  doubt they are accountents.  (function)
> db -> afcourse shelves   where the data is stored.
> modles -> may be map of whle db / more like a cheatshit.
> config > env setting ... where do i connect it with? i am not geting any idea.
> errors > they are like customer exicutives. who deals with problems persnolly.
> app_state > again dont know where i connect it.

