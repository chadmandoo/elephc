//! Purpose:
//! Per-builtin declarations for string functions migrated to the eval builtin
//! registry.
//!
//! Called from:
//! - `crate::interpreter::builtins` module loading.
//!
//! Key details:
//! - Leaf files register metadata through `eval_builtin!`.

mod addslashes;
mod base64_decode;
mod base64_encode;
mod bin2hex;
mod chop;
mod chr;
mod crc32;
mod ctype_alnum;
mod ctype_alpha;
mod ctype_digit;
mod ctype_space;
mod grapheme_strrev;
mod hash_equals;
mod hex2bin;
mod html_entity_decode;
mod htmlentities;
mod htmlspecialchars;
mod lcfirst;
mod ltrim;
mod nl2br;
mod ord;
mod rawurldecode;
mod rawurlencode;
mod rtrim;
mod str_contains;
mod str_ends_with;
mod str_ireplace;
mod str_pad;
mod str_replace;
mod strlen;
mod str_repeat;
mod str_split;
mod str_starts_with;
mod strrev;
mod stripslashes;
mod strcasecmp;
mod strcmp;
mod strpos;
mod strrpos;
mod strstr;
mod strtolower;
mod strtoupper;
mod substr;
mod substr_replace;
mod trim;
mod ucfirst;
mod ucwords;
mod urldecode;
mod urlencode;
mod wordwrap;
