module utils;
import std.string, std.regex, std.conv : to;

extern (C) bool is_id(char* str) {
    return to!bool(to!string(str).matchFirst(r"<[@#]&?(?:\d{18}|\d{19})>"));
}
extern (C) size_t trim_id(char* str) {
    return to!size_t(to!string(str).replace(r"<[@#]&?(\d{18}|\d{19})>".regex(), "$1"));
}
