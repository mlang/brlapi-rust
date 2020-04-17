bindgen /usr/include/brlapi.h -o src/bindings.rs --whitelist-function '^brlapi_.*' --whitelist-var '^BRL.*' -- -DBRLAPI_NO_SINGLE_SESSION
