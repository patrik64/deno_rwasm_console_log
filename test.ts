import init , { source, log_string, log_two_strings, log_number, log_with_macro } from "./wasm.js";

await init(source);

log_string();
log_two_strings();
log_number();
log_with_macro();