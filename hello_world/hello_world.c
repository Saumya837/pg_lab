#include "postgres.h"
#include "fmgr.h"
#include "utils/builtins.h"

PG_MODULE_MAGIC;

/*
    hello_world() -> text
    No arguments, returns a greeting
*/

PG_FUNCTION_INFO_V1(hello_world);

Datum
hello_world(PG_FUNCTION_ARGS){
    text *result = cstring_to_text("Hello World!");
    PG_RETURN_TEXT_P(result);
}

PG_FUNCTION_INFO_V1(hello_name);
Datum
hello_name(PG_FUNCTION_ARGS){
    text *name_arg = PG_GETARG_TEXT_PP(0);
    char *name_str = text_to_cstring(name_arg);
    char *greeting;
    text *result;

    greeting = psprintf("Hello %s!", name_str);
    result = cstring_to_text(greeting);
   
    PG_RETURN_TEXT_P(result);
}



