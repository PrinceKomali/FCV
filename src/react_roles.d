module react_roles;
import std.conv;
size_t role_by_emoji(char * s) {
    switch (to!string(s)) {
        case "\U0001f514": return 1136439826829287424;
        default: return 0;

    }
}