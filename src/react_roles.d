module react_roles;
import std.conv;

extern (C) extern __gshared {
    ulong NOTIF_ROLE;
    ulong RED_ROLE;
    ulong ORANGE_ROLE;
    ulong YELLOW_ROLE;
    ulong GREEN_ROLE;
    ulong BLUE_ROLE;
    ulong PURPLE_ROLE;
}
extern (C) ulong role_by_emoji(char* s) {
    switch (to!string(s)) {
    case "%F0%9F%94%94": /* bell */ return NOTIF_ROLE;
    case "%F0%9F%9F%A5": /* red square */
        return RED_ROLE;
    case "%F0%9F%9F%A7":
        return ORANGE_ROLE;
    case "%F0%9F%9F%A8":
        return YELLOW_ROLE;
    case "%F0%9F%9F%A9":
        return GREEN_ROLE;
    case "%F0%9F%9F%A6":
        return BLUE_ROLE;
    case "%F0%9F%9F%AA":
        return PURPLE_ROLE;
    default:
        return 0;

    }
}
