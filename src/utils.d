module utils;
import std.string, std.regex, std.conv : to;

extern (C) char* test(char* a) {
    return cast(char*) "d string".toStringz();
}

extern (C) bool is_id(char* str) {
    return to!bool(to!string(str).matchFirst(r"<[@#]&?(?:\d{18}|\d{19})>"));
}
