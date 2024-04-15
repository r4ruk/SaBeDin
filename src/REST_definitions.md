# Definitions depending expected/possible Params

## GET Requests

The basic query is always the page: localhost:portnumber? \
and multiple params are by design separated with an &

 - Paging informations
 - - method=getall   (this always have to be given in a requests so the backend knows which function to handle the request with)
   - amountofitems=10 (optional param: Default 20)
   - pagenum=2 (optional param: Default 0 - which is 1 in UI)
   - sortbyasc=name (Sorting the query by name, only considered in method which can handle it)
   - sortbydesc=name (Sorting the query by name, only considered in method which can handle it)

## POST Requests
 // TBD